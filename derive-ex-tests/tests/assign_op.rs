use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn add_assign_unit_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(AddAssign)]
    struct X;

    let mut x = X;
    x += X;
    x += &X;
}

#[test]
fn add_assign_tuple_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(AddAssign)]
    struct X(u32, u8);

    let mut x = X(10, 20);
    x += X(7, 8);
    assert_eq!(x, X(17, 28));

    let mut x = X(10, 20);
    x += &X(7, 8);
    assert_eq!(x, X(17, 28));
}

#[test]
fn add_assign_record_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(AddAssign)]
    struct X {
        a: u32,
        b: u8,
    }

    let mut x = X { a: 10, b: 20 };
    x += X { a: 7, b: 8 };
    assert_eq!(x, X { a: 17, b: 28 });

    let mut x = X { a: 10, b: 20 };
    x += &X { a: 7, b: 8 };
    assert_eq!(x, X { a: 17, b: 28 });
}

#[test]
fn add_assign_generics() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(AddAssign)]
    struct X<T>(T, T);

    let mut x = X(10, 20);
    x += X(7, 8);
    assert_eq!(x, X(17, 28));

    let mut x = X(10, 20);
    x += &X(7, 8);
    assert_eq!(x, X(17, 28));
}

#[test]
fn add_assign_generics_contains_self() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(AddAssign)]
    struct X<T>(T)
    where
        Self: MyTrait;

    impl MyTrait for X<u32> {}

    let mut x = X(10u32);
    x += X(7u32);
    assert_eq!(x, X(17u32));
}

#[test]
fn add_assign_bound_struct_trait() {
    use std::ops::AddAssign;
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(AddAssign(bound(T : MyTrait, ..)))]
    struct X<T>(T);

    impl MyTrait for u32 {}

    assert_impl!(AddAssign, X<u32>);
    assert_impl!(for<'a> AddAssign<&'a X<u32>>, X<u32>);
    assert_impl!(!AddAssign, X<u8>);
}

#[test]
fn add_assign_bound_struct_common() {
    use std::ops::AddAssign;
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(AddAssign, bound(T : MyTrait, ..))]
    struct X<T>(T);

    impl MyTrait for u32 {}

    assert_impl!(AddAssign, X<u32>);
    assert_impl!(for<'a> AddAssign<&'a X<u32>>, X<u32>);
    assert_impl!(!AddAssign, X<u8>);
}

#[test]
fn add_assign_bound_field_trait() {
    use std::ops::AddAssign;
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(AddAssign)]
    struct X<T>(#[derive_ex(AddAssign(bound(T : MyTrait, ..)))] T);

    impl MyTrait for u32 {}

    assert_impl!(AddAssign, X<u32>);
    assert_impl!(for<'a> AddAssign<&'a X<u32>>, X<u32>);
    assert_impl!(!AddAssign, X<u8>);
}

#[test]
fn add_assign_bound_type() {
    use std::ops::AddAssign;

    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(AddAssign)]
    struct Inner<T>(T);

    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(AddAssign(bound(T)))]
    struct X<T>(Inner<T>);

    assert_impl!(AddAssign, X<u32>);
    assert_impl!(for<'a> AddAssign<&'a X<u32>>, X<u32>);
}

#[test]
fn sub_assign_unit_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(SubAssign)]
    struct X;

    let mut x = X;
    x -= X;
    x -= &X;
}

#[test]
fn sub_assign_tuple_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(SubAssign)]
    struct X(u32, u8);

    let mut x = X(10, 20);
    x -= X(7, 8);
    assert_eq!(x, X(3, 12));

    let mut x = X(10, 20);
    x -= &X(7, 8);
    assert_eq!(x, X(3, 12));
}

#[test]
fn sub_assign_record_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(SubAssign)]
    struct X {
        a: u32,
        b: u8,
    }

    let mut x = X { a: 10, b: 20 };
    x -= X { a: 7, b: 8 };
    assert_eq!(x, X { a: 3, b: 12 });

    let mut x = X { a: 10, b: 20 };
    x -= &X { a: 7, b: 8 };
    assert_eq!(x, X { a: 3, b: 12 });
}
