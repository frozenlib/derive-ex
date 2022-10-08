# derive-ex

[![Crates.io](https://img.shields.io/crates/v/derive-ex.svg)](https://crates.io/crates/derive-ex)
[![Docs.rs](https://docs.rs/derive-ex/badge.svg)](https://docs.rs/derive-ex/)
[![Actions Status](https://github.com/frozenlib/derive-ex/workflows/CI/badge.svg)](https://github.com/frozenlib/derive-ex/actions)

Improved version of the macro to implement the traits defined in the standard library.

## Documentation

See [`#[derive_ex]` documentation](https://docs.rs/derive-ex/latest/derive_ex/attr.derive_ex.html) for details.

## Differences from standard derive macros

- A trait bound that is automatically generated is smarter.
- You can specify trait bound manually.
- You can specify default values for each field.
- Support derive `Default` for enum.
- Support derive `Clone::clone_from`.
- Support derive operators. (`Add`, `AddAssign`, `Not`, `Deref`, etc.)

## Supported traits

- `Clone`
- `Defualt`
- operators
  - Add-like (`Add`, `Sub`, `Mul`, `Shl`, etc.)
  - AddAssign-like (`AddAssign`, `SubAssign`, `MulAssign`, `ShlAssign`, etc.)
  - Not-like (`Not`, `Neg`)
  - `Deref`, `DerefMut`

## Unsupported traits

The following traits are not supported as more suitable crates exist.

| trait                | crate                                                     |
| -------------------- | --------------------------------------------------------- |
| `Display`, `FromStr` | [`parse-display`](https://crates.io/crates/parse-display) |
| `Error`              | [`thiserror`](https://crates.io/crates/thiserror)         |

## Install

Add this to your Cargo.toml:

```toml
[dependencies]
derive-ex = "0.1.2"
```

## Example

```rust
use derive_ex::derive_ex;

#[derive(Eq, PartialEq, Debug)]
#[derive_ex(Add, AddAssign, Clone, Default)]
struct X {
    #[default(10)]
    a: u32,
}
assert_eq!(X { a: 1 } + X { a: 2 }, X { a: 3 });
assert_eq!(X::default(), X { a: 10 });

#[derive(Eq, PartialEq, Debug)]
#[derive_ex(Clone, Default)]
enum Y {
    A,
    #[default]
    B,
}
assert_eq!(Y::default(), Y::B);
```

## License

This project is dual licensed under Apache-2.0/MIT. See the two LICENSE-\* files for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
