#![allow(clippy::redundant_clone)]

#[macro_use]
mod test_utils;

use derive_ex::derive_ex;

#[test]
fn unit_struct() {
    #[derive_ex(Clone)]
    struct X;
}

#[test]

fn tuple_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    struct X(String, u32);

    assert_eq!(X("abc".into(), 15).clone(), X("abc".into(), 15));
}

#[test]

fn record_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    struct X {
        a: String,
        b: u32,
    }

    assert_eq!(
        X {
            a: "abc".into(),
            b: 15
        }
        .clone(),
        X {
            a: "abc".into(),
            b: 15
        }
    );
}

#[test]
fn generics() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    struct X<T>(T, T);

    assert_eq!(X(10, 20).clone(), X(10, 20));
}

#[test]
fn generics_contains_self() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    struct X<T>(T)
    where
        Self: MyTrait;

    impl MyTrait for X<u32> {}

    assert_eq!(X(10u32).clone(), X(10u32));
}

#[test]
fn bound_struct_trait() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone(bound(T : MyTrait, ..)))]
    struct X<T>(T);

    impl MyTrait for u32 {}

    assert_impl!(Clone, X<u32>);
    assert_impl!(!Clone, X<u8>);
}

#[test]
fn bound_struct_common() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone, bound(T : MyTrait, ..))]
    struct X<T>(T);

    impl MyTrait for u32 {}

    assert_impl!(Clone, X<u32>);
    assert_impl!(!Clone, X<u8>);
}
#[test]
fn bound_field_trait() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    struct X<T>(#[derive_ex(Clone(bound(T : MyTrait, ..)))] T);

    impl MyTrait for u32 {}

    assert_impl!(Clone, X<u32>);
    assert_impl!(!Clone, X<u8>);
}

#[test]
fn bound_type() {
    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Clone)]
    struct Inner<T>(T);

    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Clone(bound(T)))]
    struct X<T>(Inner<T>);

    assert_impl!(Clone, X<u32>);
}

#[test]
fn bound_field_common() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    struct X<T>(#[derive_ex(Clone, bound(T : MyTrait, ..))] T);

    impl MyTrait for u32 {}

    assert_impl!(Clone, X<u32>);
    assert_impl!(!Clone, X<u8>);
}

#[test]
fn clone_from() {
    struct Inner {
        by_clone_from: bool,
    }
    impl Clone for Inner {
        fn clone(&self) -> Self {
            Self {
                by_clone_from: false,
            }
        }

        fn clone_from(&mut self, _source: &Self) {
            self.by_clone_from = true;
        }
    }

    #[derive_ex(Clone)]
    struct X(Inner);

    impl X {
        fn new() -> Self {
            Self(Inner {
                by_clone_from: false,
            })
        }
    }

    let mut x = X::new();
    x.clone_from(&X::new());
    assert!(x.0.by_clone_from);
}
