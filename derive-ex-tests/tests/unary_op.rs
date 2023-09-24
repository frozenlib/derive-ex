use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn neg_unit_struct() {
    #[derive_ex(Neg)]
    struct X;

    let _ = -X;
    let _ = -&X;
}

#[test]
fn neg_tuple_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Neg)]
    struct X(i32, i8);

    assert_eq!(-X(8, 20), X(-8, -20));
    assert_eq!(-&X(8, 20), X(-8, -20));
}

#[test]
fn neg_record_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Neg)]
    struct X {
        a: i32,
        b: i8,
    }
    assert_eq!(-X { a: 8, b: 20 }, X { a: -8, b: -20 });
    assert_eq!(-&X { a: 8, b: 20 }, X { a: -8, b: -20 });
}

#[test]
fn neg_generics() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Neg)]
    struct X<T>(T, T);

    assert_eq!(-X(8, 20), X(-8, -20));
    assert_eq!(-&X(8, 20), X(-8, -20));
}

#[test]
fn neg_generics_contains_self() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Neg)]
    struct X<T>(T)
    where
        Self: MyTrait;

    impl MyTrait for X<i32> {}

    assert_eq!(-X(10i32), X(-10i32));
}

#[test]
fn neg_bound_struct_trait() {
    use std::ops::Neg;
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Neg(bound(T : MyTrait, ..)))]
    struct X<T>(T);

    impl MyTrait for i32 {}

    assert_impl!(Neg, X<i32>, &X<i32>);
    assert_impl!(!Neg, X<i8>, &X<i8>);
}

#[test]
fn neg_bound_struct_common() {
    use std::ops::Neg;
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Neg, bound(T : MyTrait, ..))]
    struct X<T>(T);

    impl MyTrait for i32 {}

    assert_impl!(Neg, X<i32>, &X<i32>);
    assert_impl!(!Neg, X<i8>, &X<i8>);
}
#[test]
fn neg_bound_field_trait() {
    use std::ops::Neg;
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Neg)]
    struct X<T>(#[derive_ex(Neg(bound(T : MyTrait, ..)))] T);

    impl MyTrait for i32 {}

    assert_impl!(Neg, X<i32>, &X<i32>);
    assert_impl!(!Neg, X<i8>, &X<i8>);
}

#[test]
fn neg_bound_type() {
    use std::ops::Neg;

    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Neg)]
    struct Inner<T>(T);

    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Neg(bound(T)))]
    struct X<T>(Inner<T>);

    assert_impl!(Neg, X<i32>, &X<i32>);
}

#[test]
fn not_unit_struct() {
    #[derive_ex(Not)]
    struct X;

    let _ = !X;
    let _ = !&X;
}

#[test]
fn not_tuple_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Not)]
    struct X(bool, bool);

    assert_eq!(!X(true, false), X(false, true));
    assert_eq!(!&X(true, false), X(false, true));
}

#[test]
fn not_record_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Not)]
    struct X {
        a: bool,
        b: bool,
    }
    assert_eq!(!X { a: true, b: false }, X { a: false, b: true });
    assert_eq!(!&X { a: true, b: false }, X { a: false, b: true });
}
