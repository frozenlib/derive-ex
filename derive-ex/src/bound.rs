use proc_macro2::TokenStream;
use quote::quote;
use structmeta::{NameArgs, ToTokens};
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    Field, Generics, Result, Token, Type, WherePredicate,
};

use crate::syn_utils::GenericParamSet;

#[derive(Clone, ToTokens, Debug)]
pub enum Bound {
    Type(Type),
    Pred(WherePredicate),
    Default(Token![..]),
}

impl Parse for Bound {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![..]) {
            return Ok(Self::Default(input.parse()?));
        }
        let fork = input.fork();
        match fork.parse() {
            Ok(p) => {
                input.advance_to(&fork);
                Ok(Self::Pred(p))
            }
            Err(e) => {
                if let Ok(ty) = input.parse() {
                    Ok(Self::Type(ty))
                } else {
                    Err(e)
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct Bounds {
    pub ty: Vec<Type>,
    pub pred: Vec<WherePredicate>,
    pub default: bool,
}
impl Default for Bounds {
    fn default() -> Self {
        Self::new()
    }
}
impl Bounds {
    pub const fn new() -> Self {
        Self {
            ty: Vec::new(),
            pred: Vec::new(),
            default: true,
        }
    }

    pub fn from(bound: &Option<NameArgs<Vec<Bound>>>) -> Self {
        let mut this = Self::new();
        if let Some(bound) = bound {
            this.default = false;
            for b in &bound.args {
                this.push(b.clone());
            }
        }
        this
    }
    fn push(&mut self, bound: Bound) {
        match bound {
            Bound::Type(ty) => self.ty.push(ty),
            Bound::Pred(pred) => self.pred.push(pred),
            Bound::Default(_) => self.default = true,
        }
    }
}

pub struct WhereClauseBuilder {
    types: Vec<Type>,
    preds: Vec<WherePredicate>,
    gps: GenericParamSet,
}

impl WhereClauseBuilder {
    pub fn new(generics: &Generics) -> Self {
        let (_, _, where_g) = generics.split_for_impl();
        let mut preds = Vec::new();
        if let Some(where_g) = where_g {
            preds.extend(where_g.predicates.iter().cloned());
        }
        Self {
            types: Vec::new(),
            preds,
            gps: GenericParamSet::new(generics),
        }
    }

    pub fn push_bounds(&mut self, bounds: &Bounds) -> bool {
        self.preds.extend(bounds.pred.iter().cloned());
        self.types.extend(bounds.ty.iter().cloned());
        bounds.default
    }
    pub fn push_bounds_for_field(&mut self, field: &Field) {
        if self.gps.contains_in_type(&field.ty) {
            self.types.push(field.ty.clone());
        }
    }

    pub fn build(self, f: impl Fn(&Type) -> TokenStream) -> TokenStream {
        let mut ws = Vec::new();
        for ty in &self.types {
            ws.push(f(ty));
        }
        for p in self.preds {
            ws.push(quote!(#p));
        }
        if ws.is_empty() {
            quote!()
        } else {
            quote!(where #(#ws,)*)
        }
    }
}
