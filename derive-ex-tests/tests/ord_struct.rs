use std::cmp::Ordering;

use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn ord_unit() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X;

    assert_eq!(X.cmp(&X), Ordering::Equal);
}

#[test]
fn ord_struct() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X {
        a: u8,
        b: String,
    }

    assert_eq!(
        X {
            a: 1,
            b: "A".into()
        }
        .cmp(&X {
            a: 1,
            b: "A".into()
        }),
        Ordering::Equal
    );

    assert_eq!(
        X {
            a: 1,
            b: "A".into()
        }
        .cmp(&X {
            a: 1,
            b: "B".into()
        }),
        Ordering::Less
    );
    assert_eq!(
        X {
            a: 2,
            b: "A".into()
        }
        .cmp(&X {
            a: 1,
            b: "B".into()
        }),
        Ordering::Greater
    );
}

#[test]
fn ord_tuple() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X(u32, String);

    assert_eq!(X(1, "A".into()).cmp(&X(1, "A".into())), Ordering::Equal);
    assert_eq!(X(1, "A".into()).cmp(&X(1, "A".into())), Ordering::Equal);

    assert_eq!(X(1, "A".into()).cmp(&X(1, "B".into())), Ordering::Less);
    assert_eq!(X(1, "B".into()).cmp(&X(1, "A".into())), Ordering::Greater);

    assert_eq!(X(1, "A".into()).cmp(&X(2, "A".into())), Ordering::Less);
    assert_eq!(X(2, "A".into()).cmp(&X(1, "A".into())), Ordering::Greater);

    assert_eq!(X(1, "B".into()).cmp(&X(2, "A".into())), Ordering::Less);
    assert_eq!(X(2, "A".into()).cmp(&X(1, "B".into())), Ordering::Greater);
}

#[test]
fn ord_ord_key() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X(#[ord(key = $.len())] String);

    assert_eq!(X("ABC".into()).cmp(&X("DEF".into())), Ordering::Equal);
    assert_eq!(X("A".into()).cmp(&X("AA".into())), Ordering::Less);
    assert_eq!(X("AA".into()).cmp(&X("A".into())), Ordering::Greater);
}

#[test]
fn ord_key_ref() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X(#[ord(key = $.as_str())] String);

    assert_impl!(Ord, X);
}

#[test]
fn ord_key_expr() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X(#[ord(key = $.len() + $.len())] String);

    assert_impl!(Ord, X);
}

#[test]
fn ord_ord_by() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X(#[ord(by = |this, other| this.len().cmp(&other.len()))] String);

    assert_eq!(X("ABC".into()).cmp(&X("DEF".into())), Ordering::Equal);
    assert_eq!(X("A".into()).cmp(&X("AA".into())), Ordering::Less);
}

#[test]
fn ord_ord_ignore() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X(
        #[ord(ignore)]
        #[allow(dead_code)]
        String,
    );

    assert_eq!(X("ABC".into()).cmp(&X("D".into())), Ordering::Equal);
}

#[test]
fn ord_auto_bound() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X<T>(T);

    assert_impl!(Ord, X<u32>);
    assert_impl!(!Ord, X<f32>);
}

#[test]
fn ord_ord_bound() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X<T>(#[ord(bound(T : Copy + Ord))] T);

    assert_impl!(Ord, X<u32>);
    assert_impl!(!Ord, X<String>);
}

#[test]
fn ord_ord_bound_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X<T>(#[ord(key = $.0, bound(T))] P<T>);

    assert_impl!(Ord, X<u32>);
    assert_impl!(!Ord, X<f64>);
    assert_impl!(Ord, X<String>);
}

#[test]
fn ord_reverse() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X(#[ord(reverse)] u8);

    assert_eq!(X(1).cmp(&X(1)), Ordering::Equal);
    assert_eq!(X(1).cmp(&X(2)), Ordering::Greater);
    assert_eq!(X(2).cmp(&X(1)), Ordering::Less);
}

#[test]
fn partial_ord_reverse_2() {
    #[derive(Debug)]
    #[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
    struct X(#[ord(reverse)] u8, u8);

    assert_eq!(X(1, 1).cmp(&X(1, 1)), Ordering::Equal);
    assert_eq!(X(1, 1).cmp(&X(1, 2)), Ordering::Less);
    assert_eq!(X(1, 1).cmp(&X(2, 1)), Ordering::Greater);
}
