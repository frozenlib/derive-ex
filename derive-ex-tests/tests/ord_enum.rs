use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn ord_empty() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X {}

    assert_impl!(Ord, X);
}

#[test]
fn ord_enum_unit_variant() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X {
        A,
        B,
    }
    assert_eq!(X::A.cmp(&X::A), std::cmp::Ordering::Equal);
    assert_eq!(X::A.cmp(&X::B), std::cmp::Ordering::Less);
    assert_eq!(X::B.cmp(&X::A), std::cmp::Ordering::Greater);
}

#[test]
fn ord_enum_tuple_variant() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X {
        A(),
        B(u8),
        C(u8, String),
    }
    assert_eq!(X::A().cmp(&X::A()), std::cmp::Ordering::Equal);
    assert_eq!(X::B(1).cmp(&X::B(1)), std::cmp::Ordering::Equal);
    assert_eq!(X::B(1).cmp(&X::B(2)), std::cmp::Ordering::Less);
    assert_eq!(X::B(2).cmp(&X::B(1)), std::cmp::Ordering::Greater);
    assert_eq!(
        X::C(1, "A".into()).cmp(&X::C(2, "A".into())),
        std::cmp::Ordering::Less
    );
    assert_eq!(
        X::C(2, "A".into()).cmp(&X::C(1, "B".into())),
        std::cmp::Ordering::Greater
    );
}

#[test]
fn ord_enum_record_variant() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X {
        A {},
        B { a: u8 },
        C { a: u8, b: String },
    }
    assert_eq!(X::A {}.cmp(&X::A {}), std::cmp::Ordering::Equal);
    assert_eq!(X::B { a: 1 }.cmp(&X::B { a: 1 }), std::cmp::Ordering::Equal);
    assert_eq!(X::B { a: 1 }.cmp(&X::B { a: 2 }), std::cmp::Ordering::Less);
    assert_eq!(
        X::B { a: 2 }.cmp(&X::B { a: 1 }),
        std::cmp::Ordering::Greater
    );
    assert_eq!(
        X::C {
            a: 1,
            b: "A".into()
        }
        .cmp(&X::C {
            a: 2,
            b: "A".into()
        }),
        std::cmp::Ordering::Less
    );
    assert_eq!(
        X::C {
            a: 2,
            b: "A".into()
        }
        .cmp(&X::C {
            a: 1,
            b: "B".into()
        }),
        std::cmp::Ordering::Greater
    );
}

#[test]
fn ord_ord_key() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X {
        A(#[ord(key = $.len())] String),
        B(String),
    }
    assert_eq!(
        X::A("ABC".into()).cmp(&X::A("DEF".into())),
        std::cmp::Ordering::Equal
    );
    assert_eq!(
        X::A("A".into()).cmp(&X::A("ABC".into())),
        std::cmp::Ordering::Less
    );
    assert_eq!(
        X::B("ABC".into()).cmp(&X::B("ABC".into())),
        std::cmp::Ordering::Equal
    );
    assert_eq!(
        X::B("ABC".into()).cmp(&X::B("DEF".into())),
        std::cmp::Ordering::Less
    );
}

#[test]
fn ord_ord_by() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X {
        A(#[ord(by = |this, other| this.len().cmp(&other.len()))] String),
        B(String),
    }
    assert_eq!(
        X::A("ABC".into()).cmp(&X::A("DEF".into())),
        std::cmp::Ordering::Equal
    );
    assert_eq!(
        X::A("A".into()).cmp(&X::A("ABC".into())),
        std::cmp::Ordering::Less
    );
    assert_eq!(
        X::B("ABC".into()).cmp(&X::B("ABC".into())),
        std::cmp::Ordering::Equal
    );
    assert_eq!(
        X::B("ABC".into()).cmp(&X::B("DEF".into())),
        std::cmp::Ordering::Less
    );
}

#[test]
fn ord_ord_ignore() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X {
        A(#[ord(ignore)] String),
        B(String),
    }
    assert_eq!(
        X::A("ABC".into()).cmp(&X::A("Z".into())),
        std::cmp::Ordering::Equal
    );
    assert_eq!(
        X::B("ABC".into()).cmp(&X::B("ABC".into())),
        std::cmp::Ordering::Equal
    );
    assert_eq!(
        X::B("ABC".into()).cmp(&X::B("Z".into())),
        std::cmp::Ordering::Less
    );
}

#[test]
fn ord_auto_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X<T> {
        A(T),
        B(String),
    }

    #[derive(PartialOrd, PartialEq)]
    struct NotOrd;

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(PartialOrd, X<NotOrd>);

    assert_impl!(Ord, X<u32>);
    assert_impl!(!Ord, X<NotOrd>);
}

#[test]
fn ord_ord_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X<T> {
        A(#[ord(bound(T : Copy + Ord))] T),
        B(String),
    }

    assert_impl!(Ord, X<u32>);
    assert_impl!(!Ord, X<String>);
}

#[test]
fn ord_partial_ord_bound_type_at_field() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X<T> {
        A(#[ord(bound(T : Copy + Ord))] T),
        B(String),
    }

    assert_impl!(Ord, X<u32>);
    assert_impl!(!Ord, X<String>);
}

#[test]
fn ord_partial_ord_bound_type_at_variant() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    enum X<T> {
        #[ord(bound(T : Copy + Ord))]
        A(T),
        B(String),
    }

    assert_impl!(Ord, X<u32>);
    assert_impl!(!Ord, X<String>);
}

#[test]
fn ord_partial_ord_bound_type_at_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    #[ord(bound(T : Copy + Ord))]
    enum X<T> {
        A(T),
        B(String),
    }

    assert_impl!(Ord, X<u32>);
    assert_impl!(!Ord, X<String>);
}
