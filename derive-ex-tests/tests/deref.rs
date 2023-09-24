use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn derive_deref() {
    #[derive_ex(Deref)]
    struct X(u8);

    let _: &u8 = &X(10u8);
}

#[test]
fn derive_deref_mut() {
    #[derive_ex(Deref, DerefMut)]
    struct X(u8);

    let _: &mut u8 = &mut X(10u8);
}

#[test]
fn new_type() {
    #[derive_ex(Deref, DerefMut)]
    struct X(u8);

    let _: &u8 = &X(10u8);
    let _: &mut u8 = &mut X(10u8);
}
#[test]
fn single_field() {
    #[derive_ex(Deref, DerefMut)]
    struct X {
        x: u8,
    }

    let _: &u8 = &X { x: 10u8 };
    let _: &mut u8 = &mut X { x: 10u8 };
}

#[test]
fn with_where() {
    #[derive_ex(Deref, DerefMut)]
    struct X<T>(T)
    where
        T: Copy;

    let _: &u8 = &X(10u8);
    let _: &mut u8 = &mut X(10u8);
}

#[test]
fn with_bound() {
    use std::ops::{Deref, DerefMut};

    #[derive_ex(Deref, DerefMut, bound(T : Copy))]
    struct X<T>(T);

    let _: &u8 = &X(10u8);
    let _: &mut u8 = &mut X(10u8);

    assert_impl!(!Deref, X<String>);
    assert_impl!(!DerefMut, X<String>);
}
