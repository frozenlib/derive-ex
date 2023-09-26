#![allow(clippy::large_enum_variant)]

extern crate proc_macro;

#[macro_use]
mod syn_utils;

mod bound;
mod common;
mod item_impl;
mod item_type;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Item, Result};

#[doc = include_str!("../../doc/derive_ex.md")]
#[proc_macro_attribute]
pub fn derive_ex(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item: TokenStream = item.into();
    match build(attr.into(), item.clone()) {
        Ok(s) => s,
        Err(e) => {
            item.extend(e.to_compile_error());
            item
        }
    }
    .into()
}

fn build(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let mut item: Item = parse2(item)?;
    let ts = match &mut item {
        Item::Struct(item_struct) => item_type::build_by_item_struct(attr, item_struct),
        Item::Enum(item_enum) => item_type::build_by_item_enum(attr, item_enum),
        Item::Impl(item_impl) => item_impl::build_by_item_impl(attr, item_impl),
        _ => bail!(
            _,
            "`#[derive_ex]` can be specified only for `struct`, `enum`, or `impl`.",
        ),
    }
    .unwrap_or_else(|e| e.to_compile_error());

    Ok(quote!(#item #ts))
}
