use derive_ex::derive_ex;
use std::marker::PhantomData;

#[macro_use]
mod test_utils;

#[test]
fn copy_enum() {
    #[derive(Clone)]
    struct NonCopy;

    #[derive_ex(Copy, Clone)]
    enum AlwaysCopy<T> {
        A(PhantomData<T>),
        B,
    }

    assert_impl!(Copy, AlwaysCopy<NonCopy>);
}
