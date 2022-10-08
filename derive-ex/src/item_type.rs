use crate::{
    bound::{Bound, Bounds, WhereClauseBuilder},
    common::BinaryOp,
    syn_utils::expand_self,
};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use structmeta::{NameArgs, Parse, StructMeta};
use syn::{
    parse::Parse, parse2, parse_quote, spanned::Spanned, token, Attribute, Error, Expr, Field,
    Fields, Ident, Index, ItemEnum, ItemStruct, Path, Result, Type, Variant,
};

#[derive(StructMeta, Debug)]
#[struct_meta(name_filter = "snake_case")]
struct Args {
    #[struct_meta(unnamed)]
    items: Vec<DeriveItem>,
    bound: Option<NameArgs<Vec<Bound>>>,
    dump: bool,
}

#[derive(Parse, Debug)]
struct DeriveItem {
    trait_ident: Ident,
    args: DeriveItemArgsOption,
}

#[derive(StructMeta, Debug)]
#[struct_meta(name_filter = "snake_case")]
struct DeriveItemArgs {
    bound: Option<NameArgs<Vec<Bound>>>,
    dump: bool,
}

#[derive(Parse, Debug)]
enum DeriveItemArgsOption {
    Some {
        #[parse(peek)]
        #[to_tokens("(")]
        _paren: token::Paren,
        args: DeriveItemArgs,
    },
    None,
}

#[derive(StructMeta, Debug)]
struct ArgsForDefault {
    #[struct_meta(unnamed)]
    value: Expr,
    bound: Option<NameArgs<Vec<Bound>>>,
}
impl Default for ArgsForDefault {
    fn default() -> Self {
        Self {
            value: parse_quote!(_),
            bound: None,
        }
    }
}

pub fn build_by_item_struct(attr: TokenStream, item: &mut ItemStruct) -> Result<TokenStream> {
    let mut kinds = HelperAttributeKinds::new(true);
    let result = build_by_item_struct_core(attr, item, &mut kinds);
    remove_attrs(&mut item.attrs, &kinds);
    for field in &mut item.fields {
        remove_attrs(&mut field.attrs, &kinds)
    }
    result
}
fn build_by_item_struct_core(
    attr: TokenStream,
    item: &ItemStruct,
    kinds: &mut HelperAttributeKinds,
) -> Result<TokenStream> {
    let es = DeriveEntry::from_root(attr, &item.attrs)?;
    kinds.extend(&es);
    let hattrs = HelperAttributes::from_attrs(&item.attrs, &kinds.without_derive_ex())?;
    let fields = FieldEntry::from_fields(&item.fields, kinds)?;
    let mut ts_all = TokenStream::new();
    for e in es {
        let result = match e.kind {
            DeriveItemKind::BinaryOp(op) => build_binary_op(item, op, &e, &fields),
            DeriveItemKind::AssignOp(op) => build_assign_op(item, op, &e, &fields),
            DeriveItemKind::UnaryOp(op) => build_unary_op(item, op, &e, &fields),
            DeriveItemKind::Clone => build_clone_for_struct(item, &e, &fields),
            DeriveItemKind::Default => build_default_for_struct(item, &e, &hattrs, &fields),
            DeriveItemKind::Deref => build_deref_for_struct(item, &e, &fields),
            DeriveItemKind::DerefMut => build_deref_mut_for_struct(item, &e, &fields),
        };
        ts_all.extend(e.apply_dump(result));
    }
    Ok(ts_all)
}
pub fn build_by_item_enum(attr: TokenStream, item: &mut ItemEnum) -> Result<TokenStream> {
    let mut kinds = HelperAttributeKinds::new(true);
    let result = build_by_item_enum_core(attr, item, &mut kinds);
    remove_attrs(&mut item.attrs, &kinds);
    for variant in &mut item.variants {
        remove_attrs(&mut variant.attrs, &kinds);
        for field in &mut variant.fields {
            remove_attrs(&mut field.attrs, &kinds)
        }
    }
    result
}
fn build_by_item_enum_core(
    attr: TokenStream,
    item: &ItemEnum,
    kinds: &mut HelperAttributeKinds,
) -> Result<TokenStream> {
    let es = DeriveEntry::from_root(attr, &item.attrs)?;
    kinds.extend(&es);
    let hattrs = HelperAttributes::from_attrs(&item.attrs, &kinds.without_derive_ex())?;
    let variants = VariantEntry::from_variants(&item.variants, kinds)?;
    let mut ts_all = TokenStream::new();
    for e in es {
        let result = match e.kind {
            DeriveItemKind::Clone => build_clone_for_enum(item, &e, &variants),
            DeriveItemKind::Default => build_default_for_enum(item, &e, &hattrs, &variants),
            _ => bail!(e.span, "derive `{}` for enum is not supported", e.kind),
        };
        ts_all.extend(e.apply_dump(result));
    }
    Ok(ts_all)
}

