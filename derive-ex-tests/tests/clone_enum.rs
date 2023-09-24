#![allow(clippy::redundant_clone)]

use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn enum_unit() {
    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Clone)]
    enum X {
        A,
        B,
    }
    assert_eq!(X::A.clone(), X::A);
    assert_eq!(X::B.clone(), X::B);
}

#[test]
fn enum_tuple() {
    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Clone)]
    enum X {
        A(u32, u32),
        B(String),
    }
    assert_eq!(X::A(10, 20).clone(), X::A(10, 20));
    assert_eq!(X::B("abc".into()).clone(), X::B("abc".into()));
}

#[test]
fn enum_record() {
    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Clone)]
    enum X {
        A { a: u32, b: String },
    }
    assert_eq!(
        X::A {
            a: 10,
            b: "abc".into()
        }
        .clone(),
        X::A {
            a: 10,
            b: "abc".into()
        }
    );
}

#[test]
fn generics() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    enum X<T1, T2> {
        A(T1, T2),
        B { a: T1, b: T2 },
    }

    assert_eq!(X::A(10, 20).clone(), X::A(10, 20));
    assert_eq!(X::B { a: 10, b: 20 }.clone(), X::B { a: 10, b: 20 });
}

#[test]
fn generics_contains_self() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    enum X<T1, T2>
    where
        Self: MyTrait,
    {
        A(T1, T2),
        B { a: T1, b: T2 },
    }
    impl MyTrait for X<u32, u32> {}

    assert_eq!(X::A(10, 20).clone(), X::A(10, 20));
    assert_eq!(X::B { a: 10, b: 20 }.clone(), X::B { a: 10, b: 20 });
}

#[test]
#[allow(dead_code)]
fn bound_enum_trait() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone(bound(T1 : MyTrait, ..)))]
    enum X<T1, T2> {
        A(T1, T2),
        B { a: T1, b: T2 },
    }
    impl MyTrait for u32 {}

    assert_impl!(Clone, X<u32, u32>);
    assert_impl!(!Clone, X<u8, u8>);
}
#[test]
#[allow(dead_code)]
fn bound_enum_common() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone, bound(T1 : MyTrait, ..))]
    enum X<T1, T2> {
        A(T1, T2),
        B { a: T1, b: T2 },
    }
    impl MyTrait for u32 {}

    assert_impl!(Clone, X<u32, u32>);
    assert_impl!(!Clone, X<u8, u8>);
}

#[test]
#[allow(dead_code)]
fn bound_variant_trait() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    enum X<T1, T2> {
        #[derive_ex(Clone(bound(T1 : MyTrait, ..)))]
        A(T1, T2),
        B {
            a: T1,
            b: T2,
        },
    }
    impl MyTrait for u32 {}

    assert_impl!(Clone, X<u32, u32>);
    assert_impl!(!Clone, X<u8, u8>);
}

#[test]
#[allow(dead_code)]
fn bound_variant_common() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    enum X<T1, T2> {
        #[derive_ex(Clone, bound(T1 : MyTrait, ..))]
        A(T1, T2),
        B {
            a: T1,
            b: T2,
        },
    }
    impl MyTrait for u32 {}

    assert_impl!(Clone, X<u32, u32>);
    assert_impl!(!Clone, X<u8, u8>);
}

#[test]
#[allow(dead_code)]
fn bound_field_trait() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    enum X<T1, T2> {
        A(#[derive_ex(Clone(bound(T1 : MyTrait, ..)))] T1, T2),
        B { a: T1, b: T2 },
    }
    impl MyTrait for u32 {}

    assert_impl!(Clone, X<u32, u32>);
    assert_impl!(!Clone, X<u8, u8>);
}

#[test]
#[allow(dead_code)]
fn bound_field_common() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Clone)]
    enum X<T1, T2> {
        A(#[derive_ex(Clone, bound(T1 : MyTrait, ..))] T1, T2),
        B { a: T1, b: T2 },
    }
    impl MyTrait for u32 {}

    assert_impl!(Clone, X<u32, u32>);
    assert_impl!(!Clone, X<u8, u8>);
}

#[test]
fn bound_type() {
    struct NoClone;

    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Clone(bound(T)))]
    struct X<T>(std::marker::PhantomData<T>);

    assert_impl!(Clone, X<u32>);
    assert_impl!(!Clone, X<NoClone>);
}

#[test]
fn bound_none() {
    struct NoClone;

    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Clone(bound()))]
    struct X<T>(std::rc::Rc<T>);

    assert_impl!(Clone, X<NoClone>);
}
