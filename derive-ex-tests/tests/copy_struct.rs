use derive_ex::derive_ex;
use std::marker::PhantomData;

use derive_ex_tests::assert_impl;

#[test]
fn copy_struct() {
    #[derive(Clone)]
    struct NotCopy;

    #[derive_ex(Copy, Clone)]
    struct AlwaysCopy<T>(PhantomData<T>);

    assert_impl!(Copy, AlwaysCopy<NotCopy>);
}

#[test]
fn bound() {
    #[derive(Clone)]
    struct NotCopy;

    #[derive_ex(Copy(bound(T0)), Clone)]
    struct Test<T0, T1>(PhantomData<T0>, PhantomData<T1>);

    assert_impl!(Copy, Test<u32, NotCopy>);
    assert_impl!(!Copy, Test<NotCopy, u32>);
}
