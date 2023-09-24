use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn partial_ord_empty() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X {}

    assert_impl!(PartialOrd, X);
}

#[test]
fn partial_ord_enum_unit_variant() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X {
        A,
        B,
    }
    assert_eq!(X::A.partial_cmp(&X::A), Some(std::cmp::Ordering::Equal));
    assert_eq!(X::A.partial_cmp(&X::B), Some(std::cmp::Ordering::Less));
    assert_eq!(X::B.partial_cmp(&X::A), Some(std::cmp::Ordering::Greater));
}

#[test]
fn partial_ord_enum_tuple_variant() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X {
        A(),
        B(u8),
        C(u8, String),
    }
    assert_eq!(X::A().partial_cmp(&X::A()), Some(std::cmp::Ordering::Equal));
    assert_eq!(
        X::B(1).partial_cmp(&X::B(1)),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B(1).partial_cmp(&X::B(2)),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        X::B(2).partial_cmp(&X::B(1)),
        Some(std::cmp::Ordering::Greater)
    );
    assert_eq!(
        X::C(1, "A".into()).partial_cmp(&X::C(2, "A".into())),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        X::C(2, "A".into()).partial_cmp(&X::C(1, "B".into())),
        Some(std::cmp::Ordering::Greater)
    );
}

#[test]
fn partial_ord_enum_record_variant() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X {
        A {},
        B { a: u8 },
        C { a: u8, b: String },
    }
    assert_eq!(
        X::A {}.partial_cmp(&X::A {}),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B { a: 1 }.partial_cmp(&X::B { a: 1 }),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B { a: 1 }.partial_cmp(&X::B { a: 2 }),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        X::B { a: 2 }.partial_cmp(&X::B { a: 1 }),
        Some(std::cmp::Ordering::Greater)
    );
    assert_eq!(
        X::C {
            a: 1,
            b: "A".into()
        }
        .partial_cmp(&X::C {
            a: 2,
            b: "A".into()
        }),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        X::C {
            a: 2,
            b: "A".into()
        }
        .partial_cmp(&X::C {
            a: 1,
            b: "B".into()
        }),
        Some(std::cmp::Ordering::Greater)
    );
}

#[test]
fn partial_ord_partial_ord_key() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X {
        A(#[partial_ord(key = $.len())] String),
        B(String),
    }
    assert_eq!(
        X::A("ABC".into()).partial_cmp(&X::A("DEF".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::A("A".into()).partial_cmp(&X::A("ABC".into())),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("ABC".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("DEF".into())),
        Some(std::cmp::Ordering::Less)
    );
}

#[test]
fn partial_ord_ord_key() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X {
        A(#[ord(key = $.len())] String),
        B(String),
    }
    assert_eq!(
        X::A("ABC".into()).partial_cmp(&X::A("DEF".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::A("A".into()).partial_cmp(&X::A("ABC".into())),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("ABC".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("DEF".into())),
        Some(std::cmp::Ordering::Less)
    );
}

#[test]
fn partial_ord_partial_ord_by() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X {
        A(#[partial_ord(by = |this, other| this.len().partial_cmp(&other.len()))] String),
        B(String),
    }

    assert_eq!(
        X::A("ABC".into()).partial_cmp(&X::A("DEF".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::A("A".into()).partial_cmp(&X::A("ABC".into())),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("ABC".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("DEF".into())),
        Some(std::cmp::Ordering::Less)
    );
}

#[test]
fn partial_ord_ord_by() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X {
        A(#[ord(by = |this, other| this.len().cmp(&other.len()))] String),
        B(String),
    }
    assert_eq!(
        X::A("ABC".into()).partial_cmp(&X::A("DEF".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::A("A".into()).partial_cmp(&X::A("ABC".into())),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("ABC".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("DEF".into())),
        Some(std::cmp::Ordering::Less)
    );
}

#[test]
fn partial_ord_partial_ord_ignore() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X {
        A(#[partial_ord(ignore)] String),
        B(String),
    }
    assert_eq!(
        X::A("ABC".into()).partial_cmp(&X::A("Z".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("ABC".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("Z".into())),
        Some(std::cmp::Ordering::Less)
    );
}

#[test]
fn partial_ord_ord_ignore() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X {
        A(#[ord(ignore)] String),
        B(String),
    }
    assert_eq!(
        X::A("ABC".into()).partial_cmp(&X::A("Z".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("ABC".into())),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        X::B("ABC".into()).partial_cmp(&X::B("Z".into())),
        Some(std::cmp::Ordering::Less)
    );
}

#[test]
fn partial_ord_auto_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X<T> {
        A(T),
        B(String),
    }

    #[derive(PartialEq)]
    struct NotPartialOrd;

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(!PartialOrd, X<NotPartialOrd>);
}

#[test]
fn partial_ord_partial_ord_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X<T> {
        A(#[partial_ord(bound(T : Copy + PartialOrd))] T),
        B(String),
    }

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(!PartialOrd, X<String>);
}

#[test]
fn partial_ord_ord_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X<T> {
        A(#[ord(bound(T : Copy + PartialOrd))] T),
        B(String),
    }

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(!PartialOrd, X<String>);
}

#[test]
fn partial_ord_partial_ord_bound_type_at_field() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X<T> {
        A(#[partial_ord(bound(T : Copy + PartialOrd))] T),
        B(String),
    }

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(PartialOrd, X<f64>);
    assert_impl!(!PartialOrd, X<String>);
}

#[test]
fn partial_ord_partial_ord_bound_type_at_variant() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X<T> {
        #[partial_ord(bound(T : Copy + PartialOrd))]
        A(T),
        B(String),
    }

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(PartialOrd, X<f64>);
    assert_impl!(!PartialOrd, X<String>);
}

#[test]
fn partial_ord_partial_ord_bound_type_at_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    #[partial_ord(bound(T : Copy + PartialOrd))]
    enum X<T> {
        A(T),
        B(String),
    }

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(PartialOrd, X<f64>);
    assert_impl!(!PartialOrd, X<String>);
}

#[test]
fn partial_ord_ord_bound_type_at_field() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X<T> {
        A(#[ord(bound(T : Copy + PartialOrd))] T),
        B(String),
    }

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(PartialOrd, X<f64>);
    assert_impl!(!PartialOrd, X<String>);
}

#[test]
fn partial_ord_ord_bound_type_at_variant() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    enum X<T> {
        #[ord(bound(T : Copy + PartialOrd))]
        A(T),
        B(String),
    }

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(PartialOrd, X<f64>);
    assert_impl!(!PartialOrd, X<String>);
}

#[test]
fn partial_ord_ord_bound_type_at_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    #[ord(bound(T : Copy + PartialOrd))]
    enum X<T> {
        A(T),
        B(String),
    }

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(PartialOrd, X<f64>);
    assert_impl!(!PartialOrd, X<String>);
}
