use std::iter::once;

use crate::bound::{Bounds, WhereClauseBuilder};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use structmeta::{Flag, ToTokens};
use syn::{
    parse::Parse, parse2, parse_quote, spanned::Spanned, Attribute, Expr, ItemStruct, Result, Type,
};

use super::{
    parse_single, ArgsForCompareOp, CompareOp, DeriveEntry, DeriveItemKind, FieldEntry,
    HelperAttributeKinds,
};

pub(super) fn build_compare_op(
    item: &ItemStruct,
    op: CompareOp,
    e: &DeriveEntry,
    fields: &[FieldEntry],
) -> Result<TokenStream> {
    let kind = DeriveItemKind::CompareOp(op);
    let (impl_g, type_g, _) = item.generics.split_for_impl();
    let this_ty_ident = &item.ident;
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let trait_ = kind.to_path();

    let mut wcb = WhereClauseBuilder::new(&item.generics);
    let use_bounds = e.push_bounds_to(&mut wcb);

    let body = match op {
        CompareOp::PartialEq => build_partial_eq_body(fields, use_bounds, &mut wcb)?,
        CompareOp::Eq => build_eq_body(fields, use_bounds, &mut wcb)?,
        CompareOp::PartialOrd => build_partial_ord_body(fields, use_bounds, &mut wcb)?,
        CompareOp::Ord => build_ord_body(fields, use_bounds, &mut wcb)?,
        CompareOp::Hash => build_hash_body(fields, use_bounds, &mut wcb)?,
    };
    let wheres = wcb.build(|ty| quote!(#ty : #trait_));

    let (body, checker) = match op {
        CompareOp::PartialEq | CompareOp::PartialOrd | CompareOp::Ord | CompareOp::Hash => {
            (body, quote!())
        }
        CompareOp::Eq => (
            quote!(),
            quote! {
                const _: () = {
                    fn _f #impl_g (this: &#this_ty) #wheres {
                        #body
                    }
                };
            },
        ),
    };

    Ok(quote! {
        #[automatically_derived]
        impl #impl_g #trait_ for #this_ty #wheres {
            #body
        }

        #checker
    })
}
fn build_partial_eq_body(
    fields: &[FieldEntry],
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::PartialEq;
    let kind = DeriveItemKind::CompareOp(op);
    let mut exprs = Vec::new();
    for field in fields {
        if field.hattrs.cmp.is_ignore(op)? {
            continue;
        }
        let mut field_used = false;
        let mut use_bounds = use_bounds;
        exprs.push(build_partial_eq_expr(
            field,
            &mut field_used,
            &mut use_bounds,
            wcb,
        ));
        use_bounds = field.hattrs.push_bounds_to(use_bounds, kind, wcb);
        if use_bounds && field_used {
            wcb.push_bounds_for_field(field.field);
        }
    }
    let body = if exprs.is_empty() {
        quote!(true)
    } else {
        quote!(#(#exprs)&&*)
    };
    Ok(quote! {
        fn eq(&self, other: &Self) -> bool {
            #body
        }
    })
}
fn build_partial_eq_expr(
    field: &FieldEntry,
    field_used: &mut bool,
    use_bounds: &mut bool,
    wcb: &mut WhereClauseBuilder,
) -> TokenStream {
    let ty = &field.field.ty;
    let member = field.member();
    let fn_ident = field.make_ident("__eq_");
    let span = field.field.span();
    let this = quote_spanned!(span=>self.#member);
    let other = quote_spanned!(span=>other.#member);
    let cmp = &field.hattrs.cmp;
    cmp.partial_eq.push_bounds_to(use_bounds, wcb);

    let build_expr_by_eq = |by: &Expr| {
        quote! {
            {
                fn #fn_ident(this: &#ty, other: &#ty, eq: impl ::core::ops::Fn(&#ty, &#ty) -> bool) -> bool {
                    eq(this, other)
                }
                #fn_ident(&#this, &#other, #by)
            }
        }
    };

    if let Some(by) = &cmp.partial_eq.by {
        return build_expr_by_eq(by);
    }
    if let Some(key) = &cmp.partial_eq.key {
        return key.build_eq_expr(this, other);
    }

    cmp.eq.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &cmp.eq.by {
        return build_expr_by_eq(by);
    }
    if let Some(key) = &cmp.eq.key {
        return key.build_eq_expr(this, other);
    }

    cmp.partial_ord.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &cmp.partial_ord.by {
        return quote! {
            {
                fn #fn_ident(this: &#ty, other: &#ty, partial_cmp: impl Fn(&#ty, &#ty) -> ::core::option::Option<::core::cmp::Ordering>) -> bool {
                    partial_cmp(this, other) == ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                }
                #fn_ident(&#this, &#other, #by)
            }
        };
    }
    if let Some(key) = &cmp.partial_ord.key {
        return key.build_eq_expr(this, other);
    }

    cmp.ord.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &field.hattrs.cmp.ord.by {
        return quote! {
            {
                fn #fn_ident(this: &#ty, other: &#ty, cmp: impl ::core::ops::Fn(&#ty, &#ty) -> ::core::cmp::Ordering) -> bool {
                    cmp(this, other) == ::core::cmp::Ordering::Equal
                }
                #fn_ident(&#this, &#other, #by)
            }
        };
    }
    if let Some(key) = &field.hattrs.cmp.ord.key {
        return key.build_eq_expr(this, other);
    }
    *field_used = true;
    quote_spanned!(span=> ::core::cmp::PartialEq::eq(&(#this), &(#other)))
}

fn build_eq_body(
    fields: &[FieldEntry],
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Eq;
    let kind = DeriveItemKind::CompareOp(op);
    let mut exprs = Vec::new();
    for field in fields {
        if field.hattrs.cmp.is_ignore(op)? {
            continue;
        }
        let mut field_used = false;
        let mut use_bounds = use_bounds;
        exprs.push(build_eq_expr(field, &mut field_used, &mut use_bounds, wcb)?);
        use_bounds = field.hattrs.push_bounds_to(use_bounds, kind, wcb);
        if use_bounds && field_used {
            wcb.push_bounds_for_field(field.field);
        }
    }
    Ok(quote!(#(#exprs)*))
}
fn build_eq_expr(
    field: &FieldEntry,
    field_used: &mut bool,
    use_bounds: &mut bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Eq;
    let cmp = &field.hattrs.cmp;
    let member = field.member();
    let span = field.field.span();
    let this = quote_spanned!(span=> this.#member);

    cmp.eq.push_bounds_to(use_bounds, wcb);
    if cmp.eq.by.is_some() {
        return Ok(quote!());
    }
    if let Some(key) = &cmp.eq.key {
        return Ok(key.build_eq_checker(this));
    }

    if let Some(by) = &cmp.partial_eq.by {
        return bad_attr(by.span(), op, "eq(by = ...)", "partial_eq(by = ...)");
    }
    if let Some(key) = &cmp.partial_eq.key {
        return bad_attr(key.span(), op, "eq(key = ...)", "partial_eq(key = ...)");
    }

    cmp.ord.push_bounds_to(use_bounds, wcb);
    if cmp.ord.by.is_some() {
        return Ok(quote!());
    }
    if let Some(key) = &cmp.ord.key {
        return Ok(key.build_eq_checker(this));
    }
    if let Some(by) = &cmp.partial_ord.by {
        return bad_attr(by.span(), op, "ord(by = ...)", "partial_ord(by = ...)");
    }
    if let Some(key) = &cmp.partial_ord.key {
        return bad_attr(key.span(), op, "ord(key = ...)", "partial_ord(key = ...)");
    }

    *field_used = true;
    Ok(build_eq_checker(this))
}

fn build_partial_ord_body(
    fields: &[FieldEntry],
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::PartialOrd;
    let kind = DeriveItemKind::CompareOp(op);
    let mut body = TokenStream::new();
    for field in fields {
        if field.hattrs.cmp.is_ignore(op)? {
            continue;
        }
        let mut field_used = false;
        let mut use_bounds = use_bounds;
        let mut expr = build_partial_ord_expr(field, &mut field_used, &mut use_bounds, wcb)?;
        if field.hattrs.cmp.is_reverse(op)? {
            expr = quote!(::core::option::Option::map(#expr, ::core::cmp::Ordering::reverse));
        }
        body.extend(quote! {
            match #expr {
                Some(::core::cmp::Ordering::Equal) => {}
                o => return o,
            }
        });
        use_bounds = field.hattrs.push_bounds_to(use_bounds, kind, wcb);
        if use_bounds && field_used {
            wcb.push_bounds_for_field(field.field);
        }
    }
    Ok(quote! {
        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
            #body
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    })
}
fn build_partial_ord_expr(
    field: &FieldEntry,
    field_used: &mut bool,
    use_bounds: &mut bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::PartialOrd;
    let ty = &field.field.ty;
    let member = field.member();
    let fn_ident = field.make_ident("__partial_ord_");
    let span = field.field.span();
    let this = quote_spanned!(span=>self.#member);
    let other = quote_spanned!(span=>other.#member);
    let cmp = &field.hattrs.cmp;

    cmp.partial_ord.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &cmp.partial_ord.by {
        return Ok(quote! {
            {
                fn #fn_ident(
                    this: &#ty,
                    other: &#ty,
                    partial_cmp: impl Fn(&#ty, &#ty) -> ::core::option::Option<::core::cmp::Ordering>)
                 -> ::core::option::Option<::core::cmp::Ordering> {
                    partial_cmp(this, other)
                }
                #fn_ident(&#this, &#other, #by)
            }
        });
    }
    if let Some(key) = &cmp.partial_ord.key {
        return Ok(key.build_partial_cmp_expr(this, other));
    }

    cmp.ord.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &cmp.ord.by {
        return Ok(quote! {
            {
                fn #fn_ident(
                    this: &#ty,
                    other: &#ty,
                    cmp: impl Fn(&#ty, &#ty) -> ::core::cmp::Ordering)
                 -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::option::Option::Some(cmp(this, other))
                }
                #fn_ident(&#this, &#other, #by)
            }
        });
    }
    if let Some(key) = &cmp.ord.key {
        return Ok(key.build_partial_cmp_expr(this, other));
    }

    if let Some(by) = &cmp.partial_eq.by {
        return bad_attr(
            by.span(),
            op,
            "partial_ord(by = ...)",
            "partial_eq(by = ...)",
        );
    }
    if let Some(key) = &cmp.partial_eq.key {
        return bad_attr(
            key.span(),
            op,
            "partial_ord(key = ...)",
            "partial_eq(key = ...)",
        );
    }

    if let Some(by) = &cmp.eq.by {
        return bad_attr(by.span(), op, "ord(by = ...)", "eq(by = ...)");
    }
    if let Some(key) = &cmp.eq.key {
        return bad_attr(key.span(), op, "ord(key = ...)", "eq(key = ...)");
    }

    *field_used = true;
    Ok(quote_spanned!(span=> ::core::cmp::PartialOrd::partial_cmp(&(#this), &(#other))))
}

fn build_ord_body(
    fields: &[FieldEntry],
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Ord;
    let kind = DeriveItemKind::CompareOp(op);
    let mut body = TokenStream::new();
    for field in fields {
        if field.hattrs.cmp.is_ignore(op)? {
            continue;
        }
        let mut field_used = false;
        let mut use_bounds = use_bounds;
        let mut expr = build_ord_expr(field, &mut field_used, &mut use_bounds, wcb)?;
        if field.hattrs.cmp.is_reverse(op)? {
            expr = quote!(::core::cmp::Ordering::reverse(#expr));
        }
        body.extend(quote! {
            match #expr {
                ::core::cmp::Ordering::Equal => {}
                o => return o,
            }
        });
        use_bounds = field.hattrs.push_bounds_to(use_bounds, kind, wcb);
        if use_bounds && field_used {
            wcb.push_bounds_for_field(field.field);
        }
    }
    Ok(quote! {
        fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
            #body
            ::core::cmp::Ordering::Equal
        }
    })
}
fn build_ord_expr(
    field: &FieldEntry,
    field_used: &mut bool,
    use_bounds: &mut bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Ord;
    let ty = &field.field.ty;
    let member = field.member();
    let fn_ident = field.make_ident("__ord_");
    let span = field.field.span();
    let this = quote_spanned!(span=>self.#member);
    let other = quote_spanned!(span=>other.#member);
    let cmp = &field.hattrs.cmp;

    cmp.ord.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &cmp.ord.by {
        return Ok(quote! {
            {
                fn #fn_ident(
                    this: &#ty,
                    other: &#ty,
                    cmp: impl Fn(&#ty, &#ty) -> ::core::cmp::Ordering)
                 -> ::core::cmp::Ordering {
                    cmp(this, other)
                }
                #fn_ident(&#this, &#other, #by)
            }
        });
    }
    if let Some(key) = &cmp.ord.key {
        return Ok(key.build_cmp_expr(this, other));
    }

    if let Some(by) = &cmp.partial_ord.by {
        return bad_attr(by.span(), op, "ord(by = ...)", "partial_ord(by = ...)");
    }
    if let Some(key) = &cmp.partial_ord.key {
        return bad_attr(key.span(), op, "ord(key = ...)", "partial_ord(key = ...)");
    }

    if let Some(by) = &cmp.partial_eq.by {
        return bad_attr(by.span(), op, "ord(by = ...)", "partial_eq(by = ...)");
    }
    if let Some(key) = &cmp.partial_eq.key {
        return bad_attr(key.span(), op, "ord(key = ...)", "partial_eq(key = ...)");
    }

    if let Some(by) = &cmp.eq.by {
        return bad_attr(by.span(), op, "ord(by = ...)", "eq(by = ...)");
    }
    if let Some(key) = &cmp.eq.key {
        return bad_attr(key.span(), op, "ord(key = ...)", "eq(key = ...)");
    }

    *field_used = true;
    Ok(quote_spanned!(span=> ::core::cmp::Ord::cmp(&(#this), &(#other))))
}

fn build_hash_body(
    fields: &[FieldEntry],
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Hash;
    let kind = DeriveItemKind::CompareOp(op);
    let mut exprs = Vec::new();
    for field in fields {
        if field.hattrs.cmp.is_ignore(op)? {
            continue;
        }
        let mut field_used = false;
        let mut use_bounds = use_bounds;

        exprs.push(build_hash_expr(
            field,
            &mut field_used,
            &mut use_bounds,
            wcb,
        )?);
        use_bounds = field.hattrs.push_bounds_to(use_bounds, kind, wcb);
        if use_bounds && field_used {
            wcb.push_bounds_for_field(field.field);
        }
    }
    Ok(quote! {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            #(#exprs)*
        }
    })
}
fn build_hash_expr(
    field: &FieldEntry,
    field_used: &mut bool,
    use_bounds: &mut bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Hash;
    let ty = &field.field.ty;
    let member = field.member();
    let fn_ident = field.make_ident("__hash_");
    let span = field.field.span();
    let this = quote_spanned!(span=>self.#member);
    let cmp = &field.hattrs.cmp;

    cmp.hash.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &cmp.hash.by {
        return Ok(quote! {
            {
                fn #fn_ident<H: ::core::hash::Hasher>(
                    this: &#ty,
                    state: &mut H,
                    hash: impl Fn(&#ty, &mut H)) {
                    hash(this, state)
                }
                #fn_ident(&#this, state, #by)
            }
        });
    }
    if let Some(key) = &cmp.hash.key {
        return Ok(key.build_hash_stmt(this));
    }

    cmp.eq.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &cmp.eq.by {
        return bad_attr(by.span(), op, "hash(by = ...)", "eq(by = ...)");
    }
    if let Some(key) = &cmp.eq.key {
        return Ok(key.build_hash_stmt(this));
    }

    cmp.ord.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &cmp.ord.by {
        return bad_attr(by.span(), op, "hash(by = ...)", "ord(by = ...)");
    }
    if let Some(key) = &cmp.ord.key {
        return Ok(key.build_hash_stmt(this));
    }

    if let Some(by) = &cmp.partial_eq.by {
        return bad_attr(by.span(), op, "hash(by = ...)", "partial_eq(by = ...)");
    }
    if let Some(key) = &cmp.partial_eq.key {
        return bad_attr(key.span(), op, "eq(key = ...)", "partial_eq(by = ...)");
    }

    if let Some(by) = &cmp.partial_ord.by {
        return bad_attr(by.span(), op, "hash(by = ...)", "partial_ord(by = ...)");
    }
    if let Some(key) = &cmp.partial_ord.key {
        return bad_attr(key.span(), op, "ord(key = ...)", "partial_ord(by = ...)");
    }

    *field_used = true;
    Ok(quote_spanned!(span=> ::core::hash::Hash::hash(&(#this), state);))
}

pub(super) struct HelperAttribtuesForCompareOp {
    ord: HelperAttributeForComapreOp,
    partial_ord: HelperAttributeForComapreOp,
    eq: HelperAttributeForComapreOp,
    partial_eq: HelperAttributeForComapreOp,
    hash: HelperAttributeForComapreOp,
}
impl HelperAttribtuesForCompareOp {
    pub fn from_attrs(attrs: &[Attribute], kinds: &HelperAttributeKinds) -> Result<Self> {
        let ord = if kinds.is_match_cmp_attr(CompareOp::Ord) {
            HelperAttributeForComapreOp::from_attrs(attrs, CompareOp::Ord)?
        } else {
            HelperAttributeForComapreOp::default()
        };
        let partial_ord = if kinds.is_match_cmp_attr(CompareOp::PartialOrd) {
            HelperAttributeForComapreOp::from_attrs(attrs, CompareOp::PartialOrd)?
        } else {
            HelperAttributeForComapreOp::default()
        };
        let eq = if kinds.is_match_cmp_attr(CompareOp::Eq) {
            HelperAttributeForComapreOp::from_attrs(attrs, CompareOp::Eq)?
        } else {
            HelperAttributeForComapreOp::default()
        };
        let partial_eq = if kinds.is_match_cmp_attr(CompareOp::PartialEq) {
            HelperAttributeForComapreOp::from_attrs(attrs, CompareOp::PartialEq)?
        } else {
            HelperAttributeForComapreOp::default()
        };
        let hash = if kinds.is_match_cmp_attr(CompareOp::Hash) {
            HelperAttributeForComapreOp::from_attrs(attrs, CompareOp::Hash)?
        } else {
            HelperAttributeForComapreOp::default()
        };
        Ok(Self {
            ord,
            partial_ord,
            eq,
            partial_eq,
            hash,
        })
    }
    fn get(&self, op: CompareOp) -> &HelperAttributeForComapreOp {
        match op {
            CompareOp::Ord => &self.ord,
            CompareOp::PartialOrd => &self.partial_ord,
            CompareOp::Eq => &self.eq,
            CompareOp::PartialEq => &self.partial_eq,
            CompareOp::Hash => &self.hash,
        }
    }
    fn is_ignore(&self, op: CompareOp) -> Result<bool> {
        let bad_flag = |bad: CompareOp, good: CompareOp| -> Result<()> {
            if let Some(span) = self.get(bad).ignore.span {
                let good = good.to_str_snake_case();
                let bad = bad.to_str_snake_case();
                bad_attr(
                    span,
                    op,
                    &format!("{good}(ignore)"),
                    &format!("{bad}(ignore)"),
                )
            } else {
                Ok(())
            }
        };
        match op {
            CompareOp::Ord => {
                if self.ord.ignore.value() {
                    return Ok(true);
                }
                bad_flag(CompareOp::PartialOrd, CompareOp::Ord)?;
                bad_flag(CompareOp::PartialEq, CompareOp::Ord)?;
                bad_flag(CompareOp::Eq, CompareOp::Ord)?;
                Ok(false)
            }
            CompareOp::PartialOrd => {
                if self.partial_ord.ignore.value() || self.ord.ignore.value() {
                    return Ok(true);
                }
                bad_flag(CompareOp::PartialEq, CompareOp::PartialOrd)?;
                bad_flag(CompareOp::Eq, CompareOp::Ord)?;
                Ok(false)
            }
            CompareOp::Eq => {
                if self.eq.ignore.value() || self.ord.ignore.value() {
                    return Ok(true);
                }
                bad_flag(CompareOp::PartialEq, CompareOp::Eq)?;
                bad_flag(CompareOp::PartialOrd, CompareOp::Ord)?;
                Ok(false)
            }
            CompareOp::PartialEq => Ok(self.partial_eq.ignore.value()
                || self.eq.ignore.value()
                || self.partial_ord.ignore.value()
                || self.ord.ignore.value()),
            CompareOp::Hash => {
                if self.hash.ignore.value() || self.eq.ignore.value() || self.ord.ignore.value() {
                    return Ok(true);
                }
                bad_flag(CompareOp::PartialEq, CompareOp::Eq)?;
                bad_flag(CompareOp::PartialOrd, CompareOp::Ord)?;
                Ok(false)
            }
        }
    }
    fn is_reverse(&self, op: CompareOp) -> Result<bool> {
        match op {
            CompareOp::Ord => {
                if let Some(span) = self.partial_ord.reverse.span {
                    return bad_attr(span, op, "ord(reverse)", "partial_ord(reverse)");
                }
                Ok(self.ord.reverse.value())
            }
            CompareOp::PartialOrd => {
                Ok(self.partial_ord.reverse.value() || self.ord.reverse.value())
            }
            CompareOp::Eq | CompareOp::PartialEq | CompareOp::Hash => {
                unreachable!()
            }
        }
    }
}

#[derive(Default)]
struct HelperAttributeForComapreOp {
    ignore: Flag,
    reverse: Flag,
    by: Option<Expr>,
    key: Option<Template>,
    bounds: Bounds,
}
impl HelperAttributeForComapreOp {
    fn from_attrs(attrs: &[Attribute], op: CompareOp) -> Result<Self> {
        if let Some(args) =
            parse_single::<TemplateOf<ArgsForCompareOp>>(attrs, op.to_str_snake_case())?
        {
            let args = args.0;
            Ok(Self {
                ignore: args.ignore,
                reverse: args.reverse,
                by: args.by.map(|x| x.value),
                key: args.key.map(|x| Template::new(x.value)),
                bounds: Bounds::from(&args.bound),
            })
        } else {
            Ok(Self::default())
        }
    }
    fn push_bounds_to(&self, use_bounds: &mut bool, wcb: &mut WhereClauseBuilder) {
        if *use_bounds {
            *use_bounds = wcb.push_bounds(&self.bounds)
        }
    }
}

fn replace_tokens(
    input: TokenStream,
    is_match: &impl Fn(&TokenTree) -> bool,
    replacer: &TokenStream,
) -> TokenStream {
    let mut ts = TokenStream::new();
    for i in input {
        if is_match(&i) {
            let span = i.span();
            for mut t in replacer.clone() {
                t.set_span(span);
                ts.extend(once(t));
            }
        } else if let TokenTree::Group(g) = &i {
            let mut g = proc_macro2::Group::new(
                g.delimiter(),
                replace_tokens(g.stream(), is_match, replacer),
            );
            g.set_span(i.span());
            ts.extend(g.to_token_stream())
        } else {
            ts.extend(i.to_token_stream());
        }
    }
    ts
}

fn placeholder() -> &'static str {
    "__placeholder"
}

fn dollar_token_to_placeholder(input: TokenStream) -> TokenStream {
    replace_tokens(
        input,
        &|t| matches!(t, TokenTree::Punct(p) if p.as_char() == '$'),
        &proc_macro2::Ident::new(placeholder(), Span::call_site()).to_token_stream(),
    )
}

#[derive(Default)]
struct TemplateOf<T>(T);

impl<T: Parse> Parse for TemplateOf<T> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        Ok(Self(parse2::<T>(dollar_token_to_placeholder(
            input.parse()?,
        ))?))
    }
}

#[derive(ToTokens)]
struct Template(TokenStream);

impl Template {
    fn new(input: impl ToTokens) -> Self {
        Self(input.to_token_stream())
    }
    fn apply(&self, value: TokenStream) -> TokenStream {
        replace_tokens(
            self.0.clone(),
            &|t| matches!(t, TokenTree::Ident(i) if i == placeholder()),
            &value,
        )
    }

    fn build_eq_expr(&self, this: TokenStream, other: TokenStream) -> TokenStream {
        let this = self.apply(this);
        let other = self.apply(other);
        quote_spanned!(self.span()=> ::core::cmp::PartialEq::eq(&(#this), &(#other)))
    }

    fn build_eq_checker(&self, this: TokenStream) -> TokenStream {
        build_eq_checker(self.apply(this))
    }

    fn build_partial_cmp_expr(&self, this: TokenStream, other: TokenStream) -> TokenStream {
        let this = self.apply(this);
        let other = self.apply(other);
        quote_spanned!(self.span()=> ::core::cmp::PartialOrd::partial_cmp(&(#this), &(#other)))
    }

    fn build_cmp_expr(&self, this: TokenStream, other: TokenStream) -> TokenStream {
        let this = self.apply(this);
        let other = self.apply(other);
        quote_spanned!(self.span()=> ::core::cmp::Ord::cmp(&(#this), &(#other)))
    }

    fn build_hash_stmt(&self, this: TokenStream) -> TokenStream {
        let this = self.apply(this);
        quote_spanned!(this.span()=> ::core::hash::Hash::hash(&(#this), state);)
    }
}

fn build_eq_checker(this: TokenStream) -> TokenStream {
    quote_spanned!(this.span()=>{
        fn _eq<T: Eq + ?Sized>(_this: &T) { }
        _eq(&(#this))
    })
}

fn bad_attr<T>(span: Span, op: CompareOp, good: &str, bad: &str) -> Result<T> {
    bail!(
        span,
        "When `#[derive_ex({op})]` is specified, `#[{good}]` must be used instead of `#[{bad}]`."
    )
}
