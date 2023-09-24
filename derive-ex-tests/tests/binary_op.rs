use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn add_unit_struct() {
    use std::ops::Add;
    #[derive_ex(Add)]
    struct X;

    let _ = X + X;
    let _ = &X + X;
    let _ = X + &X;
    let _ = &X + &X;

    assert_impl!(Add, X, &X);
    assert_impl!(for<'a> Add<&'a X>, X, &X);
}

#[test]
fn add_tuple_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Add)]
    struct X(u32, u8);

    assert_eq!(X(10, 20) + X(7, 8), X(17, 28));
    assert_eq!(&X(10, 20) + X(7, 8), X(17, 28));
    assert_eq!(X(10, 20) + &X(7, 8), X(17, 28));
    assert_eq!(&X(10, 20) + &X(7, 8), X(17, 28));
}

#[test]
fn add_record_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Add)]
    struct X {
        a: u32,
        b: u8,
    }

    assert_eq!(X { a: 10, b: 20 } + X { a: 7, b: 8 }, X { a: 17, b: 28 });
    assert_eq!(&X { a: 10, b: 20 } + X { a: 7, b: 8 }, X { a: 17, b: 28 });
    assert_eq!(X { a: 10, b: 20 } + &X { a: 7, b: 8 }, X { a: 17, b: 28 });
    assert_eq!(&X { a: 10, b: 20 } + &X { a: 7, b: 8 }, X { a: 17, b: 28 });
}

#[test]
fn add_generics() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Add)]
    struct X<T>(T, T);

    assert_eq!(X(10, 20) + X(7, 8), X(17, 28));
    assert_eq!(&X(10, 20) + X(7, 8), X(17, 28));
    assert_eq!(X(10, 20) + &X(7, 8), X(17, 28));
    assert_eq!(&X(10, 20) + &X(7, 8), X(17, 28));
}

#[test]
fn add_generics_contains_self() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Add)]
    struct X<T>(T)
    where
        Self: MyTrait;

    impl MyTrait for X<u32> {}

    assert_eq!(X(10u32) + X(7u32), X(17u32));
}

#[test]
fn add_bound_struct_trait() {
    use std::ops::Add;
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Add(bound(T : MyTrait, ..)))]
    struct X<T>(T);

    impl MyTrait for u32 {}

    assert_impl!(Add, X<u32>);
    assert_impl!(!Add, X<u8>);
}

#[test]
fn add_bound_struct_common() {
    use std::ops::Add;
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Add, bound(T : MyTrait, ..))]
    struct X<T>(T);

    impl MyTrait for u32 {}

    assert_impl!(Add, X<u32>);
    assert_impl!(!Add, X<u8>);
}

#[test]
fn add_bound_field_trait() {
    use std::ops::Add;
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Add)]
    struct X<T>(#[derive_ex(Add(bound(T : MyTrait, ..)))] T);

    impl MyTrait for u32 {}

    assert_impl!(Add, X<u32>);
    assert_impl!(!Add, X<u8>);
}

#[test]
fn add_bound_type() {
    use std::ops::Add;

    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Add)]
    struct Inner<T>(T);

    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Add(bound(T)))]
    struct X<T>(Inner<T>);

    assert_impl!(Add, X<u32>, &X<u32>);
    assert_impl!(for<'a> Add<&'a X<u32>>, X<u32>, &X<u32>);
}

#[test]
fn add_bound_field_common() {
    use std::ops::Add;
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Add)]
    struct X<T>(#[derive_ex(Add, bound(T : MyTrait, ..))] T);

    impl MyTrait for u32 {}

    assert_impl!(Add, X<u32>);
    assert_impl!(!Add, X<u8>);
}

#[test]
fn sub_unit_struct() {
    #[derive_ex(Sub)]
    struct X;

    let _ = X - X;
    let _ = &X - X;
    let _ = X - &X;
    let _ = &X - &X;
}

#[test]
fn sub_tuple_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Sub)]
    struct X(u32, u8);

    assert_eq!(X(10, 20) - X(7, 8), X(3, 12));
    assert_eq!(&X(10, 20) - X(7, 8), X(3, 12));
    assert_eq!(X(10, 20) - &X(7, 8), X(3, 12));
    assert_eq!(&X(10, 20) - &X(7, 8), X(3, 12));
}

#[test]
fn sub_record_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Sub)]
    struct X {
        a: u32,
        b: u8,
    }

    assert_eq!(X { a: 10, b: 20 } - X { a: 7, b: 8 }, X { a: 3, b: 12 });
    assert_eq!(&X { a: 10, b: 20 } - X { a: 7, b: 8 }, X { a: 3, b: 12 });
    assert_eq!(X { a: 10, b: 20 } - &X { a: 7, b: 8 }, X { a: 3, b: 12 });
    assert_eq!(&X { a: 10, b: 20 } - &X { a: 7, b: 8 }, X { a: 3, b: 12 });
}

#[test]
fn derive_other_1() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Add)]
    struct X;
    assert_eq!(X + X, X);
}

#[test]
fn derive_other_2() {
    #[derive_ex(Add)]
    #[derive(Eq, PartialEq, Debug)]
    struct X;
    assert_eq!(X + X, X);
}
