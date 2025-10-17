#![allow(clippy::large_enum_variant)]

extern crate proc_macro;

#[macro_use]
mod syn_utils;

mod bound;
mod common;
mod item_impl;
mod item_type;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Item, Result};

// #[include_doc("../../doc/derive_ex.md", start)]
/// Improved version of the macro to implement the traits defined in the standard library.
///
/// - [Attributes](#attributes)
/// - [Derive `Copy`](#derive-copy)
/// - [Derive `Clone`](#derive-clone)
/// - [Derive `Debug`](#derive-debug)
///   - [`#[debug(skip)]`](#debugskip)
///   - [`#[debug(transparent)]`](#debugtransparent)
///   - [`#[debug(bound(...))]`](#debugbound)
/// - [Derive `Default`](#derive-default)
/// - [Derive `Ord`, `PartialOrd`, `Eq`, `PartialEq`, `Hash`](#derive-ord-partialord-eq-partialeq-hash)
///   - [`#[ord(skip)]`](#ordskip)
///   - [`#[ord(reverse)]`](#ordreverse)
///   - [`#[ord(by = ...)]`](#ordby--)
///   - [`#[ord(key = ...)]`](#ordkey--)
///   - [`#[ord(bound(...))]`](#ordbound)
/// - [Derive `Deref`](#derive-deref)
/// - [Derive `DerefMut`](#derive-derefmut)
/// - [Derive operators](#derive-operators)
///   - [`Add`-like](#add-like)
///     - [Derive `Add` from struct definition](#derive-add-from-struct-definition)
///     - [Derive `Add` from `impl Add`](#derive-add-from-impl-add)
///     - [Derive `Add` from `impl AddAssign`](#derive-add-from-impl-addassign)
///   - [`AddAssign`-like](#addassign-like)
///     - [Derive `AddAssign` from struct definition](#derive-addassign-from-struct-definition)
///     - [Derive `AddAssign` from `impl Add`](#derive-addassign-from-impl-add)
///     - [Derive both `Add` and `AddAssign` from `impl Add`](#derive-both-add-and-addassign-from-impl-add)
///   - [`Not`-like](#not-like)
/// - [Specify trait bound](#specify-trait-bound)
///   - [`#[bound(T)]`](#boundt)
///   - [`#[bound(T : TraitName)]`](#boundt--traitname)
///   - [`#[bound(..)]`](#bound)
///   - [`#[bound()]`](#bound-1)
/// - [Display generated code](#display-generated-code)
///
/// # Attributes
///
/// You can write attributes in the following positions.
///
/// | attribute                  | impl | struct | enum | variant | field |
/// | -------------------------- | ---- | ------ | ---- | ------- | ----- |
/// | `#[derive_ex(Copy)]`       |      | ✔      | ✔    | ✔       | ✔     |
/// | `#[derive_ex(Clone)]`      |      | ✔      | ✔    | ✔       | ✔     |
/// | `#[derive_ex(Debug)]`      |      | ✔      | ✔    | ✔       | ✔     |
/// | `#[derive_ex(Default)]`    |      | ✔      | ✔    | ✔       | ✔     |
/// | `#[derive_ex(Ord)]`        |      | ✔      | ✔    | ✔       | ✔     |
/// | `#[derive_ex(Deref)]`      |      | ✔      |      |         |       |
/// | `#[derive_ex(DerefMut)]`   |      | ✔      |      |         |       |
/// | `#[derive_ex(Add)]`        | ✔    | ✔      |      |         | ✔     |
/// | `#[derive_ex(AddAssign)]`  | ✔    | ✔      |      |         | ✔     |
/// | `#[derive_ex(Not)]`        |      | ✔      | ✔    | ✔       | ✔     |
/// | `#[derive_ex(bound(...))]` |      | ✔      | ✔    | ✔       | ✔     |
/// | `#[derive_ex(dump))]`      | ✔    | ✔      | ✔    |         |       |
/// | `#[default]`               |      | ✔      | ✔    | ✔       | ✔     |
/// | `#[debug]`                 |      | ✔      | ✔    | ✔       | ✔     |
/// | `#[ord]`                   |      | ✔      | ✔    | ✔       | ✔     |
///
/// # Derive `Copy`
///
/// You can use `#[derive_ex(Copy)]` to implement [`Copy`].
///
/// ```rust
/// use derive_ex::derive_ex;
/// use std::marker::PhantomData;
///
/// #[derive_ex(Copy, Clone)]
/// struct X<T>(PhantomData<T>);
/// ```
///
/// The standard `#[derive(Copy)]` sets `Copy` constraint on the generic parameters, while `#[derive_ex(Copy)]` sets `Copy` constraint on the type of field containing generic parameters.
///
/// For example, adding `#[derive(Copy)]` to the previous structure `X<T>` generates the following code.
///
/// Since `where T: Copy` is specified, if `T` does not implement `DebugCopy`, then `X<T>` does not implement `Copy`.
///
/// ```rust
/// # use std::marker::PhantomData;
/// # #[derive(Clone)]
/// # struct X<T>(PhantomData<T>);
/// use core::marker::Copy;
/// impl<T: Copy> Copy for X<T>
/// where
///     T : Copy
/// {
/// }
/// ```
///
/// On the other hand, `#[derive_ex(Copy)]` generates the following code.
///
/// Unlike the standard `#[derive(Copy)]`, `X<T>` always implements `Copy`.
///
/// ```rust
/// # use derive_ex::derive_ex;
/// # use std::marker::PhantomData;
/// # #[derive_ex(Clone)]
/// # struct X<T>(PhantomData<T>);
/// use core::marker::Copy;
/// impl<T: Copy> Copy for X<T>
/// where
///     PhantomData<T>: Copy,
/// {
/// }
/// ```
///
/// # Derive `Clone`
///
/// You can use `#[derive_ex(Clone)]` to implement [`Clone`].
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(Clone)]
/// struct X(String);
///
/// #[derive_ex(Clone)]
/// enum Y {
///   X,
///   B,
/// }
/// ```
///
/// It has the following differences from the standard `#[derive(Clone)]`.
///
/// - Generates [`Clone::clone_from`].
/// - The standard `#[derive(Clone)]` sets `Clone` constraint on the generic parameters, while `#[derive_ex(Clone)]` sets `Clone` constraint on the type of field containing generic parameters.
///
/// For example, to derive `Clone` for the following type
///
/// ```rust
/// use std::rc::Rc;
/// struct X<T> {
///     a: Rc<T>,
///     b: String,
/// }
/// ```
///
/// The standard `#[derive(Clone)]` generates the following code.
///
/// Since `where T: Clone` is specified, if `T` does not implement `Clone`, then `X<T>` does not implement `Clone`.
///
/// ```rust
/// # use std::rc::Rc;
/// # struct X<T> {
/// #    a: Rc<T>,
/// #    b: String,
/// # }
/// impl<T> Clone for X<T>
/// where
///     T: Clone,
/// {
///     fn clone(&self) -> Self {
///         X {
///             a: self.a.clone(),
///             b: self.b.clone(),
///         }
///     }
/// }
/// ```
///
/// On the other hand, `#[derive_ex(Clone)]` generates the following code.
///
/// Unlike the standard `#[derive(Clone)]`, `X<T>` always implements `Clone`.
///
/// ```rust
/// # use std::rc::Rc;
/// # struct X<T> {
/// #    a: Rc<T>,
/// #    b: String,
/// # }
/// impl<T> Clone for X<T>
/// where
///     Rc<T>: Clone,
/// {
///     fn clone(&self) -> Self {
///         X {
///             a: self.a.clone(),
///             b: self.b.clone(),
///         }
///     }
///     fn clone_from(&mut self, source: &Self) {
///         self.a.clone_from(&source.a);
///         self.b.clone_from(&source.b);
///     }
/// }
/// ```
///
/// If you want to remove `where Rc<T>: Clone` in the above example, you can use `#[derive(Clone(bound()))]`.
/// See [Specify trait bound](#specify-trait-bound) for details.
///
/// # Derive `Debug`
///
/// You can use `#[derive_ex(Debug)]` to implement [`Debug`].
///
/// The following helper attribute arguments allow you to customize your `Debug` implementation.
///
/// | attribute                          | struct | enum | variant | field |
/// | ---------------------------------- | ------ | ---- | ------- | ----- |
/// | [`skip`](#debugskip)               |        |      |         | ✔     |
/// | [`transparent`](#debugtransparent) |        |      |         | ✔     |
/// | [`bound(...)`](#debugbound)        | ✔      | ✔    | ✔       | ✔     |
///
/// ## `#[debug(skip)]`
///
/// By setting `#[debug(skip)]` to a field, you can exclude that field from debug output.
///
/// For backward compatibility, `#[debug(ignore)]` is also supported and works the same as `skip`.
///
/// ```rust
/// use derive_ex::derive_ex;
/// #[derive_ex(Debug)]
/// struct X {
///     a: u32,
///     #[debug(skip)]
///     b: u32,
/// }
/// assert_eq!(format!("{:?}", X { a: 1, b: 2 }), "X { a: 1 }");
/// ```
///
/// ## `#[debug(transparent)]`
///
/// You can transfer processing to a field by setting `#[debug(transparent)]` to that field.
///
/// ```rust
/// use derive_ex::derive_ex;
/// #[derive_ex(Debug)]
/// struct X {
///     a: u32,
///     #[debug(transparent)]
///     b: u32,
/// }
/// assert_eq!(format!("{:?}", X { a: 1, b: 2 }), "2");
/// ```
///
/// ## `#[debug(bound(...))]`
///
/// The standard `#[derive(Debug)]` sets `Debug` constraint on the generic parameters, while `#[derive_ex(Debug)]` sets `Debug` constraint on the type of field containing generic parameters.
///
/// For example, to derive `Debug` for the following type
///
/// ```rust
/// use std::marker::PhantomData;
/// struct X<T>(PhantomData<T>);
/// ```
///
/// The standard `#[derive(Debug)]` generates the following code.
///
/// Since `where T: Debug` is specified, if `T` does not implement `Debug`, then `X<T>` does not implement `Debug`.
///
/// ```rust
/// # use std::marker::PhantomData;
/// # struct X<T>(PhantomData<T>);
/// use std::fmt::Debug;
/// impl<T> Debug for X<T>
/// where
///     T: Debug,
/// {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.debug_struct("X").finish()
///     }
/// }
/// ```
///
/// On the other hand, `#[derive_ex(Debug)]` generates the following code.
///
/// Unlike the standard `#[derive(Debug)]`, `X<T>` always implements `Debug`.
///
/// ```rust
/// # use std::marker::PhantomData;
/// # struct X<T>(PhantomData<T>);
/// use std::fmt::Debug;
/// impl<T> Debug for X<T>
/// where
///     PhantomData<T>: Debug,
/// {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.debug_struct("X").finish()
///     }
/// }
/// ```
///
/// You can also manually specify a trait bound. See [Specify trait bound](#specify-trait-bound) for details.
///
/// # Derive `Default`
///
/// You can use `#[derive_ex(Default)]` to implement [`Default`].
///
/// ```rust
/// use derive_ex::derive_ex;
/// #[derive_ex(Default)]
/// struct X {
///     a: u8,
/// }
/// let x = X::default();
/// assert_eq!(x.a, 0);
/// ```
///
/// You can use `#[default(...)]` with struct or enum to set the default value.
///
/// ```rust
/// use derive_ex::derive_ex;
/// #[derive_ex(Default)]
/// #[default(X::new())]
/// struct X(u8);
///
/// impl X {
///     fn new() -> Self {
///         X(5)
///     }
/// }
///
/// let x = X::default();
/// assert_eq!(x.0, 5);
/// ```
///
/// You can use `#[default(...)]` with a field to set the field's default value.
///
/// ```rust
/// use derive_ex::derive_ex;
/// #[derive_ex(Default)]
/// struct X {
///     #[default(5)]
///     a: u8,
/// }
///
/// let x = X::default();
/// assert_eq!(x.a, 5);
/// ```
///
/// You can use `#[default]` to set the default variant.
///
/// ```rust
/// use derive_ex::derive_ex;
/// #[derive(Debug, Eq, PartialEq)]
/// #[derive_ex(Default)]
/// enum X {
///     A,
///     #[default]
///     B,
/// }
/// assert_eq!(X::default(), X::B);
/// ```
///
/// If a string literal or a single path is specified in `#[default(...)]`, conversion by `Into` is performed.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive(Debug, PartialEq)]
/// #[derive_ex(Default)]
/// struct X(#[default("abc")]String);
/// assert_eq!(X::default(), X("abc".into()));
///
/// const S: &str = "xyz";
/// #[derive(Debug, PartialEq)]
/// #[derive_ex(Default)]
/// struct Y(#[default(S)]String);
/// assert_eq!(Y::default(), Y(S.into()));
/// ```
///
/// The standard `#[derive(Default)]` sets `Default` constraint on the generic parameters, while `#[derive_ex(Default)]` sets `Default` constraint on the type of field containing generic parameters.
///
/// For example, to derive `Default` for the following type
///
/// ```rust
/// struct X<T>(Option<T>);
/// ```
///
/// The standard `#[derive(Default)]` generates the following code.
///
/// Since `where T: Default` is specified, if `T` does not implement `Default`, then `X<T>` does not implement `Default`.
///
/// ```rust
/// # struct X<T>(Option<T>);
/// impl<T> Default for X<T>
/// where
///     T: Default,
/// {
///     fn default() -> Self {
///         X(<Option<T>>::default())
///     }
/// }
/// ```
///
/// On the other hand, `#[derive_ex(Default)]` generates the following code.
///
/// Unlike the standard `#[derive(Default)]`, `X<T>` always implements `Default`.
///
/// ```rust
/// # struct X<T>(Option<T>);
/// impl<T> Default for X<T>
/// where
///     Option<T>: Default,
/// {
///     fn default() -> Self {
///         X(<Option<T>>::default())
///     }
/// }
/// ```
///
/// Field types with manually set default values are not used as type constraints.
///
/// In the following example, `where T : Default` is not generated.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive(Eq, PartialEq, Debug)]
/// struct NoDefault;
/// trait New {
///     fn new() -> Self;
/// }
/// impl New for NoDefault {
///     fn new() -> Self {
///         NoDefault
///     }
/// }
///
/// #[derive(Eq, PartialEq, Debug)]
/// #[derive_ex(Default)]
/// struct X<T: New> {
///     #[default(T::new())]
///     a: T,
/// }
/// assert_eq!(X::default(), X { a: NoDefault })
/// ```
///
/// # Derive `Ord`, `PartialOrd`, `Eq`, `PartialEq`, `Hash`
///
/// `Ord`, `PartialOrd`, `Eq`, `PartialEq`, `Hash` can be derived using common settings.
///
/// To avoid repeating settings, one helper attribute affects multiple traits.
///
/// The table below shows which helper attributes affect which trait.
///
/// The helper attributes in the lines below are applied preferentially.
///
/// | attribute             | `Ord` | `PartialOrd` | `Eq` | `PartialEq` | `Hash` |
/// | --------------------- | ----- | ------------ | ---- | ----------- | ------ |
/// | `#[ord(...)]`         | ✔     | ✔            | ✔    | ✔           | ✔      |
/// | `#[partial_ord(...)]` |       | ✔            |      | ✔           |        |
/// | `#[eq(...)]`          |       |              | ✔    | ✔           | ✔      |
/// | `#[partial_eq(...)]`  |       |              | ✔    | ✔           |        |
/// | `#[hash(...)]`        |       |              |      |             | ✔      |
///
/// The following table shows the helper attribute arguments and the locations where they can be used.
///
/// | argument                  | struct | enum | variant | field | `#[ord]` | `#[partial_ord]` | `#[eq]` | `#[partial_eq]` | `#[hash]` |
/// | ------------------------- | ------ | ---- | ------- | ----- | -------- | ---------------- | ------- | --------------- | --------- |
/// | [`skip`](#ordskip)        |        |      |         | ✔     | ✔        | ✔                | ✔       | ✔               | ✔         |
/// | [`reverse`](#ordreverse)  |        |      |         | ✔     | ✔        | ✔                |         |                 |           |
/// | [`by = ...`](#ordby--)    |        |      |         | ✔     | ✔        | ✔                | ✔       | ✔               | ✔         |
/// | [`key = ...`](#ordkey--)  |        |      |         | ✔     | ✔        | ✔                | ✔       | ✔               | ✔         |
/// | [`bound(...)`](#ordbound) | ✔      | ✔    | ✔       | ✔     | ✔        | ✔                | ✔       | ✔               | ✔         |
///
/// If you modify the behavior of a specific trait by `by = ...` or `key = ...`, you must also modify the behavior of other traits by `by = ...` or `key = ...`.
/// Trying to mix modified behavior with default behavior will result in a compilation error.
///
/// ## `#[ord(skip)]`
///
/// Specifying `skip` for a field excludes that field from comparison and hash calculation.
///
/// You cannot change whether `skip` is applied by the trait.
///
/// A compile error occurs when trying to apply `skip` to only some of the traits.
///
/// For backward compatibility, `ignore` is also supported and works the same as `skip`.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(Eq, PartialEq, Debug)]
/// struct X(#[eq(skip)] u8);
///
/// assert_eq!(X(1), X(2));
/// ```
///
/// ## `#[ord(reverse)]`
///
/// Specifying `reverse` for a field reverses the comparison order.
///
/// It can also be used together with `by = ...` or `key = ...`.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(PartialOrd, PartialEq, Debug)]
/// struct X(#[partial_ord(reverse)] u8);
///
/// assert!(X(1) > X(2));
/// ```
///
/// ## `#[ord(by = ...)]`
///
/// You can set up a comparison method by specifying a function with the same signature as the trait's required method.
///
/// `#[hash(by = ...)]` only changes the behavior of `Hash`.
///
/// Other attributes act on attributes other than `Hash`.
///
/// ```rust
/// use derive_ex::derive_ex;
/// use std::cmp::Ordering;
///
/// #[derive_ex(Ord, PartialOrd, Eq, PartialEq, Debug)]
/// struct X(#[ord(by = f64::total_cmp)] f64);
///
/// assert_eq!(X(1.0).cmp(&X(2.0)), Ordering::Less);
/// ```
///
/// ## `#[ord(key = ...)]`
///
/// Delegates processing to the value of the specified expression.
///
/// In this expression, the field itself is represented by `$`.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(Eq, PartialEq, Debug)]
/// struct X(#[eq(key = $.len())] &'static str);
///
/// assert_eq!(X("a"), X("b"));
/// assert_ne!(X("a"), X("aa"));
/// ```
///
/// ## `#[ord(bound(...))]`
///
/// Specify the trait bounds.
///
/// If the default trait bounds are specified with a high-priority helper attribute, the trait bounds of a lower-priority helper attribute will be used.
///
/// If `by = ...` or `key = ...` is specified with a high-priority helper attribute, the trait bounds of a lower-priority helper attribute will not be used.
///
/// For details, see [Specify trait bound](#specify-trait-bound).
///
/// # Derive `Deref`
///
/// You can use `#[derive(Deref)]` for struct with a single field to implement `Deref`.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(Deref)]
/// struct X(u8);
///
/// let _: &u8 = &X(10u8);
/// ```
///
/// # Derive `DerefMut`
///
/// You can use `#[derive(DerefMut)]` for struct with a single field to implement `DerefMut`.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(Deref, DerefMut)]
/// struct X(u8);
///
/// let _: &mut u8 = &mut X(10u8);
/// ```
///
/// # Derive operators
///
/// ## `Add`-like
///
/// In this document, `Add` is used as an example, but all of the following traits can be used in the same way.
///
/// - `Add`
/// - `BitAnd`
/// - `BitOr`
/// - `BitXor`
/// - `Div`
/// - `Mul`
/// - `Rem`
/// - `Shl`
/// - `Shr`
/// - `Sub`
///
/// ### Derive `Add` from struct definition
///
/// You can use `#[derive(Add)]` on struct definition to derive `Add`.
///
/// ```rust
/// use derive_ex::derive_ex;
/// #[derive_ex(Add)]
/// struct X {
///     a: u8,
///     b: u32,
/// }
/// ```
///
/// The above code generates the following code.
///
/// ```rust
/// # use ::core::ops::Add;
/// # use derive_ex::derive_ex;
/// # struct X {
/// #     a: u8,
/// #     b: u32,
/// # }
/// impl Add<X> for X {
///     type Output = X;
///     fn add(self, rhs: X) -> Self::Output {
///         X {
///             a: self.a + rhs.a,
///             b: self.b + rhs.b,
///         }
///     }
/// }
/// impl Add<&X> for X {
///     type Output = X;
///     fn add(self, rhs: &X) -> Self::Output {
///         X {
///             a: self.a + &rhs.a,
///             b: self.b + &rhs.b,
///         }
///     }
/// }
/// impl Add<X> for &X {
///     type Output = X;
///     fn add(self, rhs: X) -> Self::Output {
///         X {
///             a: &self.a + rhs.a,
///             b: &self.b + rhs.b,
///         }
///     }
/// }
/// impl Add<&X> for &X {
///     type Output = X;
///     fn add(self, rhs: &X) -> Self::Output {
///         X {
///             a: &self.a + &rhs.a,
///             b: &self.b + &rhs.b,
///         }
///     }
/// }
/// ```
///
/// ### Derive `Add` from `impl Add`
///
/// By applying `#[derive_ex(Add)]` to one of the following, you can implement the remaining three.
///
/// - `impl Add<T> for T`
/// - `impl Add<T> for &T`
/// - `impl Add<&T> for T`
/// - `impl Add<&T> for &T`
///
/// `T` must implement `Clone`, except when `#[derive_ex(Add)]` is applied to `impl Add<&T> for &T`.
///
/// ```rust
/// use derive_ex::derive_ex;
/// use std::ops::Add;
///
/// #[derive(Clone)]
/// struct A {
///     a: u8,
/// }
///
/// #[derive_ex(Add)]
/// impl Add<A> for A {
///     type Output = A;
///     fn add(self, rhs: A) -> Self::Output {
///         A { a: self.a + rhs.a }
///     }
/// }
/// ```
///
/// The above code generates the following code.
///
/// ```rust
/// # use std::ops::Add;
/// #
/// # #[derive(Clone)]
/// # struct A {
/// #     a: u8,
/// # }
/// #
/// # impl Add<A> for A {
/// #     type Output = A;
/// #     fn add(self, rhs: A) -> Self::Output {
/// #         A { a: self.a + rhs.a }
/// #     }
/// # }
/// impl Add<&A> for A {
///     type Output = A;
///     fn add(self, rhs: &A) -> Self::Output {
///         self + rhs.clone()
///     }
/// }
/// impl Add<A> for &A {
///     type Output = A;
///     fn add(self, rhs: A) -> Self::Output {
///         self.clone() + rhs
///     }
/// }
/// impl Add<&A> for &A {
///     type Output = A;
///     fn add(self, rhs: &A) -> Self::Output {
///         self.clone() + rhs.clone()
///     }
/// }
/// ```
///
/// ### Derive `Add` from `impl AddAssign`
///
/// By applying `#[derive_ex(Add)]` to `impl AddAssign<Rhs> for T`, you can implement `Add<Rhs> for T`.
///
/// ```rust
/// use derive_ex::derive_ex;
/// use std::ops::AddAssign;
///
/// struct X {
///     a: u8,
///     b: u32,
/// }
/// #[derive_ex(Add)]
/// impl std::ops::AddAssign<u8> for X {
///     fn add_assign(&mut self, rhs: u8) {
///         self.a += rhs;
///         self.b += rhs as u32;
///     }
/// }
/// ```
///
/// The above code generates the following code.
///
/// ```rust
/// # use derive_ex::derive_ex;
/// # use std::ops::AddAssign;
/// #
/// # struct X {
/// #     a: u8,
/// #     b: u32,
/// # }
/// # impl AddAssign<u8> for X {
/// #     fn add_assign(&mut self, rhs: u8) {
/// #         self.a += rhs;
/// #         self.b += rhs as u32;
/// #     }
/// # }
/// use std::ops::Add;
///
/// impl Add<u8> for X {
///     type Output = X;
///     fn add(mut self, rhs: u8) -> Self::Output {
///         self += rhs;
///         self
///     }
/// }
/// ```
///
/// ## `AddAssign`-like
///
/// In this document, `AddAssign` is used as an example, but all of the following traits can be used in the same way.
///
/// - `AddAssign`
/// - `BitAndAssign`
/// - `BitOrAssign`
/// - `BitXorAssign`
/// - `DivAssign`
/// - `MulAssign`
/// - `RemAssign`
/// - `ShlAssign`
/// - `ShrAssign`
/// - `SubAssign`
///
/// ### Derive `AddAssign` from struct definition
///
/// You can use `#[derive(AddAssign)]` on struct definition to derive `Assign`.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(AddAssign)]
/// struct X {
///     a: u8,
///     b: u32,
/// }
/// ```
///
/// The above code generates the following code.
///
/// ```rust
/// # use derive_ex::derive_ex;
/// # use ::core::ops::Add;
/// #
/// # struct X {
/// #     a: u8,
/// #     b: u32,
/// # }
/// use std::ops::AddAssign;
///
/// impl AddAssign<X> for X {
///     fn add_assign(&mut self, rhs: X) {
///         self.a += rhs.a;
///         self.b += rhs.b;
///     }
/// }
/// impl AddAssign<&X> for X {
///     fn add_assign(&mut self, rhs: &X) {
///         self.a += &rhs.a;
///         self.b += &rhs.b;
///     }
/// }
/// ```
///
/// ### Derive `AddAssign` from `impl Add`
///
/// By applying `#[derive_ex(AddAssign)]` to `impl Add<Rhs> for T` or `impl Add<Rhs> for &T`, you can implement `AddAssign<Rhs> for T`.
///
/// When `#[derive_ex(AddAssign)]` is applied to `impl Add<Rhs> for &T`, `T` must implement `Clone`.
///
/// When `#[derive_ex(AddAssign)]` is applied to `impl Add<Rhs> for T`, `T` does not have to implement `Clone`.
///
/// ```rust
/// use derive_ex::derive_ex;
/// use std::ops::Add;
///
/// #[derive(Clone)]
/// struct X {
///     a: u8,
///     b: u32,
/// }
///
/// #[derive_ex(AddAssign)]
/// impl Add<u8> for X {
///     type Output = X;
///     fn add(mut self, rhs: u8) -> Self::Output {
///         self.a += rhs;
///         self.b += rhs as u32;
///         self
///     }
/// }
/// ```
///
/// ```rust
/// # use derive_ex::derive_ex;
/// # use std::ops::Add;
/// #
/// # #[derive(Clone)]
/// # struct X {
/// #     a: u8,
/// #     b: u32,
/// # }
/// #
/// # impl Add<u8> for X {
/// #     type Output = X;
/// #     fn add(mut self, rhs: u8) -> Self::Output {
/// #         self.a += rhs;
/// #         self.b += rhs as u32;
/// #         self
/// #     }
/// # }
/// use std::ops::AddAssign;
///
/// impl AddAssign<u8> for X {
///     fn add_assign(&mut self, rhs: u8) {
///         *self = self.clone() + rhs;
///     }
/// }
/// ```
///
/// ### Derive both `Add` and `AddAssign` from `impl Add`
///
/// Applying `#[derive_ex(Add, AddAssign)]` to one of the four below will implement the other three, plus generate the same code as when `#[derive_ex(AddAssign)]` is applied to `impl Add<T> for &T` and `impl Add<&T> for &T`.
///
/// - `impl Add<T> for T`
/// - `impl Add<T> for &T`
/// - `impl Add<&T> for T`
/// - `impl Add<&T> for &T`
///
/// ## `Not`-like
///
/// In this document, `Not` is used as an example, but all of the following traits can be used in the same way.
///
/// - `Not`
/// - `Neg`
///
/// You can use `#[derive_ex(Not)]` to implement `Not`.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(Not)]
/// struct X {
///     a: bool,
///     b: bool,
/// }
/// ```
///
/// The above code generates the following code.
///
/// ```rust
/// # use derive_ex::derive_ex;
/// #
/// # struct X {
/// #     a: bool,
/// #     b: bool,
/// # }
/// use std::ops::Not;
///
/// impl Not for X {
///     type Output = X;
///     fn not(self) -> Self::Output {
///         X {
///             a: !self.a,
///             b: !self.b,
///         }
///     }
/// }
/// impl Not for &X {
///     type Output = X;
///     fn not(self) -> Self::Output {
///         X {
///             a: !&self.a,
///             b: !&self.b,
///         }
///     }
/// }
/// ```
///
/// # Specify trait bound
///
/// If the type definition or impl item to which `#[derive_ex]` is applied has generic parameters, then by default, trait bound required by the auto-generated code is set.
///
/// You can change the generated trait bound by specifying `bound(...)` in attribute.
///
/// `bound(...)` can be used in the following places, the lower the number, the higher the priority.
///
/// |                                       | struct, enum | variant | field |
/// | ------------------------------------- | ------------ | ------- | ----- |
/// | `#[trait_name(bound(...))]`           | 1            | 4       | 7     |
/// | `#[derive_ex(TraitName(bound(...)))]` | 2            | 5       | 8     |
/// | `#[derive_ex(TraitName, bound(...))]` | 3            | 6       | 9     |
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(Default(bound(T)/* <-- 2 */), bound(T)/* <-- 3 */)]
/// #[default(_, bound(T))] // <-- 1
/// enum X<T> {
///     A,
///     #[derive_ex(Default(bound(T)/* <-- 5 */), bound(T)/* <-- 6 */)]
///     #[default(_, bound(T))] // <-- 4
///     B {
///         #[derive_ex(Default(bound(T)/* <-- 8 */), bound(T)/* <-- 9 */)]
///         #[default(_, bound(T))] // <-- 7
///         t: T,
///     },
/// }
/// ```
///
/// In 3, 6, and 9, common trait bound can be set for multiple traits by using it as like `#[derive_ex(Clone, Default, bound(T))]`.
///
/// Trait bound can be specified in the following three ways.
///
/// - Type (`T`)
/// - Predicate (`T : TraitName`)
/// - Default (`..`)
///
/// ## `#[bound(T)]`
///
/// If you specify a type in `#[bound(...)]`, make sure that the specified type implements the trait to be generated.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(Default, Clone, bound(T))]
/// struct X<T>(Box<T>);
/// ```
///
/// The above code generates the following code.
///
/// By `bound(T)`, `Box<T> : Default` is changed to `T : Default` and `Box<T> : Clone` is changed to `T : Clone`.
///
/// ```rust
/// # use derive_ex::derive_ex;
/// # struct X<T>(Box<T>);
/// impl<T> Default for X<T>
/// where
///     T: Default,
/// {
///     fn default() -> Self {
///         X(Box::default())
///     }
/// }
///
/// impl<T> Clone for X<T>
/// where
///     T: Clone,
/// {
///     fn clone(&self) -> Self {
///         X(self.0.clone())
///     }
///     fn clone_from(&mut self, source: &Self) {
///         self.0.clone_from(&source.0)
///     }
/// }
/// ```
///
/// ## `#[bound(T : TraitName)]`
///
/// If a predicate is specified, it is used as-is.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(Clone, bound(T : Clone + Copy))]
/// struct X<T>(T);
/// ```
///
/// The above code generates the following code.
///
/// By `bound(T : Clone + Copy)`, `T : Clone` is changed to `T : Clone + Copy`.
///
/// ```rust
/// # use derive_ex::derive_ex;
/// # struct X<T>(T);
/// impl<T> Clone for X<T>
/// where
///     T: Clone + Copy,
/// {
///     fn clone(&self) -> Self {
///         X(self.0.clone())
///     }
///     fn clone_from(&mut self, source: &Self) {
///         self.0.clone_from(&source.0)
///     }
/// }
/// ```
///
/// ## `#[bound(..)]`
///
/// `..` means default trait bound.
///
/// If `..` is specified, the lower priority trait bound is used.
///
/// ```rust
/// use derive_ex::derive_ex;
///
/// #[derive_ex(Clone, bound(T : Copy, ..))]
/// struct X<T>(T);
/// ```
///
/// The above code generates the following code.
///
/// By `bound(T : Copy, ..)` ,`T : Copy` is added to `T : Clone` which is the default trait bound.
///
/// ```rust
/// # use derive_ex::derive_ex;
/// # struct X<T>(T);
/// impl<T> Clone for X<T>
/// where
///     T: Clone,
///     T: Copy,
/// {
///     fn clone(&self) -> Self {
///         X(self.0.clone())
///     }
///     fn clone_from(&mut self, source: &Self) {
///         self.0.clone_from(&source.0)
///     }
/// }
/// ```
///
/// ## `#[bound()]`
///
/// An empty argument means no constraint.
///
/// ```rust
/// use derive_ex::derive_ex;
/// use std::rc::Rc;
///
/// #[derive_ex(Clone, bound())]
/// struct X<T>(Rc<T>);
/// ```
///
/// The above code generates the following code.
///
/// By `bound()`, `where Rc<T> : Clone` is removed.
///
/// ```rust
/// # use std::rc::Rc;
/// # struct X<T>(Rc<T>);
/// impl<T> Clone for X<T> {
///     fn clone(&self) -> Self {
///         X(self.0.clone())
///     }
///     fn clone_from(&mut self, source: &Self) {
///         self.0.clone_from(&source.0)
///     }
/// }
/// ```
///
/// # Display generated code
///
/// The generated code is output as an error message by using `#[derive_ex(dump)]`.
///
/// ```compile_fail
/// #[derive_ex(Clone, dump)]
/// struct X<T>(T);
/// ```
///
/// The above code causes the following error.
///
/// ```txt
/// error: dump:
///        impl < T > :: core :: clone :: Clone for X < T > where T : :: core :: clone ::
///        Clone,
///        {
///            fn clone(& self) -> Self
///            { X(< T as :: core :: clone :: Clone > :: clone(& self.0),) } fn
///            clone_from(& mut self, source : & Self)
///            {
///                < T as :: core :: clone :: Clone > ::
///                clone_from(& mut self.0, & source.0) ;
///            }
///        }
/// ```
///
/// As with `bound(...)`, `dump` can be applied to multiple traits by writing `#[derive_ex(Clone, Default, dump)]`.
// #[include_doc("../../doc/derive_ex.md", end)]
#[proc_macro_attribute]
pub fn derive_ex(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item: TokenStream = item.into();
    match build(attr.into(), item.clone()) {
        Ok(s) => s,
        Err(e) => {
            item.extend(e.to_compile_error());
            item
        }
    }
    .into()
}

