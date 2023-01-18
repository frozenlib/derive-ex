use derive_ex::derive_ex;
use std::marker::PhantomData;

#[macro_use]
mod test_utils;

#[test]
fn copy_struct() {
    #[derive(Clone)]
    struct NonCopy;

    #[derive_ex(Copy, Clone)]
    struct AlwaysCopy<T>(PhantomData<T>);

    assert_impl!(Copy, AlwaysCopy<NonCopy>);
}

#[test]
fn bound() {
    #[derive(Clone)]
    struct NonCopy;

    #[derive_ex(Copy(bound(T0)), Clone)]
    struct Test<T0, T1>(PhantomData<T0>, PhantomData<T1>);

    assert_impl!(Copy, Test<u32, NonCopy>);
    assert_impl!(!Copy, Test<NonCopy, u32>);
}
