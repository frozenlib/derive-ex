// #![include_doc("../../README.md", start)]
//! # derive-ex
//!
//! [![Crates.io](https://img.shields.io/crates/v/derive-ex.svg)](https://crates.io/crates/derive-ex)
//! [![Docs.rs](https://docs.rs/derive-ex/badge.svg)](https://docs.rs/derive-ex/)
//! [![Actions Status](https://github.com/frozenlib/derive-ex/workflows/CI/badge.svg)](https://github.com/frozenlib/derive-ex/actions)
//!
//! Improved version of the macro to implement the traits defined in the standard library.
//!
//! ## Documentation
//!
//! See [`#[derive_ex]` documentation](https://docs.rs/derive-ex/latest/derive_ex/attr.derive_ex.html) for details.
//!
//! ## Differences from standard derive macros
//!
//! - A trait bound that is automatically generated is smarter.
//! - You can specify trait bound manually.
//! - You can specify default values for each field.
//! - You can specify ignored field with the derivation of `Debug`.
//! - Support derive `Clone::clone_from`.
//! - Support derive operators. (`Add`, `AddAssign`, `Not`, `Deref`, etc.)
//!
//! ## Supported traits
//!
//! - `Copy`
//! - `Clone`
//! - `Debug`
//! - `Default`
//! - operators
//!   - Add-like (`Add`, `Sub`, `Mul`, `Shl`, etc.)
//!   - AddAssign-like (`AddAssign`, `SubAssign`, `MulAssign`, `ShlAssign`, etc.)
//!   - Not-like (`Not`, `Neg`)
//!   - `Deref`, `DerefMut`
//!
//! ## Unsupported traits
//!
//! The following traits are not supported as more suitable crates exist.
//!
//! | trait                | crate                                                     |
//! | -------------------- | --------------------------------------------------------- |
//! | `Display`, `FromStr` | [`parse-display`](https://crates.io/crates/parse-display) |
//! | `Error`              | [`thiserror`](https://crates.io/crates/thiserror)         |
//!
//! ## Install
//!
//! Add this to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! derive-ex = "0.1.5"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use derive_ex::derive_ex;
//!
//! #[derive(Eq, PartialEq, Debug)]
//! #[derive_ex(Add, AddAssign, Clone, Default)]
//! struct X {
//!     #[default(10)]
//!     a: u32,
//! }
//! assert_eq!(X { a: 1 } + X { a: 2 }, X { a: 3 });
//! assert_eq!(X::default(), X { a: 10 });
//!
//! #[derive(Eq, PartialEq, Debug)]
//! #[derive_ex(Clone, Default)]
//! enum Y {
//!     A,
//!     #[default]
//!     B,
//! }
//! assert_eq!(Y::default(), Y::B);
//! ```
// #![include_doc("../../README.md", end("## License"))]

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
