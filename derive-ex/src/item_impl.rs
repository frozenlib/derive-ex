use crate::{common::BinaryOp, syn_utils::expand_self};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::fmt::Display;
use structmeta::StructMeta;
use syn::{
    parse2, parse_quote, spanned::Spanned, Error, GenericArgument, Ident, ImplItem, ItemImpl, Path,
    PathArguments, PathSegment, Result, Type,
};

#[derive(StructMeta, Debug)]
#[struct_meta(name_filter = "snake_case")]
struct ArgList {
    #[struct_meta(unnamed)]
    items: Vec<Ident>,
    dump: bool,
}

struct Args {
    dump: bool,
    make_binary: bool,
    make_assign: bool,
}
impl Args {
    fn from_attr_args(attr: TokenStream, op: Op) -> Result<Args> {
        let args: ArgList = parse2(attr)?;
        let mut make_binary = false;
        let mut make_assign = false;
        for item in &args.items {
            let target_op = Op::from_ident(item)?;
            if target_op.op != op.op {
                bail!(
                    item.span(),
                    "expected `{}` or `{}`",
                    Op::new(op.op, OpForm::Binary),
                    Op::new(op.op, OpForm::Assign)
                );
            }
            match target_op.form {
                OpForm::Binary => make_binary = true,
                OpForm::Assign => make_assign = true,
            }
        }
        Ok(Self {
            dump: args.dump,
            make_binary,
            make_assign,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Op {
    op: BinaryOp,
    form: OpForm,
}

impl Op {
    fn new(op: BinaryOp, form: OpForm) -> Self {
        Self { op, form }
    }
    fn from_str(mut s: &str) -> Option<Self> {
        let suffix = "Assign";
        let mut form = OpForm::Binary;
        if s.ends_with(suffix) {
            s = &s[..s.len() - suffix.len()];
            form = OpForm::Assign;
        }
        Some(Self::new(BinaryOp::from_str(s)?, form))
    }
    fn from_ident(s: &Ident) -> Result<Self> {
        let s = s.to_string();
        if let Some(op) = Self::from_str(&s) {
            Ok(op)
        } else {
            bail!(s.span(), "`{}` is not supported for `#[derive_ex]`", s);
        }
    }
    fn to_func_ident(self) -> Ident {
        let name = self.op.to_func_name();
        let assign = if self.form == OpForm::Assign {
            "_assign"
        } else {
            ""
        };
        Ident::new(&format!("{name}{assign}"), Span::call_site())
    }
    fn to_trait_ident(self) -> Ident {
        Ident::new(&self.to_string(), Span::call_site())
    }
    fn to_trait_path(self) -> Path {
        let ident = self.to_trait_ident();
        parse_quote!(::core::ops::#ident)
    }
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.op)?;
        if self.form == OpForm::Assign {
            write!(f, "Assign")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OpForm {
    Binary,
    Assign,
}

pub fn build_by_item_impl(attr: TokenStream, item_impl: &ItemImpl) -> Result<TokenStream> {
    let span = Span::call_site();
    let message = "must be used with `impl {Trait} for {Type}`";
    let t = item_impl
        .trait_
        .as_ref()
        .ok_or_else(|| Error::new(span, message))?;
    if t.0.is_some() {
        bail!(span, "cannot use with negative trait");
    }
    let s =
        t.1.segments
            .last()
            .ok_or_else(|| Error::new(span, message))?;

    let this_orig = &item_impl.self_ty;
    let (this, this_is_ref) = to_ref_elem(this_orig);
    let rhs_orig = to_rhs(s, this_orig);
    let (rhs, rhs_is_ref) = to_ref_elem(&rhs_orig);
    let g = expand_self(&item_impl.generics, this_orig);
    let (impl_g, _, where_g) = &g.split_for_impl();

    let op = Op::from_ident(&s.ident)?;
    let args = Args::from_attr_args(attr, op)?;

    let binary_op = Op::new(op.op, OpForm::Binary);
    let binary_func = binary_op.to_func_ident();
    let binary_trait = binary_op.to_trait_path();

    let assign_op = Op::new(op.op, OpForm::Assign);
    let assign_func = assign_op.to_func_ident();
    let assign_trait = assign_op.to_trait_path();

    let mut ts = TokenStream::new();
    match op.form {
        OpForm::Binary => {
            let output = expand_self(find_output_type(item_impl)?, this_orig);
            let impl_binary =
                |impl_l_ref: bool, impl_r_ref: bool, call_l_ref: bool, call_r_ref: bool| {
                    if impl_l_ref == call_l_ref && impl_r_ref == call_r_ref {
                        return quote!();
                    }
                    let impl_this = ref_type_with(&this, impl_l_ref);
                    let impl_rhs = ref_type_with(&rhs, impl_r_ref);
                    let l = ref_type_with(&this, call_l_ref);
                    let r = ref_type_with(&rhs, call_r_ref);
                    let l_expr = change_owned(quote!(self), &this, impl_l_ref, call_l_ref);
                    let r_expr = change_owned(quote!(rhs), &rhs, impl_r_ref, call_r_ref);
                    quote! {
                        #[automatically_derived]
                        impl #impl_g #binary_trait<#impl_rhs> for #impl_this #where_g {
                            type Output = #output;
                            fn #binary_func(self, rhs: #impl_rhs) -> Self::Output {
                                <#l as #binary_trait<#r>>::#binary_func(#l_expr, #r_expr)
                            }
                        }
                    }
                };

            let impl_assign = |rhs: &Type, call_l_ref: bool| {
                let l = ref_type_with(&this, call_l_ref);
                let l_expr = change_owned(quote!(self), &this, true, call_l_ref);
                quote! {
                    #[automatically_derived]
                    impl #impl_g #assign_trait<#rhs> for #this #where_g {
                        fn #assign_func(&mut self, rhs: #rhs) {
                            *self = <#l as #binary_trait<#rhs>>::#binary_func(#l_expr, rhs)
                        }
                    }
                }
            };

            if args.make_binary {
                for this in [false, true] {
                    for rhs in [false, true] {
                        ts.extend(impl_binary(this, rhs, this_is_ref, rhs_is_ref));
                    }
                }
            }
            if args.make_assign {
                if args.make_binary {
                    ts.extend(impl_assign(&rhs, true));
                    ts.extend(impl_assign(&ref_type(&rhs), true));
                } else {
                    ts.extend(impl_assign(&rhs_orig, this_is_ref));
                }
            }
        }
        OpForm::Assign => {
            if args.make_assign {
                bail!(
                    _,
                    "`#[derive_ex({})]` can be used only with `impl {} for T`",
                    Op::new(op.op, OpForm::Assign),
                    Op::new(op.op, OpForm::Binary),
                );
            }
            if args.make_binary {
                let this = this_orig;
                let rhs = &rhs_orig;
                ts.extend(quote! {
                    #[automatically_derived]
                    impl #impl_g #binary_trait<#rhs> for #this #where_g {
                        type Output = #this;
                        fn #binary_func(mut self, rhs: #rhs) -> Self::Output {
                            <#this as #assign_trait<#rhs>>::#assign_func(&mut self, rhs);
                            self
                        }
                    }
                });
            }
        }
    }

