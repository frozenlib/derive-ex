use std::iter::once;

use crate::bound::{Bounds, WhereClauseBuilder};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use structmeta::{Flag, ToTokens};
use syn::{
    parse::Parse, parse2, parse_quote, spanned::Spanned, Attribute, Expr, Generics, Ident,
    ItemEnum, ItemStruct, Result, Type,
};

use super::{
    parse_single, ArgsForCompareOp, AttributeTarget, CompareOp, DeriveEntry, DeriveItemKind,
    FieldEntry, HelperAttributeKinds, HelperAttributes, VariantEntry,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ItemSourceKind {
    Struct,
    Enum,
}

impl ItemSourceKind {
    fn self_of(self, field: &FieldEntry) -> TokenStream {
        let span = field.field.span();
        match self {
            ItemSourceKind::Struct => {
                let member = field.member();
                quote_spanned!(span=> (self.#member))
            }
            ItemSourceKind::Enum => {
                let ident = field.make_ident("_self");
                quote_spanned!(span=> (*#ident))
            }
        }
    }
    fn this_of(self, field: &FieldEntry) -> TokenStream {
        let span = field.field.span();
        match self {
            ItemSourceKind::Struct => {
                let member = field.member();
                quote_spanned!(span=> (this.#member))
            }
            ItemSourceKind::Enum => {
                let ident = field.make_ident("_this");
                quote_spanned!(span=> (*#ident))
            }
        }
    }
    fn other_of(self, field: &FieldEntry) -> TokenStream {
        let span = field.field.span();
        match self {
            ItemSourceKind::Struct => {
                let member = field.member();
                quote_spanned!(span=> (other.#member))
            }
            ItemSourceKind::Enum => {
                let ident = field.make_ident("_other");
                quote_spanned!(span=> (*#ident))
            }
        }
    }
}

#[derive(Copy, Clone)]
enum ItemSource<'a> {
    Struct {
        item: &'a ItemStruct,
        fields: &'a [FieldEntry<'a>],
    },
    Enum {
        item: &'a ItemEnum,
        variants: &'a [VariantEntry<'a>],
    },
}
impl<'a> ItemSource<'a> {
    fn generics(&self) -> &Generics {
        match self {
            Self::Struct { item, .. } => &item.generics,
            Self::Enum { item, .. } => &item.generics,
        }
    }
    fn ident(&self) -> &Ident {
        match self {
            Self::Struct { item, .. } => &item.ident,
            Self::Enum { item, .. } => &item.ident,
        }
    }
    fn kind(&self) -> ItemSourceKind {
        match self {
            Self::Struct { .. } => ItemSourceKind::Struct,
            Self::Enum { .. } => ItemSourceKind::Enum,
        }
    }
}

pub(super) fn build_compare_op_for_struct(
    op: CompareOp,
    item: &ItemStruct,
    e: &DeriveEntry,
    hattrs: &HelperAttributes,
    fields: &[FieldEntry],
) -> Result<TokenStream> {
    build_compare_op(op, ItemSource::Struct { item, fields }, e, hattrs)
}
pub(super) fn build_compare_op_for_enum(
    op: CompareOp,
    item: &ItemEnum,
    e: &DeriveEntry,
    hattrs: &HelperAttributes,
    variants: &[VariantEntry],
) -> Result<TokenStream> {
    build_compare_op(op, ItemSource::Enum { item, variants }, e, hattrs)
}

fn build_compare_op(
    op: CompareOp,
    source: ItemSource,
    e: &DeriveEntry,
    hattrs: &HelperAttributes,
) -> Result<TokenStream> {
    let kind = DeriveItemKind::CompareOp(op);
    let (impl_g, type_g, _) = source.generics().split_for_impl();
    let this_ty_ident = source.ident();
    let this_ty: Type = parse_quote!(#this_ty_ident #type_g);
    let trait_ = kind.to_path();

    let mut wcb = WhereClauseBuilder::new(source.generics());
    let use_bounds = e.push_bounds_to_with(hattrs, kind, &mut wcb);
    let body = match op {
        CompareOp::PartialEq => build_partial_eq_body(source, use_bounds, &mut wcb)?,
        CompareOp::Eq => build_eq_body(source, use_bounds, &mut wcb)?,
        CompareOp::PartialOrd => build_partial_ord_body(source, use_bounds, &mut wcb)?,
        CompareOp::Ord => build_ord_body(source, use_bounds, &mut wcb)?,
        CompareOp::Hash => build_hash_body(source, use_bounds, &mut wcb)?,
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
                    #[allow(clippy::double_parens)]
                    #[allow(unused_parens)]
                    fn _f #impl_g (this: &#this_ty) #wheres {
                        #body
                    }
                };
            },
        ),
    };

    Ok(quote! {
        #[automatically_derived]
        #[allow(clippy::double_parens)]
        #[allow(unused_parens)]
        impl #impl_g #trait_ for #this_ty #wheres {
            #body
        }

        #checker
    })
}

fn build_partial_eq_body(
    source: ItemSource,
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::PartialEq;
    let kind = DeriveItemKind::CompareOp(op);
    let build_from_fields = |fields: &[FieldEntry],
                             use_bounds: bool,
                             wcb: &mut WhereClauseBuilder|
     -> Result<TokenStream> {
        let mut exprs = Vec::new();
        for field in fields {
            if field.hattrs.cmp.is_ignore(op)? {
                continue;
            }
            let mut field_used = false;
            let mut use_bounds = use_bounds;
            exprs.push(build_partial_eq_expr(
                source.kind(),
                field,
                &mut field_used,
                &mut use_bounds,
                wcb,
            )?);
            use_bounds = field
                .hattrs
                .push_bounds_to_without_helper(use_bounds, kind, wcb);
            if use_bounds && field_used {
                wcb.push_bounds_for_field(field.field);
            }
        }
        Ok(if exprs.is_empty() {
            quote!(true)
        } else {
            quote!(#(#exprs)&&*)
        })
    };
    let body = match source {
        ItemSource::Struct { fields, .. } => build_from_fields(fields, use_bounds, wcb)?,
        ItemSource::Enum { variants, .. } => {
            let mut arms = Vec::new();
            for variant in variants {
                let use_bounds = variant.hattrs.push_bounds_to(use_bounds, kind, wcb);
                let body = build_from_fields(&variant.fields, use_bounds, wcb)?;
                let pat_this = variant.make_pat("_self");
                let pat_other = variant.make_pat("_other");
                arms.push(quote!((#pat_this, #pat_other) => { #body }))
            }
            quote! {
                match (self, other) {
                    #(#arms)*
                    _ => false,
                }
            }
        }
    };
    Ok(quote! {
        fn eq(&self, other: &Self) -> bool {
            #body
        }
    })
}
fn build_partial_eq_expr(
    source: ItemSourceKind,
    field: &FieldEntry,
    field_used: &mut bool,
    use_bounds: &mut bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::PartialEq;
    let ty = &field.field.ty;
    let fn_ident = field.make_ident("__eq_");
    let this = source.self_of(field);
    let other = source.other_of(field);
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
        return Ok(build_expr_by_eq(by));
    }
    if let Some(key) = &cmp.partial_eq.key {
        return Ok(key.build_eq_expr(this, other));
    }

    cmp.eq.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &cmp.eq.by {
        return Ok(build_expr_by_eq(by));
    }
    if let Some(key) = &cmp.eq.key {
        return Ok(key.build_eq_expr(this, other));
    }

    cmp.partial_ord.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &cmp.partial_ord.by {
        return Ok(quote! {
            {
                fn #fn_ident(this: &#ty, other: &#ty, partial_cmp: impl Fn(&#ty, &#ty) -> ::core::option::Option<::core::cmp::Ordering>) -> bool {
                    partial_cmp(this, other) == ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                }
                #fn_ident(&#this, &#other, #by)
            }
        });
    }
    if let Some(key) = &cmp.partial_ord.key {
        return Ok(key.build_eq_expr(this, other));
    }

    cmp.ord.push_bounds_to(use_bounds, wcb);
    if let Some(by) = &field.hattrs.cmp.ord.by {
        return Ok(quote! {
            {
                fn #fn_ident(this: &#ty, other: &#ty, cmp: impl ::core::ops::Fn(&#ty, &#ty) -> ::core::cmp::Ordering) -> bool {
                    cmp(this, other) == ::core::cmp::Ordering::Equal
                }
                #fn_ident(&#this, &#other, #by)
            }
        });
    }
    if let Some(key) = &field.hattrs.cmp.ord.key {
        return Ok(key.build_eq_expr(this, other));
    }
    if let Some(bad) = cmp.bad_attr() {
        return bad_attr(
            op,
            bad,
            &[
                "partial_eq(key = ...)",
                "partial_eq(by = ...)",
                "eq(key = ...)",
                "eq(by = ...)",
                "partial_ord(key = ...)",
                "partial_ord(by = ...)",
                "ord(key = ...)",
                "ord(by = ...)",
            ],
        );
    }

    *field_used = true;
    Ok(quote_spanned!(field.field.span()=> ::core::cmp::PartialEq::eq(&(#this), &(#other))))
}

fn build_eq_body(
    source: ItemSource,
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Eq;
    let kind = DeriveItemKind::CompareOp(op);

    let build_from_fields = |fields: &[FieldEntry],
                             use_bounds: bool,
                             wcb: &mut WhereClauseBuilder|
     -> Result<TokenStream> {
        let mut exprs = Vec::new();
        for field in fields {
            if field.hattrs.cmp.is_ignore(op)? {
                continue;
            }
            let mut field_used = false;
            let mut use_bounds = use_bounds;
            exprs.push(build_eq_expr(
                source.kind(),
                field,
                &mut field_used,
                &mut use_bounds,
                wcb,
            )?);
            use_bounds = field
                .hattrs
                .push_bounds_to_without_helper(use_bounds, kind, wcb);
            if use_bounds && field_used {
                wcb.push_bounds_for_field(field.field);
            }
        }
        Ok(quote!(#(#exprs)*))
    };
    match source {
        ItemSource::Struct { fields, .. } => build_from_fields(fields, use_bounds, wcb),
        ItemSource::Enum { variants, .. } => {
            let mut arms = Vec::new();
            for variant in variants {
                let use_bounds = variant.hattrs.push_bounds_to(use_bounds, kind, wcb);
                let body = build_from_fields(&variant.fields, use_bounds, wcb)?;
                let pat_this = variant.make_pat_with_self_path("_this", source.ident());
                arms.push(quote!(#pat_this => { #body }));
            }
            Ok(quote! {
                match this {
                    #(#arms)*
                    _ => { }
                }
            })
        }
    }
}
fn build_eq_expr(
    source: ItemSourceKind,
    field: &FieldEntry,
    field_used: &mut bool,
    use_bounds: &mut bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Eq;
    let cmp = &field.hattrs.cmp;
    let this = source.this_of(field);

    cmp.eq.push_bounds_to(use_bounds, wcb);
    if cmp.eq.by.is_some() {
        return Ok(quote!());
    }
    if let Some(key) = &cmp.eq.key {
        return Ok(key.build_eq_checker(this));
    }

    cmp.ord.push_bounds_to(use_bounds, wcb);
    if cmp.ord.by.is_some() {
        return Ok(quote!());
    }
    if let Some(key) = &cmp.ord.key {
        return Ok(key.build_eq_checker(this));
    }

    if let Some(bad) = cmp.bad_attr() {
        return bad_attr(
            op,
            bad,
            &[
                "eq(key = ...)",
                "eq(by = ...)",
                "ord(key = ...)",
                "ord(by = ...)",
            ],
        );
    }

    *field_used = true;
    Ok(build_eq_checker(this))
}

fn build_partial_ord_body(
    source: ItemSource,
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::PartialOrd;
    let kind = DeriveItemKind::CompareOp(op);

    let build_from_fields = |fields: &[FieldEntry],
                             use_bounds: bool,
                             wcb: &mut WhereClauseBuilder|
     -> Result<TokenStream> {
        let mut body = TokenStream::new();
        for field in fields {
            if field.hattrs.cmp.is_ignore(op)? {
                continue;
            }
            let mut field_used = false;
            let mut use_bounds = use_bounds;
            let mut expr = build_partial_ord_expr(
                source.kind(),
                field,
                &mut field_used,
                &mut use_bounds,
                wcb,
            )?;
            if field.hattrs.cmp.is_reverse(op)? {
                expr = quote!(::core::option::Option::map(#expr, ::core::cmp::Ordering::reverse));
            }
            body.extend(quote! {
                match #expr {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {}
                    o => return o,
                }
            });
            use_bounds = field
                .hattrs
                .push_bounds_to_without_helper(use_bounds, kind, wcb);
            if use_bounds && field_used {
                wcb.push_bounds_for_field(field.field);
            }
        }
        Ok(quote! {
            #body
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        })
    };
    let body = match source {
        ItemSource::Struct { fields, .. } => build_from_fields(fields, use_bounds, wcb)?,
        ItemSource::Enum { variants, .. } => {
            let mut arms = Vec::new();
            for variant in variants {
                let use_bounds = variant.hattrs.push_bounds_to(use_bounds, kind, wcb);
                let body = build_from_fields(&variant.fields, use_bounds, wcb)?;
                let pat_this = variant.make_pat("_self");
                let pat_other = variant.make_pat("_other");
                arms.push(quote!((#pat_this, #pat_other) => { #body }));
            }
            let to_index_fn = build_to_index_fn(variants);
            quote! {
                match (self, other) {
                    #(#arms)*
                    (this, otehr) => {
                        #to_index_fn
                        ::core::cmp::PartialOrd::partial_cmp(&to_index(this), &to_index(other))
                    },
                }
            }
        }
    };
    Ok(quote! {
        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
            #body
        }
    })
}
fn build_partial_ord_expr(
    source: ItemSourceKind,
    field: &FieldEntry,
    field_used: &mut bool,
    use_bounds: &mut bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::PartialOrd;
    let ty = &field.field.ty;
    let fn_ident = field.make_ident("__partial_ord_");
    let span = field.field.span();
    let this = source.self_of(field);
    let other = source.other_of(field);
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

    if let Some(bad) = cmp.bad_attr() {
        return bad_attr(
            op,
            bad,
            &[
                "partial_ord(key = ...)",
                "partial_ord(by = ...)",
                "ord(key = ...)",
                "ord(by = ...)",
            ],
        );
    }

    *field_used = true;
    Ok(quote_spanned!(span=> ::core::cmp::PartialOrd::partial_cmp(&(#this), &(#other))))
}

fn build_ord_body(
    source: ItemSource,
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Ord;
    let kind = DeriveItemKind::CompareOp(op);
    let build_from_fields = |fields: &[FieldEntry],
                             use_bounds: bool,
                             wcb: &mut WhereClauseBuilder|
     -> Result<TokenStream> {
        let mut body = TokenStream::new();
        for field in fields {
            if field.hattrs.cmp.is_ignore(op)? {
                continue;
            }
            let mut field_used = false;
            let mut use_bounds = use_bounds;
            let mut expr =
                build_ord_expr(source.kind(), field, &mut field_used, &mut use_bounds, wcb)?;
            if field.hattrs.cmp.is_reverse(op)? {
                expr = quote!(::core::cmp::Ordering::reverse(#expr));
            }
            body.extend(quote! {
                match #expr {
                    ::core::cmp::Ordering::Equal => {}
                    o => return o,
                }
            });
            use_bounds = field
                .hattrs
                .push_bounds_to_without_helper(use_bounds, kind, wcb);
            if use_bounds && field_used {
                wcb.push_bounds_for_field(field.field);
            }
        }
        Ok(quote! {
            #body
            ::core::cmp::Ordering::Equal
        })
    };

    let body = match source {
        ItemSource::Struct { fields, .. } => build_from_fields(fields, use_bounds, wcb)?,
        ItemSource::Enum { variants, .. } => {
            let mut arms = Vec::new();
            for variant in variants {
                let use_bounds = variant.hattrs.push_bounds_to(use_bounds, kind, wcb);
                let body = build_from_fields(&variant.fields, use_bounds, wcb)?;
                let pat_this = variant.make_pat("_self");
                let pat_other = variant.make_pat("_other");
                arms.push(quote!((#pat_this, #pat_other) => { #body }));
            }
            let to_index_fn = build_to_index_fn(variants);
            quote! {
                match (self, other) {
                    #(#arms)*
                    (this, otehr) => {
                        #to_index_fn
                        ::core::cmp::Ord::cmp(&to_index(this), &to_index(other))
                    },
                }
            }
        }
    };
    Ok(quote! {
        fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
            #body
        }
    })
}
fn build_ord_expr(
    source: ItemSourceKind,
    field: &FieldEntry,
    field_used: &mut bool,
    use_bounds: &mut bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Ord;
    let ty = &field.field.ty;
    let fn_ident = field.make_ident("__ord_");
    let span = field.field.span();
    let this = source.self_of(field);
    let other = source.other_of(field);
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

    if let Some(bad) = cmp.bad_attr() {
        return bad_attr(op, bad, &["ord(key = ...)", "ord(by = ...)"]);
    }

    *field_used = true;
    Ok(quote_spanned!(span=> ::core::cmp::Ord::cmp(&(#this), &(#other))))
}

fn build_hash_body(
    source: ItemSource,
    use_bounds: bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Hash;
    let kind = DeriveItemKind::CompareOp(op);
    let build_from_fields = |fields: &[FieldEntry],
                             use_bounds: bool,
                             wcb: &mut WhereClauseBuilder|
     -> Result<TokenStream> {
        let mut exprs = Vec::new();
        for field in fields {
            if field.hattrs.cmp.is_ignore(op)? {
                continue;
            }
            let mut field_used = false;
            let mut use_bounds = use_bounds;

            exprs.push(build_hash_expr(
                source.kind(),
                field,
                &mut field_used,
                &mut use_bounds,
                wcb,
            )?);
            use_bounds = field
                .hattrs
                .push_bounds_to_without_helper(use_bounds, kind, wcb);
            if use_bounds && field_used {
                wcb.push_bounds_for_field(field.field);
            }
        }
        Ok(quote!(#(#exprs)*))
    };
    let body = match source {
        ItemSource::Struct { fields, .. } => build_from_fields(fields, use_bounds, wcb)?,
        ItemSource::Enum { variants, .. } => {
            let mut arms = Vec::new();
            for variant in variants {
                let use_bounds = variant.hattrs.push_bounds_to(use_bounds, kind, wcb);
                let body = build_from_fields(&variant.fields, use_bounds, wcb)?;
                let pat_self = variant.make_pat("_self");
                arms.push(quote!(#pat_self => { #body }));
            }
            quote! {
                match self {
                    #(#arms)*
                    _ => unreachable!(),
                }
            }
        }
    };
    Ok(quote! {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            #body
        }
    })
}
fn build_hash_expr(
    source: ItemSourceKind,
    field: &FieldEntry,
    field_used: &mut bool,
    use_bounds: &mut bool,
    wcb: &mut WhereClauseBuilder,
) -> Result<TokenStream> {
    let op = CompareOp::Hash;
    let ty = &field.field.ty;
    let fn_ident = field.make_ident("__hash_");
    let span = field.field.span();
    let this = source.self_of(field);
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
    if let Some(key) = &cmp.eq.key {
        return Ok(key.build_hash_stmt(this));
    }

    cmp.ord.push_bounds_to(use_bounds, wcb);
    if let Some(key) = &cmp.ord.key {
        return Ok(key.build_hash_stmt(this));
    }

    if let Some(bad) = cmp.bad_attr() {
        return bad_attr(
            op,
            bad,
            &[
                "hash(key = ...)",
                "hash(by = ...)",
                "eq(key = ...)",
                "ord(key = ...)",
            ],
        );
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
    pub fn push_bounds(&self, op: CompareOp, wcb: &mut WhereClauseBuilder) -> bool {
        let mut use_bounds = true;
        for &source in CompareOp::VARIANTS {
            if source.is_effects_to(op) && use_bounds {
                use_bounds = wcb.push_bounds(&self.get(source).bounds);
            }
        }
        use_bounds
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
                bad_attr_1(
                    span,
                    op,
                    &format!("{bad}(ignore)"),
                    &format!("{good}(ignore)"),
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
                    return bad_attr_1(span, op, "partial_ord(reverse)", "ord(reverse)");
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
    fn bad_attr(&self) -> Option<(String, Span)> {
        for &op in CompareOp::VARIANTS {
            if let Some((bad, span)) = self.get(op).bad_attr() {
                let op = op.to_str_snake_case();
                return Some((format!("{op}({bad})"), span));
            }
        }
        None
    }

    pub(crate) fn verify(&self, target: AttributeTarget) -> Result<()> {
        for &op in CompareOp::VARIANTS {
            self.get(op).verify(target)?;
        }
        Ok(())
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
    fn bad_attr(&self) -> Option<(&str, Span)> {
        if let Some(key) = &self.key {
            return Some(("key = ...", key.span()));
        }
        if let Some(by) = &self.by {
            return Some(("by = ...", by.span()));
        }
        None
    }

    fn verify(&self, target: AttributeTarget) -> Result<()> {
        match target {
            AttributeTarget::Type => {
                if let Some(by) = &self.by {
                    bail!(by.span(), "cannot specify `by = ...` for type");
                }
                if let Some(key) = &self.key {
                    bail!(key.span(), "cannot specify `key = ...` for type");
                }
                if let Some(span) = self.reverse.span {
                    bail!(span, "cannot specify `reverse` for type");
                }
                if let Some(span) = self.ignore.span {
                    bail!(span, "cannot specify `ignore` for type");
                }
            }
            AttributeTarget::Variant => {
                if let Some(by) = &self.by {
                    bail!(by.span(), "cannot specify `by = ...` for enum variants");
                }
                if let Some(key) = &self.key {
                    bail!(key.span(), "cannot specify `key = ...` for enum variants");
                }
                if let Some(span) = self.reverse.span {
                    bail!(span, "cannot specify `reverse` for enum variants");
                }
                if let Some(span) = self.ignore.span {
                    bail!(span, "cannot specify `ignore` for enum variants");
                }
            }
            AttributeTarget::Field => {}
        }
        Ok(())
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
fn build_to_index_fn(variants: &[VariantEntry]) -> TokenStream {
    let mut arms = Vec::new();
    for (index, variant) in variants.iter().enumerate() {
        let pat = variant.make_pat_wildcard();
        arms.push(quote!((#pat) => #index,));
    }
    quote! {
        let to_index = |this: &Self| -> usize {
            match this {
                #(#arms)*
                _ => unreachable!(),
            }
        };
    }
}

fn build_eq_checker(this: TokenStream) -> TokenStream {
    quote_spanned!(this.span()=>{
        fn _eq<T: Eq + ?Sized>(_this: &T) { }
        _eq(&(#this))
    })
}

fn bad_attr<T>(op: CompareOp, (bad, span): (String, Span), good: &[&str]) -> Result<T> {
    let mut msg = format!(
        "Since `#[{bad}]` was specified, the default implementation of `{op}` cannot be used.
One of the following attributes is required.\n\n"
    );
    for good in good {
        msg.push_str(&format!("#[{good}]\n"));
    }
    bail!(span, "{msg}")
}

fn bad_attr_1<T>(span: Span, op: CompareOp, bad: &str, good: &str) -> Result<T> {
    bail!(
        span,
        "When `#[derive_ex({op})]` is specified, `#[{good}]` must be used instead of `#[{bad}]`."
    )
}
