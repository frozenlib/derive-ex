use std::collections::HashSet;
use syn::{
    ext::IdentExt,
    parse_quote,
    visit::{visit_path, Visit},
    visit_mut::{visit_type_mut, VisitMut},
    GenericParam, Generics, Ident, Type,
};

macro_rules! bail {
    (_, $($arg:tt)*) => {
        bail!(proc_macro2::Span::call_site(), $($arg)*)
    };
    ($span:expr, $fmt:expr $(,)?) => {
        return std::result::Result::Err(syn::Error::new($span, std::format!($fmt)))
    };
    ($span:expr, $fmt:expr, $($arg:tt)*) => {
        return std::result::Result::Err(syn::Error::new($span, std::format!($fmt, $($arg)*)))
    };
}

pub trait VisitableMut {
    fn visit_mut(&mut self, visit: &mut impl VisitMut);
}
impl VisitableMut for Type {
    fn visit_mut(&mut self, visit: &mut impl VisitMut) {
        visit.visit_type_mut(self);
    }
}
impl VisitableMut for Generics {
    fn visit_mut(&mut self, visit: &mut impl VisitMut) {
        visit.visit_generics_mut(self);
    }
}
pub fn expand_self<T: VisitableMut + Clone>(input: &T, to: &Type) -> T {
    struct ExpandSelfVisitor<'a> {
        to: &'a Type,
    }
    impl<'a> VisitMut for ExpandSelfVisitor<'a> {
        fn visit_type_mut(&mut self, i: &mut Type) {
            let tself: Type = parse_quote!(Self);
            if i == &tself {
                *i = self.to.clone();
            } else {
                visit_type_mut(self, i);
            }
        }
    }
    let mut input = input.clone();
    input.visit_mut(&mut ExpandSelfVisitor { to });
    input
}

pub struct GenericParamSet {
    idents: HashSet<Ident>,
}

impl GenericParamSet {
    pub fn new(generics: &Generics) -> Self {
        let mut idents = HashSet::new();
        for p in &generics.params {
            match p {
                GenericParam::Type(t) => {
                    idents.insert(t.ident.unraw());
                }
                GenericParam::Const(t) => {
                    idents.insert(t.ident.unraw());
                }
                _ => {}
            }
        }
        Self { idents }
    }
    fn contains(&self, ident: &Ident) -> bool {
        self.idents.contains(&ident.unraw())
    }

    pub fn contains_in_type(&self, ty: &Type) -> bool {
        struct Visitor<'a> {
            generics: &'a GenericParamSet,
            result: bool,
        }
        impl<'a, 'ast> Visit<'ast> for Visitor<'a> {
            fn visit_path(&mut self, i: &'ast syn::Path) {
                if i.leading_colon.is_none() {
                    if let Some(s) = i.segments.iter().next() {
                        if self.generics.contains(&s.ident) {
                            self.result = true;
                        }
                    }
                }
                visit_path(self, i);
            }
        }
        let mut visitor = Visitor {
            generics: self,
            result: false,
        };
        visitor.visit_type(ty);
        visitor.result
    }
}
