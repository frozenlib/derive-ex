#[macro_export]
macro_rules! assert_impl {
    (@@ ($($lt:lifetime,)*), $trait:path, $($ty:ty),*) => {{
        struct Helper<T>(T);
        impl<$($lt,)* T: $trait> Helper<T>
        {
            #[allow(dead_code)]
            fn assert_impl(_msg: &str) {}
        }
        trait NotImpl {
            #[allow(dead_code)]
            fn assert_impl(msg: &str) {
                panic!("{}", msg)
            }
        }
        impl<T> NotImpl for Helper<T> {}
        $(
            <Helper<$ty>>::assert_impl(&format!(
                "`{}` should implement `{}`, but did not",
                stringify!($ty),
                stringify!($trait),
            ))
        );*
    }};
    (@@ ($($lt:lifetime,)*), !$trait:path, $($ty:ty),*) => {{
        struct Helper<T>(T);
        impl<$($lt,)* T: $trait> Helper<T> {
            #[allow(dead_code)]
            fn assert_not_impl(msg: &str) {
                panic!("{}", msg)
            }
        }
        trait NotImpl {
            #[allow(dead_code)]
            fn assert_not_impl(_msg: &str) {}
        }
        impl<T> NotImpl for Helper<T> {}
        $(
            <Helper<$ty>>::assert_not_impl(&format!(
                "`{}` should not implement `{}`, but it did",
                stringify!($ty),
                stringify!($trait),
            ))
        );*
    }};
    (for <$($lt:lifetime),*> $trait:path, $($ty:ty),*) => {
        $crate::assert_impl!(@@ ($($lt,)*), $trait, $($ty),*)
    };
    (for <$($lt:lifetime),*> !$trait:path, $($ty:ty),*) => {
        $crate::assert_impl!(@@ ($($lt,)*), !$trait, $($ty),*)
    };
    ($trait:path, $($ty:ty),*) => {
        $crate::assert_impl!(@@ (), $trait, $($ty),*)
    };
    (!$trait:path, $($ty:ty),*) => {
        $crate::assert_impl!(@@ (), !$trait, $($ty),*)
    };
}