fn build_binary_op(
    item: &ItemStruct,
    op: BinaryOp,
    e: &DeriveEntry,
    fields: &[FieldEntry],
) -> Result<TokenStream> {
    let kind = DeriveItemKind::BinaryOp(op);
    let (_, type_g, _) = item.generics.split_for_impl();
    let this_ty_ident = &item.ident;
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let generics = expand_self(&item.generics, &this_ty);
    let (impl_g, _, _) = generics.split_for_impl();
    let trait_ = kind.to_path();
    let func_name = format_ident!("{}", op.to_func_name());

    let build = |lhs_is_ref: bool, rhs_is_ref: bool| {
        let self_ty = with_ref(&this_ty, lhs_is_ref);
        let rhs_ty = with_ref(&this_ty, rhs_is_ref);
        let mut wcb = WhereClauseBuilder::new(&generics);
        let use_bounds = e.push_bounds_to(&mut wcb);
        let mut values = Vec::new();
        for field in fields {
            let field_ty = &field.field.ty;
            let lhs = with_ref(&member(quote!(self), field), lhs_is_ref);
            let rhs = with_ref(&member(quote!(rhs), field), rhs_is_ref);
            let lhs_ty = with_ref(field_ty, lhs_is_ref);
            let rhs_ty = with_ref(field_ty, rhs_is_ref);
            values.push(quote!(<#lhs_ty as #trait_<#rhs_ty>>::#func_name(#lhs, #rhs)));
            field.push_bounds_to(use_bounds, kind, &mut wcb);
        }
        let ctor_args = build_ctor_args(&item.fields, &values);
        let wheres = wcb.build(|ty| match (lhs_is_ref, rhs_is_ref) {
            (true, true) => quote!(for<'a> &'a #ty : #trait_<&'a #ty, Output = #ty>),
            (true, false) => quote!(for<'a> &'a #ty : #trait_<#ty, Output = #ty>),
            (false, true) => quote!(for<'a> #ty : #trait_<&'a #ty, Output = #ty>),
            (false, false) => quote!(#ty : #trait_<#ty, Output = #ty>),
        });
        quote! {
            impl #impl_g #trait_<#rhs_ty> for #self_ty #wheres {
                type Output = #this_ty;
                fn #func_name(self, rhs: #rhs_ty) -> Self::Output {
                    #this_ty_ident #ctor_args
                }
            }
        }
    };
    let mut ts = TokenStream::new();
    for lhs_is_ref in [false, true] {
        for rhs_is_ref in [false, true] {
            ts.extend(build(lhs_is_ref, rhs_is_ref));
        }
    }
    Ok(ts)
}
fn build_assign_op(
    item: &ItemStruct,
    op: BinaryOp,
    e: &DeriveEntry,
    fields: &[FieldEntry],
) -> Result<TokenStream> {
    let kind = DeriveItemKind::AssignOp(op);
    let (_, type_g, _) = item.generics.split_for_impl();
    let this_ty_ident = &item.ident;
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let generics = expand_self(&item.generics, &this_ty);
    let (impl_g, _, _) = generics.split_for_impl();
    let trait_ = kind.to_path();
    let func_name = format_ident!("{}_assign", op.to_func_name());

    let build = |rhs_is_ref: bool| {
        let rhs_ty = with_ref(&this_ty, rhs_is_ref);
        let mut wcb = WhereClauseBuilder::new(&generics);
        let use_bounds = e.push_bounds_to(&mut wcb);
        let mut exprs = Vec::new();
        for field in fields {
            let field_ty = &field.field.ty;
            let lhs = member(quote!(self), field);
            let rhs = with_ref(&member(quote!(rhs), field), rhs_is_ref);
            let rhs_ty = with_ref(field_ty, rhs_is_ref);
            exprs.push(quote!(<#field_ty as #trait_<#rhs_ty>>::#func_name(&mut #lhs, #rhs)));
            field.push_bounds_to(use_bounds, kind, &mut wcb);
        }
        let wheres = wcb.build(|ty| match rhs_is_ref {
            true => parse_quote!(for<'a> #ty : #trait_<&'a #ty>),
            false => parse_quote!(#ty : #trait_<#ty>),
        });
        quote! {
            impl #impl_g #trait_<#rhs_ty> for #this_ty #wheres {
                fn #func_name(&mut self, rhs: #rhs_ty) {
                    #(#exprs;)*
                }
            }
        }
    };
    let mut ts = TokenStream::new();
    for rhs_is_ref in [false, true] {
        ts.extend(build(rhs_is_ref));
    }
    Ok(ts)
}
fn build_unary_op(
    item: &ItemStruct,
    op: UnaryOp,
    e: &DeriveEntry,
    fields: &[FieldEntry],
) -> Result<TokenStream> {
    let kind = DeriveItemKind::UnaryOp(op);
    let (_, type_g, _) = item.generics.split_for_impl();
    let this_ty_ident = &item.ident;
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let generics = expand_self(&item.generics, &this_ty);
    let (impl_g, _, _) = generics.split_for_impl();
    let trait_ = kind.to_path();
    let func_name = format_ident!("{}", op.to_func_name());

    let build = |lhs_is_ref: bool| {
        let self_ty = with_ref(&this_ty, lhs_is_ref);
        let mut wcb = WhereClauseBuilder::new(&generics);
        let use_bounds = e.push_bounds_to(&mut wcb);
        let mut values = Vec::new();
        for field in fields {
            let field_ty = &field.field.ty;
            let lhs = with_ref(&member(quote!(self), field), lhs_is_ref);
            let lhs_ty = with_ref(field_ty, lhs_is_ref);
            values.push(quote!(<#lhs_ty as #trait_>::#func_name(#lhs)));
            field.push_bounds_to(use_bounds, kind, &mut wcb);
        }
        let ctor_args = build_ctor_args(&item.fields, &values);
        let wheres = wcb.build(|ty| match lhs_is_ref {
            true => quote!(for<'a> &'a #ty : #trait_<Output = #ty>),
            false => quote!(#ty : #trait_<Output = #ty>),
        });
        quote! {
            impl #impl_g #trait_ for #self_ty #wheres {
                type Output = #this_ty;
                fn #func_name(self) -> Self::Output {
                    #this_ty_ident #ctor_args
                }
            }
        }
    };
    let mut ts = TokenStream::new();
    for lhs_is_ref in [false, true] {
        ts.extend(build(lhs_is_ref));
    }
    Ok(ts)
}
fn build_clone_for_struct(
    item: &ItemStruct,
    e: &DeriveEntry,
    fields: &[FieldEntry],
) -> Result<TokenStream> {
    let kind = DeriveItemKind::Clone;
    let (_, type_g, _) = item.generics.split_for_impl();
    let this_ty_ident = &item.ident;
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let generics = expand_self(&item.generics, &this_ty);
    let (impl_g, _, _) = generics.split_for_impl();
    let trait_ = kind.to_path();

    let mut wcb = WhereClauseBuilder::new(&generics);
    let use_bounds = e.push_bounds_to(&mut wcb);
    let mut ctor_args = Vec::new();
    let mut clone_from_exprs = Vec::new();
    for field in fields {
        let field_ty = &field.field.ty;
        let lhs = &member(quote!(self), field);
        let rhs = &member(quote!(source), field);
        ctor_args.push(quote!(<#field_ty as #trait_>::clone(&#lhs)));
        clone_from_exprs.push(quote!(<#field_ty as #trait_>::clone_from(&mut #lhs, &#rhs)));
        field.push_bounds_to(use_bounds, kind, &mut wcb);
    }
    let ctor_args = build_ctor_args(&item.fields, &ctor_args);
    let wheres = wcb.build(|ty| quote!(#ty : #trait_));
    Ok(quote! {
        impl #impl_g #trait_ for #this_ty #wheres {
            fn clone(&self) -> Self {
                #this_ty_ident #ctor_args
            }
            fn clone_from(&mut self, source: &Self) {
                #(#clone_from_exprs;)*
            }
        }
    })
}
fn build_clone_for_enum(
    item: &ItemEnum,
    e: &DeriveEntry,
    variants: &[VariantEntry],
) -> Result<TokenStream> {
    let kind = DeriveItemKind::Clone;
    let (_, type_g, _) = item.generics.split_for_impl();
    let this_ty_ident = &item.ident;
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let generics = expand_self(&item.generics, &this_ty);
    let (impl_g, _, _) = generics.split_for_impl();
    let trait_ = kind.to_path();

    let mut wcb = WhereClauseBuilder::new(&generics);
    let use_bounds = e.push_bounds_to(&mut wcb);
    let mut arms_clone = Vec::new();
    let mut arms_clone_from = Vec::new();
    for variant in variants {
        let variant_ident = &variant.variant.ident;
        let mut pat_args_l = Vec::new();
        let mut pat_args_r = Vec::new();
        let mut ctor_args = Vec::new();
        let mut clone_from_exprs = Vec::new();
        let use_bounds = variant.hattrs.push_bounds_to(use_bounds, kind, &mut wcb);
        for field in &variant.fields {
            let field_ty = &field.field.ty;
            let lhs = field.make_ident("l");
            let rhs = field.make_ident("r");
            pat_args_l.push(quote!(#lhs));
            pat_args_r.push(quote!(#rhs));
            ctor_args.push(quote!(<#field_ty as #trait_>::clone(#lhs)));
            clone_from_exprs.push(quote!(<#field_ty as #trait_>::clone_from(#lhs, #rhs)));
            field.push_bounds_to(use_bounds, kind, &mut wcb);
        }

        let pat_l = build_ctor_args(&variant.variant.fields, &pat_args_l);
        let pat_r = build_ctor_args(&variant.variant.fields, &pat_args_r);
        let ctor_args = build_ctor_args(&variant.variant.fields, &ctor_args);

        arms_clone.push(quote!(Self::#variant_ident #pat_l => Self::#variant_ident #ctor_args));
        arms_clone_from.push(quote! {
            (Self::#variant_ident #pat_l , Self::#variant_ident #pat_r) => {
                #(#clone_from_exprs;)*
            }
        });
    }
    let wheres = wcb.build(|ty| quote!(#ty : #trait_));
    Ok(quote! {
        impl #impl_g #trait_ for #this_ty #wheres {
            fn clone(&self) -> Self {
                match self {
                    #(#arms_clone,)*
                }
            }
            fn clone_from(&mut self, source: &Self) {
                match (self, source) {
                    #(#arms_clone_from,)*
                    (lhs, rhs) => *lhs = <Self as ::core::clone::Clone>::clone(rhs),
                }
            }
        }
    })
}

fn build_default_for_struct(
    item: &ItemStruct,
    e: &DeriveEntry,
    hattrs: &HelperAttributes,
    fields: &[FieldEntry],
) -> Result<TokenStream> {
    let kind = DeriveItemKind::Default;
    let (_, type_g, _) = item.generics.split_for_impl();
    let this_ty_ident = &item.ident;
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let generics = expand_self(&item.generics, &this_ty);
    let (impl_g, _, _) = generics.split_for_impl();
    let trait_ = kind.to_path();

    let mut wcb = WhereClauseBuilder::new(&generics);
    let use_bounds = e.push_bounds_to_with(hattrs, kind, &mut wcb);
    let value = if let Some(a) = &hattrs.default {
        a.value.as_ref()
    } else {
        None
    };
    let value = if let Some(value) = value {
        quote!(#value)
    } else {
        let ctor_args = build_default_ctor_args(&item.fields, fields, use_bounds, &mut wcb)?;
        quote!(#this_ty_ident #ctor_args)
    };
    let wheres = wcb.build(|ty| quote!(#ty : #trait_));
    Ok(quote! {
        impl #impl_g #trait_ for #this_ty #wheres {
            fn default() -> Self {
                #value
            }
        }
    })
}

fn build_default_for_enum(
    item: &ItemEnum,
    e: &DeriveEntry,
    hattrs: &HelperAttributes,
    variants: &[VariantEntry],
) -> Result<TokenStream> {
    let kind = DeriveItemKind::Default;
    let (_, type_g, _) = item.generics.split_for_impl();
    let this_ty_ident = &item.ident;
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let generics = expand_self(&item.generics, &this_ty);
    let (impl_g, _, _) = generics.split_for_impl();
    let trait_ = kind.to_path();

    let mut wcb = WhereClauseBuilder::new(&generics);
    let mut use_bounds = e.push_bounds_to_with(hattrs, kind, &mut wcb);
    let value = if let Some(value) = hattrs.default_value() {
        quote!(#value)
    } else {
        let vs: Vec<_> = variants
            .iter()
            .filter_map(|v| Some((v, v.hattrs.default.as_ref()?)))
            .collect();
        let a_default = HelperAttributeForDefault::default();
        let (v, a) = match vs.len() {
            0 => {
                if variants.len() == 1 {
                    (&variants[0], &a_default)
                } else {
                    bail!(_, "variant with `#[default(...)]` does not exist.")
                }
            }
            1 => vs[0],
            _ => {
                let names: Vec<String> = vs
                    .iter()
                    .map(|variant| variant.0.variant.ident.to_string())
                    .collect();

                bail!(
                    vs[0].0.variant.span(),
                    "there are multiple variants with `#[default(...)]` ({})",
                    names.join(", "),
                )
            }
        };
        use_bounds = v.hattrs.push_bounds_to(use_bounds, kind, &mut wcb);
        if let Some(value) = &a.value {
            bail!(
                value.span(),
                "`#[default(...)]` on a variant cannot specify a default value"
            )
        }
        let ctor_args =
            build_default_ctor_args(&v.variant.fields, &v.fields, use_bounds, &mut wcb)?;
        let variant_ident = &v.variant.ident;
        quote!(#this_ty_ident::#variant_ident #ctor_args)
    };
    let wheres = wcb.build(|ty| quote!(#ty : #trait_));
    Ok(quote! {
        impl #impl_g #trait_ for #this_ty #wheres {
            fn default() -> Self {
                #value
            }
        }
    })
}
fn build_default_ctor_args(
    fields_source: &Fields,
    fields: &[FieldEntry],
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let kind = DeriveItemKind::Default;
    let trait_ = kind.to_path();
    let mut ctor_args = Vec::new();
    for field in fields {
        let value = field.hattrs.default_value();
        if field.hattrs.push_bounds_to(use_bounds, kind, wcb) && value.is_none() {
            wcb.push_bounds_for_field(field.field)
        }
        let value = if let Some(value) = value {
            value.to_token_stream()
        } else {
            let field_ty = &field.field.ty;
            quote!(<#field_ty as #trait_>::default())
        };
        ctor_args.push(value);
    }
    Ok(build_ctor_args(fields_source, &ctor_args))
}

fn build_deref_for_struct(
    item: &ItemStruct,
    e: &DeriveEntry,
    fields: &[FieldEntry],
) -> Result<TokenStream> {
    let kind = DeriveItemKind::Deref;
    let (_, type_g, _) = item.generics.split_for_impl();
    let this_ty_ident = &item.ident;
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let generics = expand_self(&item.generics, &this_ty);
    let (impl_g, _, _) = generics.split_for_impl();
    let trait_ = kind.to_path();
    let mut wcb = WhereClauseBuilder::new(&generics);
    e.push_bounds_to(&mut wcb);

    if fields.len() != 1 {
        bail!(
            Span::call_site(),
            "`#[deirve_ex(Deref)]` supports only single field struct."
        );
    }
    let target_ty = &fields[0].field.ty;
    let member = fields[0].member();

    let wheres = wcb.build(|ty| quote!(#ty : #trait_));
    Ok(quote! {
        impl #impl_g #trait_ for #this_ty #wheres {
            type Target = #target_ty;
            fn deref(&self) -> & #target_ty {
                &self.#member
            }
        }
    })
}
fn build_deref_mut_for_struct(
    item: &ItemStruct,
    e: &DeriveEntry,
    fields: &[FieldEntry],
) -> Result<TokenStream> {
    let kind = DeriveItemKind::DerefMut;
    let (_, type_g, _) = item.generics.split_for_impl();
    let this_ty_ident = &item.ident;
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let generics = expand_self(&item.generics, &this_ty);
    let (impl_g, _, _) = generics.split_for_impl();
    let trait_ = kind.to_path();
    let mut wcb = WhereClauseBuilder::new(&generics);
    e.push_bounds_to(&mut wcb);

    if fields.len() != 1 {
        bail!(
            Span::call_site(),
            "`#[deirve_ex(DerefMut)]` supports only single field struct."
        );
    }
    let target_ty = &fields[0].field.ty;
    let member = fields[0].member();

    let wheres = wcb.build(|ty| quote!(#ty : #trait_));
    Ok(quote! {
        impl #impl_g #trait_ for #this_ty #wheres {
            fn deref_mut(&mut self) -> &mut #target_ty {
                &mut self.#member
            }
        }
    })
}

fn with_ref(source: &impl ToTokens, is_ref: bool) -> TokenStream {
    if is_ref {
        quote!(&#source)
    } else {
        quote!(#source)
    }
}
fn build_ctor_args(fields: &Fields, values: &[TokenStream]) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            let names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
            quote!({ #(#names: #values,)* })
        }
        Fields::Unnamed(_) => quote!((#(#values,)*)),
        Fields::Unit => quote!(),
    }
}
fn member(this: TokenStream, field: &FieldEntry) -> TokenStream {
    let member = field.member();
    quote!(#this.#member)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum UnaryOp {
    Neg,
    Not,
}

impl UnaryOp {
    fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "Neg" => Self::Neg,
            "Not" => Self::Not,
            _ => return None,
        })
    }
    fn to_str(self) -> &'static str {
        match self {
            UnaryOp::Neg => "Neg",
            UnaryOp::Not => "Not",
        }
    }
    fn to_func_name(self) -> &'static str {
        match self {
            UnaryOp::Neg => "neg",
            UnaryOp::Not => "not",
        }
    }
}
impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum DeriveItemKind {
    BinaryOp(BinaryOp),
    AssignOp(BinaryOp),
    UnaryOp(UnaryOp),
    Clone,
    Default,
    Deref,
    DerefMut,
    // Index,
    // IndexMut,
    // AsRef,
    // AsMut,
    // From,
    // Into,
    // TryInto,
}

impl DeriveItemKind {
    fn from_str(s: &str) -> Option<Self> {
        if let Some(s) = s.strip_suffix("Assign") {
            return Some(Self::AssignOp(BinaryOp::from_str(s)?));
        }
        if let Some(value) = BinaryOp::from_str(s) {
            return Some(Self::BinaryOp(value));
        }
        if let Some(value) = UnaryOp::from_str(s) {
            return Some(Self::UnaryOp(value));
        }
        Some(match s {
            "Clone" => Self::Clone,
            "Default" => Self::Default,
            "Deref" => Self::Deref,
            "DerefMut" => Self::DerefMut,
            // "Index" => Self::Index,
            // "IndexMut" => Self::IndexMut,
            // "AsRef" => Self::AsRef,
            // "AsMut" => Self::AsMut,
            // "From" => Self::From,
            // "Into" => Self::Into,
            // "TryInto" => Self::TryInto,
            _ => return None,
        })
    }
    fn from_ident(s: &Ident) -> Result<Self> {
        let span = s.span();
        let s = s.to_string();
        if let Some(value) = Self::from_str(&s) {
            Ok(value)
        } else {
            bail!(span, "unsupported trait");
        }
    }

    fn to_path(self) -> Path {
        match self {
            DeriveItemKind::BinaryOp(op) => {
                let ident = format_ident!("{}", op.to_str());
                parse_quote!(::core::ops::#ident)
            }
            DeriveItemKind::AssignOp(op) => {
                let ident = format_ident!("{}Assign", op.to_str());
                parse_quote!(::core::ops::#ident)
            }
            DeriveItemKind::UnaryOp(op) => {
                let ident = format_ident!("{}", op.to_str());
                parse_quote!(::core::ops::#ident)
            }
            DeriveItemKind::Clone => parse_quote!(::core::clone::Clone),
            DeriveItemKind::Default => parse_quote!(::core::default::Default),
            DeriveItemKind::Deref => parse_quote!(::core::ops::Deref),
            DeriveItemKind::DerefMut => parse_quote!(::core::ops::DerefMut),
        }
    }
}
impl std::fmt::Display for DeriveItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeriveItemKind::BinaryOp(op) => write!(f, "{}", op),
            DeriveItemKind::AssignOp(op) => write!(f, "{}Assign", op),
            DeriveItemKind::UnaryOp(op) => write!(f, "{}", op),
            DeriveItemKind::Clone => write!(f, "Clone"),
            DeriveItemKind::Default => write!(f, "Default"),
            DeriveItemKind::Deref => write!(f, "Deref"),
            DeriveItemKind::DerefMut => write!(f, "DerefMut"),
        }
    }
}

struct DeriveEntry {
    kind: DeriveItemKind,
    span: Span,
    dump: bool,
    bounds_this: Bounds,
    bounds_common: Bounds,
}
impl DeriveEntry {
    fn from_root(attr: TokenStream, attrs: &[Attribute]) -> Result<Vec<Self>> {
        let mut args_list = vec![parse2(attr)?];
        args_list.extend(parse_derive_ex_attrs(attrs)?);
        Self::from_args_list(&args_list)
    }
    fn from_args_list(args_list: &[Args]) -> Result<Vec<Self>> {
        let mut results = Vec::new();
        for a in args_list {
            for item in &a.items {
                let (dump, bounds_this) =
                    if let DeriveItemArgsOption::Some { args, .. } = &item.args {
                        (args.dump, Bounds::from(&args.bound))
                    } else {
                        (false, Bounds::new())
                    };
                results.push(Self {
                    kind: DeriveItemKind::from_ident(&item.trait_ident)?,
                    span: item.trait_ident.span(),
                    dump: a.dump | dump,
                    bounds_this,
                    bounds_common: Bounds::from(&a.bound),
                });
            }
        }
        Ok(results)
    }

    fn push_bounds_to(&self, wcb: &mut WhereClauseBuilder) -> bool {
        let mut use_bounds = wcb.push_bounds(&self.bounds_this);
        if use_bounds {
            use_bounds = wcb.push_bounds(&self.bounds_common);
        }
        use_bounds
    }
    fn push_bounds_to_with(
        &self,
        hattrs: &HelperAttributes,
        kind: DeriveItemKind,
        wcb: &mut WhereClauseBuilder,
    ) -> bool {
        let mut use_bounds = hattrs.push_bounds_to(true, kind, wcb);
        if use_bounds {
            use_bounds = wcb.push_bounds(&self.bounds_this)
        }
        if use_bounds {
            use_bounds = wcb.push_bounds(&self.bounds_common);
        }
        use_bounds
    }

    fn apply_dump(&self, result: Result<TokenStream>) -> TokenStream {
        match (result, self.dump) {
            (Ok(ts), false) => ts,
            (Ok(ts), true) => Error::new(self.span, format!("dump:\n{}", ts)).to_compile_error(),
            (Err(e), _) => e.to_compile_error(),
        }
    }
}

struct VariantEntry<'a> {
    variant: &'a Variant,
    fields: Vec<FieldEntry<'a>>,
    hattrs: HelperAttributes,
}

impl<'a> VariantEntry<'a> {
    fn new(variant: &'a Variant, kinds: &HelperAttributeKinds) -> Result<Self> {
        Ok(Self {
            variant,
            fields: FieldEntry::from_fields(&variant.fields, kinds)?,
            hattrs: HelperAttributes::from_attrs(&variant.attrs, kinds)?,
        })
    }
    fn from_variants(
        variants: impl IntoIterator<Item = &'a Variant>,
        kinds: &HelperAttributeKinds,
    ) -> Result<Vec<Self>> {
        variants
            .into_iter()
            .map(|variant| Self::new(variant, kinds))
            .collect()
    }
}

struct FieldEntry<'a> {
    index: usize,
    field: &'a Field,
    hattrs: HelperAttributes,
}

impl<'a> FieldEntry<'a> {
    fn new(index: usize, field: &'a Field, kinds: &HelperAttributeKinds) -> Result<Self> {
        Ok(Self {
            index,
            field,
            hattrs: HelperAttributes::from_attrs(&field.attrs, kinds)?,
        })
    }
    fn from_fields(fields: &'a Fields, kinds: &HelperAttributeKinds) -> Result<Vec<Self>> {
        fields
            .iter()
            .enumerate()
            .map(|(index, field)| Self::new(index, field, kinds))
            .collect()
    }

    fn member(&self) -> TokenStream {
        if let Some(ident) = &self.field.ident {
            quote!(#ident)
        } else {
            let index = Index::from(self.index);
            quote!(#index)
        }
    }
    fn make_ident(&self, prefix: &str) -> Ident {
        if let Some(ident) = &self.field.ident {
            format_ident!("{}_{}", prefix, ident)
        } else {
            format_ident!("{}_{}", prefix, self.index)
        }
    }
    fn push_bounds_to(&self, use_bounds: bool, kind: DeriveItemKind, wcb: &mut WhereClauseBuilder) {
        if self.hattrs.push_bounds_to(use_bounds, kind, wcb) {
            wcb.push_bounds_for_field(self.field)
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct HelperAttributeKinds {
    derive_ex: bool,
    default: bool,
}

impl HelperAttributeKinds {
    fn new(derive_ex: bool) -> Self {
        Self {
            derive_ex,
            default: false,
        }
    }
    fn extend(&mut self, es: &[DeriveEntry]) {
        for e in es {
            if e.kind == DeriveItemKind::Default {
                self.default = true
            }
        }
    }

    fn is_match(&self, attr: &Attribute) -> bool {
        let p = &attr.path;
        (self.derive_ex && p.is_ident("derive_ex")) || (self.default && p.is_ident("default"))
    }

    fn without_derive_ex(&self) -> HelperAttributeKinds {
        HelperAttributeKinds {
            derive_ex: false,
            ..*self
        }
    }
}

struct HelperAttributes {
    items: HashMap<DeriveItemKind, DeriveEntry>,
    default: Option<HelperAttributeForDefault>,
}

impl HelperAttributes {
    fn from_attrs(attrs: &[Attribute], kinds: &HelperAttributeKinds) -> Result<Self> {
        let items = if kinds.derive_ex {
            DeriveEntry::from_args_list(&parse_derive_ex_attrs(attrs)?)?
                .into_iter()
                .map(|x| (x.kind, x))
                .collect()
        } else {
            HashMap::new()
        };
        let default = if kinds.default {
            HelperAttributeForDefault::from_attrs(attrs)?
        } else {
            None
        };
        Ok(Self { items, default })
    }

    #[must_use]
    fn push_bounds_to(
        &self,
        mut use_bounds: bool,
        kind: DeriveItemKind,
        wcb: &mut WhereClauseBuilder,
    ) -> bool {
        if use_bounds && kind == DeriveItemKind::Default {
            if let Some(a) = &self.default {
                use_bounds = wcb.push_bounds(&a.bounds)
            }
        }
        if use_bounds {
            if let Some(a) = self.items.get(&kind) {
                use_bounds = a.push_bounds_to(wcb);
            }
        }
        use_bounds
    }
    fn default_value(&self) -> Option<&Expr> {
        if let Some(a) = &self.default {
            a.value.as_ref()
        } else {
            None
        }
    }
}

#[derive(Default)]
struct HelperAttributeForDefault {
    value: Option<Expr>,
    bounds: Bounds,
}

impl HelperAttributeForDefault {
    fn from_attrs(attrs: &[Attribute]) -> Result<Option<Self>> {
        if let Some(args) = parse_single::<ArgsForDefault>(attrs, "default")? {
            let value = if args.value == parse_quote!(_) {
                None
            } else {
                Some(args.value)
            };
            Ok(Some(Self {
                value,
                bounds: Bounds::from(&args.bound),
            }))
        } else {
            Ok(None)
        }
    }
}

fn remove_attrs(attrs: &mut Vec<Attribute>, kinds: &HelperAttributeKinds) {
    attrs.retain(|attr| !kinds.is_match(attr));
}

fn parse_derive_ex_attrs<T: Parse>(attrs: &[Attribute]) -> Result<Vec<T>> {
    let mut items = Vec::new();
    for attr in attrs {
        if attr.path == parse_quote!(derive_ex) {
            items.push(attr.parse_args()?);
        }
    }
    Ok(items)
}
fn parse_single<T: Parse + Default>(attrs: &[Attribute], name: &str) -> Result<Option<T>> {
    let mut item = None;
    for attr in attrs {
        if attr.path.is_ident(name) {
            if item.is_some() {
                bail!(attr.span(), "#[{}] was specified twice", name)
            }
            item = Some(if attr.tokens.is_empty() {
                Default::default()
            } else {
                attr.parse_args()?
            });
        }
    }
    Ok(item)
}