/// Use attribute macro [`macro@derive_ex`] as derive macro.
///
/// [`macro@derive_ex`], being an attribute macro designed to mimic the functionality of the derive macro,
/// may cause rust-analyzer's assistance to not work correctly in certain cases.
///
/// Adding `#[derive(Ex)]` to an item with `#[derive_ex]` will allow rust-analyzer's assistance to work correctly.
///
/// In the example below, without `#[derive(Ex)]`,
/// the jump from `value: String` to the definition of `String` is not possible,
/// but with `#[derive(Ex)]`, it is possible.
///
/// ```
/// use derive_ex::Ex;
///
/// #[derive(Ex)]
/// #[derive_ex(Eq, PartialEq)]
/// struct X {
///     #[eq(key = $.len())]
///     value: String,
/// }
/// ```
#[proc_macro_derive(
    Ex,
    attributes(derive_ex, ord, partial_ord, eq, partial_eq, hash, debug, default)
)]
pub fn derive_ex_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    match item_type::build_derive(input) {
        Ok(s) => s,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

fn build(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let mut item: Item = parse2(item)?;
    let ts = match &mut item {
        Item::Struct(item_struct) => item_type::build_by_item_struct(attr, item_struct),
        Item::Enum(item_enum) => item_type::build_by_item_enum(attr, item_enum),
        Item::Impl(item_impl) => item_impl::build_by_item_impl(attr, item_impl),
        _ => bail!(
            _,
            "`#[derive_ex]` can be specified only for `struct`, `enum`, or `impl`.",
        ),
    }
    .unwrap_or_else(|e| e.to_compile_error());

    Ok(quote!(#item #ts))
}
