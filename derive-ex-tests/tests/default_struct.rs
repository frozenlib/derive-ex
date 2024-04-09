use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
#[allow(clippy::default_constructed_unit_structs)]
fn unit_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X;

    assert_eq!(X::default(), X);
}

#[test]

fn tuple_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X(String, u32);

    assert_eq!(X::default(), X(String::new(), 0))
}

#[test]

fn record_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X {
        a: String,
        b: u32,
    }
    assert_eq!(
        X::default(),
        X {
            a: String::new(),
            b: 0
        }
    );
}

#[test]
fn generics() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X<T>(T, T);

    assert_eq!(X::default(), X(0, 0));
}

#[test]
fn generics_auto_bound() {
    #[derive(Eq, PartialEq, Debug)]
    struct NoDefault<T>(T);

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X<T>(Option<NoDefault<T>>);

    assert_eq!(X::<NoDefault<u32>>::default(), X(None));
}

#[test]
fn generics_contains_self() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X<T>(T)
    where
        Self: MyTrait;

    impl MyTrait for X<u32> {}

    assert_eq!(X::default(), X(0));
}

#[test]
fn bound_struct_trait() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default(bound(T : MyTrait, ..)))]
    struct X<T>(T);

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_struct_common() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default, bound(T : MyTrait, ..))]
    struct X<T>(T);

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_field_trait() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X<T>(#[derive_ex(Default(bound(T : MyTrait, ..)))] T);

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn bound_struct_helper() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    #[default(_, bound(T : MyTrait, ..))]
    struct X<T> {
        a: T,
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
    struct X<T>(Inner<T>);

    assert_impl!(Default, X<u32>);
}

#[test]
fn bound_field_common() {
    trait MyTrait {}

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X<T>(#[derive_ex(Default, bound(T : MyTrait, ..))] T);

    impl MyTrait for u32 {}

    assert_impl!(Default, X<u32>);
    assert_impl!(!Default, X<u8>);
}

#[test]
fn default_value_of_struct() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    #[default(X(50))]
    struct X(u32);

    assert_eq!(X::default(), X(50));
}

#[test]
fn default_value_of_struct_str() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    #[default("abc")]
    struct X(String);

    impl<'a> From<&'a str> for X {
        fn from(s: &'a str) -> Self {
            X(s.into())
        }
    }

    assert_eq!(X::default(), X("abc".into()));
}

#[test]
fn default_value_of_field() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X(#[default(50)] u32);

    assert_eq!(X::default(), X(50));
}

#[test]
fn default_value_of_field_bound() {
    #[derive(Eq, PartialEq, Debug)]
    struct NoDefault;

    trait New {
        fn new() -> Self;
    }
    impl New for NoDefault {
        fn new() -> Self {
            NoDefault
        }
    }

    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X<T: New>(#[default(T::new())] T);

    assert_eq!(X::default(), X(NoDefault));
}

#[test]
fn default_value_of_field_default() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X(#[default(_)] u32);

    assert_eq!(X::default(), X(0));
}

#[test]
fn default_value_of_field_lit_str() {
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X(#[default("abc")] String);

    assert_eq!(X::default(), X("abc".into()));
}

#[test]
fn default_value_of_field_path() {
    const ABC: &str = "abc";
    #[derive(Eq, PartialEq, Debug)]
    #[derive_ex(Default)]
    struct X(#[default(ABC)] String);

    assert_eq!(X::default(), X("abc".into()));
}

#[test]
fn derive_macro() {
    #[derive(Eq, PartialEq, Debug, derive_ex::Ex)]
    #[derive_ex(Default)]
    struct X(#[default(50)] u32);

    assert_eq!(X::default(), X(50));
}
