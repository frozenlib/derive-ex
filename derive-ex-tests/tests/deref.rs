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
fn multiple_fields_tuple() {
    #[derive_ex(Deref, DerefMut)]
    #[expect(unused, reason = "testing with additional fields")]
    struct X(#[deref] u8, u32);

    let _: &u8 = &X(10u8, 1u32);
    let _: &mut u8 = &mut X(10u8, 1u32);
}
#[test]
fn multiple_fields_named() {
    #[derive_ex(Deref, DerefMut)]
    #[expect(unused, reason = "testing with additional fields")]
    struct X {
        #[deref]
        x: u8,
        y: u32,
    }

    let _: &u8 = &X { x: 10u8, y: 1u32 };
    let _: &mut u8 = &mut X { x: 10u8, y: 1u32 };
}
#[test]
fn not_first_field() {
    #[derive_ex(Deref, DerefMut)]
    #[expect(unused, reason = "testing with additional fields")]
    struct X {
        x: u8,
        #[deref]
        y: u32,
    }

    let _: &u32 = &X { x: 10u8, y: 1u32 };
    let _: &mut u32 = &mut X { x: 10u8, y: 1u32 };
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

#[test]
#[deny(deprecated)]
#[allow(unused)]
fn deref_deprecated() {
    #[deprecated]
    #[derive(derive_ex::Ex)]
    #[derive_ex(Deref)]
    struct X(u32);
}

#[test]
#[deny(deprecated)]
#[allow(unused)]
fn deref_allow_deprecated() {
    #[deprecated]
    #[allow(unused)]
    struct Y(u32);
    #[allow(deprecated)]
    impl std::ops::Deref for Y {
        type Target = u32;
        fn deref(&self) -> &u32 {
            &self.0
        }
    }

    #[allow(deprecated)]
    #[derive(derive_ex::Ex)]
    #[derive_ex(Deref)]
    struct X(Y);
}

#[test]
#[deny(deprecated)]
#[allow(unused)]
fn deref_mut_deprecated() {
    #[deprecated]
    #[derive(derive_ex::Ex)]
    #[derive_ex(Deref, DerefMut)]
    struct X(u32);
}

#[test]
#[deny(deprecated)]
#[allow(unused)]
fn deref_mut_allow_deprecated() {
    #[deprecated]
    #[allow(unused)]
    struct Y(u32);
    #[allow(deprecated)]
    impl std::ops::Deref for Y {
        type Target = u32;
        fn deref(&self) -> &u32 {
            &self.0
        }
    }
    #[allow(deprecated)]
    impl std::ops::DerefMut for Y {
        fn deref_mut(&mut self) -> &mut u32 {
            &mut self.0
        }
    }

    #[allow(deprecated)]
    #[derive(derive_ex::Ex)]
    #[derive_ex(Deref, DerefMut)]
    struct X(Y);
}
