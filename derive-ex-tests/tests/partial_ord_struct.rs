use std::cmp::Ordering;

use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn partial_ord_unit() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X;

    assert_eq!(X.partial_cmp(&X), Some(Ordering::Equal));
}

#[test]
fn partial_ord_struct() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X {
        a: u8,
        b: String,
    }

    assert_eq!(
        X {
            a: 1,
            b: "A".into()
        }
        .partial_cmp(&X {
            a: 1,
            b: "A".into()
        }),
        Some(Ordering::Equal)
    );

    assert_eq!(
        X {
            a: 1,
            b: "A".into()
        }
        .partial_cmp(&X {
            a: 1,
            b: "B".into()
        }),
        Some(Ordering::Less)
    );
    assert_eq!(
        X {
            a: 2,
            b: "A".into()
        }
        .partial_cmp(&X {
            a: 1,
            b: "B".into()
        }),
        Some(Ordering::Greater)
    );
}

#[test]
fn partial_ord_tuple() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(f64, String);

    assert_eq!(
        X(1.0, "A".into()).partial_cmp(&X(1.0, "A".into())),
        Some(Ordering::Equal)
    );

    assert_eq!(
        X(1.0, "A".into()).partial_cmp(&X(1.0, "B".into())),
        Some(Ordering::Less)
    );
    assert_eq!(
        X(1.0, "B".into()).partial_cmp(&X(1.0, "A".into())),
        Some(Ordering::Greater)
    );

    assert_eq!(
        X(1.0, "A".into()).partial_cmp(&X(2.0, "A".into())),
        Some(Ordering::Less)
    );
    assert_eq!(
        X(2.0, "A".into()).partial_cmp(&X(1.0, "A".into())),
        Some(Ordering::Greater)
    );

    assert_eq!(
        X(1.0, "B".into()).partial_cmp(&X(2.0, "A".into())),
        Some(Ordering::Less)
    );
    assert_eq!(
        X(2.0, "A".into()).partial_cmp(&X(1.0, "B".into())),
        Some(Ordering::Greater)
    );

    assert_eq!(
        X(f64::NAN, "B".into()).partial_cmp(&X(1.0, "A".into())),
        None,
    );
    assert_eq!(
        X(1.0, "A".into()).partial_cmp(&X(f64::NAN, "B".into())),
        None,
    );
}

#[test]
fn partial_ord_partial_ord_key() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(#[partial_ord(key = $.len())] String);

    assert_eq!(
        X("ABC".into()).partial_cmp(&X("DEF".into())),
        Some(Ordering::Equal)
    );
    assert_eq!(
        X("A".into()).partial_cmp(&X("AA".into())),
        Some(Ordering::Less)
    );
}

#[test]
fn partial_ord_ord_key() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(#[ord(key = $.len())] String);

    assert_eq!(
        X("ABC".into()).partial_cmp(&X("DEF".into())),
        Some(Ordering::Equal)
    );
    assert_eq!(
        X("A".into()).partial_cmp(&X("AA".into())),
        Some(Ordering::Less)
    );
    assert_eq!(
        X("AA".into()).partial_cmp(&X("A".into())),
        Some(Ordering::Greater)
    );
}

#[test]
fn partial_ord_key_ref() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(#[partial_ord(key = $.as_str())] String);

    assert_impl!(PartialOrd, X);
}

#[test]
fn partial_ord_key_expr() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(#[partial_ord(key = $.len() + $.len())] String);

    assert_impl!(PartialOrd, X);
}

#[test]
fn partial_ord_partial_ord_by() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(#[partial_ord(by = |this, other| this.len().partial_cmp(&other.len()))] String);

    assert_eq!(
        X("ABC".into()).partial_cmp(&X("DEF".into())),
        Some(Ordering::Equal)
    );
    assert_eq!(
        X("A".into()).partial_cmp(&X("AA".into())),
        Some(Ordering::Less)
    );
}

#[test]
fn partial_ord_ord_by() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(#[ord(by = |this, other| this.len().cmp(&other.len()))] String);

    assert_eq!(
        X("ABC".into()).partial_cmp(&X("DEF".into())),
        Some(Ordering::Equal)
    );
    assert_eq!(
        X("A".into()).partial_cmp(&X("AA".into())),
        Some(Ordering::Less)
    );
}

#[test]
fn partial_ord_partial_ord_skip() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(
        #[partial_ord(skip)]
        #[allow(dead_code)]
        String,
    );

    assert_eq!(
        X("ABC".into()).partial_cmp(&X("D".into())),
        Some(Ordering::Equal)
    );
}

#[test]
fn partial_ord_ord_skip() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(
        #[ord(skip)]
        #[allow(dead_code)]
        String,
    );

    assert_eq!(
        X("ABC".into()).partial_cmp(&X("D".into())),
        Some(Ordering::Equal)
    );
}

#[test]
fn partial_ord_auto_bound() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X<T>(T);

    struct NotPartialOrd;

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(!PartialOrd, X<NotPartialOrd>);
}

#[test]
fn partial_ord_partial_ord_bound() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X<T>(#[partial_ord(bound(T : Copy + PartialOrd))] T);

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(!PartialOrd, X<String>);
}

#[test]
fn partial_ord_ord_bound() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X<T>(#[ord(bound(T : Copy + Ord))] T);

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(!PartialOrd, X<String>);
}

#[test]
fn partial_ord_partial_ord_bound_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X<T>(#[partial_ord(key = $.0, bound(T))] P<T>);

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(PartialOrd, X<f64>);
    assert_impl!(PartialOrd, X<String>);
}

#[test]
fn partial_ord_ord_bound_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X<T>(#[ord(key = $.0, bound(T))] P<T>);

    assert_impl!(PartialOrd, X<u32>);
    assert_impl!(PartialOrd, X<f64>);
    assert_impl!(PartialOrd, X<String>);
}

#[test]
fn partial_ord_partial_ord_key_and_ord_bound() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X<T>(
        #[partial_ord(key = $.0, bound(T, ..))]
        #[ord(bound(T : Copy + PartialEq))]
        P<T>,
    );

    // `#[partial_ord(key = ..)]` is specified, so `#[ord(...)]` is not used.
    // Therefore, `#[ord(bound(T : Copy + PartialEq))]` is not applied.
    assert_impl!(PartialOrd, X<String>);
}

#[test]
fn partial_ord_reverse() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(#[partial_ord(reverse)] u8);

    assert_eq!(X(1).partial_cmp(&X(1)), Some(Ordering::Equal));
    assert_eq!(X(1).partial_cmp(&X(2)), Some(Ordering::Greater));
    assert_eq!(X(2).partial_cmp(&X(1)), Some(Ordering::Less));
}

#[test]
fn partial_ord_reverse_2() {
    #[derive(Debug)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(#[partial_ord(reverse)] u8, u8);

    assert_eq!(X(1, 1).partial_cmp(&X(1, 1)), Some(Ordering::Equal));
    assert_eq!(X(1, 1).partial_cmp(&X(1, 2)), Some(Ordering::Less));
    assert_eq!(X(1, 1).partial_cmp(&X(2, 1)), Some(Ordering::Greater));
}

#[test]
fn derive_macro() {
    #[derive(Debug, derive_ex::Ex)]
    #[derive_ex(PartialOrd, PartialEq)]
    struct X(#[partial_ord(key = $.len())] String);

    assert_eq!(
        X("ABC".into()).partial_cmp(&X("DEF".into())),
        Some(Ordering::Equal)
    );
    assert_eq!(
        X("A".into()).partial_cmp(&X("AA".into())),
        Some(Ordering::Less)
    );
}
