use derive_ex::derive_ex;
use std::marker::PhantomData;

use derive_ex_tests::assert_impl;

#[test]
#[allow(dead_code)]
fn copy_enum() {
    #[derive(Clone)]
    struct NotCopy;

    #[derive_ex(Copy, Clone)]
    enum AlwaysCopy<T> {
        A(PhantomData<T>),
        B,
    }

    assert_impl!(Copy, AlwaysCopy<NotCopy>);
}

#[test]
#[deny(deprecated)]
#[allow(unused)]
fn copy_deprecated() {
    #[deprecated]
    #[derive(derive_ex::Ex)]
    #[derive_ex(Copy, Clone)]
    enum X {
        A,
    }
}

#[test]
#[deny(deprecated)]
#[allow(unused)]
fn copy_allow_deprecated() {
    #[deprecated]
    #[allow(unused)]
    #[derive(Copy, Clone)]
    struct Y;

    #[allow(deprecated)]
    #[derive(derive_ex::Ex)]
    #[derive_ex(Copy, Clone)]
    enum X {
        A(Y),
    }
}
