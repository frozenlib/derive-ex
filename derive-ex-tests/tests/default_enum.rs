#[macro_use]
mod test_utils;

use derive_ex::derive_ex;

#[test]
fn sinvle_variant() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X {
        A,
    }
    assert_eq!(X::default(), X::A);
}

#[test]
fn select_variant() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X {
        #[default]
        A,
        #[allow(dead_code)]
        B,
    }
    assert_eq!(X::default(), X::A);

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum Y {
        #[allow(dead_code)]
        A,
        #[default]
        B,
    }
    assert_eq!(Y::default(), Y::B);
}

#[test]
fn unit_variant() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X {
        #[allow(dead_code)]
        A,
    }
    assert_eq!(X::default(), X::A);
}

#[test]

fn tuple_variant() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X {
        A(String, u32),
    }

    assert_eq!(X::default(), X::A(String::new(), 0))
}

#[test]

fn record_variant() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X {
        A { a: String, b: u32 },
    }
    assert_eq!(
        X::default(),
        X::A {
            a: String::new(),
            b: 0
        }
    );
}

#[test]
fn generics() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X<T> {
        A(T),
    }
    assert_eq!(X::default(), X::A(0));
}

#[test]
fn generics_auto_bound() {
    #[derive(Eq, PartialEq, Debug)]
    struct NoDefault<T>(T);

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X<T> {
        #[allow(dead_code)]
        A(NoDefault<T>),
        #[default]
        B(Option<NoDefault<T>>),
    }
    assert_eq!(X::<NoDefault<u32>>::default(), X::B(None));
}

#[test]
fn generics_contains_self() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X<T>
    where
        Self: MyTrait,
    {
        A(T),
    }

    impl MyTrait for X<u32> {}

    assert_eq!(X::default(), X::A(0));
}

#[test]
fn bound_enum_trait() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default(bound(T : MyTrait, ..)))]
    enum X<T> {
        A(T),
    }

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_enum_common() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default, bound(T : MyTrait, ..))]
    enum X<T> {
        A(T),
    }

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_variant_trait() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X<T> {
        #[derive_ex(Default(bound(T : MyTrait, ..)))]
        A(T),
    }

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_variant_common() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X<T> {
        #[derive_ex(Default, bound(T : MyTrait, ..))]
        A(T),
    }

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_field_trait() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X<T> {
        A(#[derive_ex(Default(bound(T : MyTrait, ..)))] T),
    }

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_field_common() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X<T> {
        A(#[derive_ex(Default, bound(T : MyTrait, ..))] T),
    }

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_enum_helper() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    #[default(_, bound(T : MyTrait, ..))]
    enum X<T> {
        A(T),
    }

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_variant_helper() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X<T> {
        #[default(_, bound(T : MyTrait, ..))]
        A(T),
    }

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_field_helper() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X<T> {
        A(#[default(_, bound(T : MyTrait, ..))] T),
    }

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_type() {
    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Default)]
    struct Inner<T>(T);

    #[derive(Debug, Eq, PartialEq)]
    #[derive_ex(Default(bound(T)))]
    enum X<T> {
        A(Inner<T>),
    }

    assert_impl!(Default, X<u32>);
}

#[test]
fn default_value_of_enum() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    #[default(X::A(50))]
    enum X {
        A(u32),
    }

    assert_eq!(X::default(), X::A(50));
}

#[test]
fn default_value_of_field() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    enum X {
        A(#[default(50)] u32),
    }

    assert_eq!(X::default(), X::A(50));
}
