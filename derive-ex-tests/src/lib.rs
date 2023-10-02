#[cfg(doctest)]
mod tests {
    #[doc = include_str!("../../README.md")]
    mod readme_md {}
}

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

pub fn assert_debug_eq(a: impl std::fmt::Debug, e: impl std::fmt::Debug) {
    assert_eq!(format!("{a:?}"), format!("{e:?}"));
    assert_eq!(format!("{a:#?}"), format!("{e:#?}"));
}

pub fn assert_eq_hash<T: std::hash::Hash>(v0: T, v1: T) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    let mut h0 = DefaultHasher::default();
    v0.hash(&mut h0);

    let mut h1 = DefaultHasher::default();
    v1.hash(&mut h1);

    assert_eq!(h0.finish(), h1.finish());
}
