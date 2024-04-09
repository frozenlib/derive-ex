use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn partial_eq_unit() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X;

    assert_eq!(X, X);
}

#[test]
fn partial_eq_struct() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X {
        a: u8,
        b: String,
    }

    assert_eq!(
        X {
            a: 1,
            b: "A".into()
        },
        X {
            a: 1,
            b: "A".into()
        }
    );

    assert_ne!(
        X {
            a: 1,
            b: "A".into()
        },
        X {
            a: 1,
            b: "B".into()
        }
    );
    assert_ne!(
        X {
            a: 1,
            b: "A".into()
        },
        X {
            a: 2,
            b: "A".into()
        }
    );
}

#[test]
fn partial_eq_tuple() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(u8, String);

    assert_eq!(X(1, "A".into()), X(1, "A".into()));

    assert_ne!(X(1, "A".into()), X(1, "B".into()));
    assert_ne!(X(1, "A".into()), X(2, "A".into()));
}

#[test]
fn partial_eq_partial_eq_key() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[partial_eq(key = $.len())] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_eq_key() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[eq(key = $.len())] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_partial_ord_key() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[partial_ord(key = $.len())] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_ord_key() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[ord(key = $.len())] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_key_identity() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[partial_eq(key = $)] String);

    assert_ne!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_key_ref() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[partial_eq(key = $.as_str())] String);

    assert_ne!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_key_expr() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[partial_eq(key = $.len() + $.len())] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_partial_eq_by() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[partial_eq(by = |this, other| this.len() == other.len())] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_eq_by() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[eq(by = |this, other| this.len() == other.len())] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_partial_ord_by() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[partial_ord(by = |this, other| this.len().partial_cmp(&other.len()))] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_ord_by() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(#[ord(by = |this, other| this.len().cmp(&other.len()))] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_partial_eq_ignore() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(
        #[partial_eq(ignore)]
        #[allow(dead_code)]
        String,
    );

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_eq!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_eq_ignore() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(
        #[eq(ignore)]
        #[allow(dead_code)]
        String,
    );

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_eq!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_partial_ord_ignore() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(
        #[partial_ord(ignore)]
        #[allow(dead_code)]
        String,
    );

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_eq!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_ord_ignore() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X(
        #[ord(ignore)]
        #[allow(dead_code)]
        String,
    );

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_eq!(X("A".into()), X("AA".into()));
}

#[test]
fn partial_eq_auto_bound() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X<T>(T);

    struct NotPartialEq;

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(!PartialEq, X<NotPartialEq>);
}

#[test]
fn partial_eq_partial_eq_bound() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X<T>(#[partial_eq(bound(T : Copy + PartialEq))] T);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_eq_bound() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X<T>(#[eq(bound(T : Copy + PartialEq))] T);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_partial_ord_bound() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X<T>(#[partial_ord(bound(T : Copy + PartialOrd))] T);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_ord_bound() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X<T>(#[ord(bound(T : Copy + Ord))] T);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_partial_eq_bound_type_at_field() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X<T>(#[partial_eq(key = $.0, bound(T))] P<T>);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(PartialEq, X<String>);
}

#[test]
fn partial_eq_partial_eq_bound_type_at_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    #[partial_eq(bound(T))]
    struct X<T>(#[partial_eq(key = $.0)] P<T>);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(PartialEq, X<String>);
}

#[test]
fn partial_eq_eq_bound_type_at_field() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X<T>(#[eq(key = $.0, bound(T))] P<T>);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(PartialEq, X<String>);
}

#[test]
fn partial_eq_eq_bound_type_at_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    #[eq(bound(T))]
    struct X<T>(#[eq(key = $.0)] P<T>);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(PartialEq, X<String>);
}

#[test]
fn partial_eq_partial_ord_bound_type_at_field() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X<T>(#[partial_ord(key = $.0, bound(T))] P<T>);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(PartialEq, X<String>);
}

#[test]
fn partial_eq_partial_ord_bound_type_at_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    #[partial_ord(bound(T))]
    struct X<T>(#[partial_ord(key = $.0)] P<T>);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(PartialEq, X<String>);
}

#[test]
fn partial_eq_ord_bound_type_at_field() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X<T>(#[ord(key = $.0, bound(T))] P<T>);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(PartialEq, X<String>);
}

#[test]
fn partial_eq_ord_bound_type_at_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    #[ord(bound(T))]
    struct X<T>(#[ord(key = $.0)] P<T>);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(PartialEq, X<String>);
}

#[test]
fn partial_eq_partial_eq_key_and_eq_bound() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    struct X<T>(
        #[partial_eq(key = $.0, bound(T, ..))]
        #[eq(bound(T : Copy + PartialEq))]
        P<T>,
    );

    // `#[partial_eq(key = ..)]` is specified, so `#[eq(...)]` is not used.
    // Therefore, `#[eq(bound(T : Copy + PartialEq))]` is not applied.
    assert_impl!(PartialEq, X<String>);
}

#[test]
fn derive_macro() {
    #[derive(Debug, derive_ex::Ex)]
    #[derive_ex(PartialEq)]
    struct X(#[partial_eq(key = $.len())] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}