    if args.dump {
        bail!(_, "{}", format!("dump:\n{ts}"));
    }
    Ok(ts)
}
fn find_output_type(item_impl: &ItemImpl) -> Result<&Type> {
    for item in &item_impl.items {
        if let ImplItem::Type(t) = item {
            if t.ident == "Output" {
                return Ok(&t.ty);
            }
        }
    }
    bail!(_, "cannot find associate type `Output`");
}
fn to_ref_elem(ty: &Type) -> (Type, bool) {
    if let Type::Reference(tr) = ty {
        if tr.lifetime.is_none() && tr.mutability.is_none() {
            return (tr.elem.as_ref().clone(), true);
        }
    }
    (ty.clone(), false)
}
fn to_rhs(s: &PathSegment, self_ty: &Type) -> Type {
    if let PathArguments::AngleBracketed(args) = &s.arguments {
        if args.args.len() == 1 {
            if let GenericArgument::Type(ty) = &args.args[0] {
                return expand_self(ty, self_ty);
            }
        }
    }
    self_ty.clone()
}
fn ref_type(ty: &Type) -> Type {
    parse_quote!(&#ty)
}
fn ref_type_with(ty: &Type, is_ref: bool) -> Type {
    if is_ref {
        ref_type(ty)
    } else {
        ty.clone()
    }
}
fn change_owned(expr: TokenStream, ty: &Type, input_ref: bool, output_ref: bool) -> TokenStream {
    match (input_ref, output_ref) {
        (true, false) => quote!(<#ty as ::core::clone::Clone>::clone(#expr)),
        (false, true) => quote!(&#expr),
        _ => expr,
    }
}
