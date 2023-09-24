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
