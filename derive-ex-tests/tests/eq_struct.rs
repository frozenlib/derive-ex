use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn eq_unit() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X;

    assert_impl!(Eq, X);
    assert_eq!(X, X);
}

#[test]
fn eq_struct() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X {
        a: u8,
        b: String,
    }

    assert_impl!(Eq, X);
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
fn eq_tuple() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X(u8, String);

    assert_impl!(Eq, X);

    assert_eq!(X(1, "A".into()), X(1, "A".into()));

    assert_ne!(X(1, "A".into()), X(1, "B".into()));
    assert_ne!(X(1, "A".into()), X(2, "A".into()));
}

#[test]
fn eq_eq_key() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X(#[eq(key = $.len())] String);

    assert_impl!(Eq, X);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn eq_ord_key() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X(#[ord(key = $.len())] String);

    assert_impl!(Eq, X);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn eq_key_ref() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X(#[eq(key = $.as_str())] String);

    assert_ne!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn eq_key_expr() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X(#[eq(key = $.len() + $.len())] String);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn eq_eq_by() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X(#[eq(by = |this, other| this.len() == other.len())] String);

    assert_impl!(Eq, X);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn eq_ord_by() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X(#[ord(by = |this, other| this.len().cmp(&other.len()))] String);

    assert_impl!(Eq, X);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}

#[test]
fn eq_eq_ignore() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X(
        #[eq(ignore)]
        #[allow(dead_code)]
        String,
    );

    assert_impl!(Eq, X);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_eq!(X("A".into()), X("AA".into()));
}

#[test]
fn eq_ord_ignore() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X(
        #[ord(ignore)]
        #[allow(dead_code)]
        String,
    );

    assert_impl!(Eq, X);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_eq!(X("A".into()), X("AA".into()));
}

#[test]
fn eq_atuo_bounds() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X<T>(T);

    assert_impl!(Eq, X<u32>);
    assert_impl!(!Eq, X<f64>);
}

#[test]
fn eq_eq_bound() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X<T>(#[eq(bound(T : Copy + Eq))] T);

    assert_impl!(Eq, X<u32>);
    assert_impl!(!Eq, X<String>);
}

#[test]
fn eq_ord_bound() {
    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X<T>(#[ord(bound(T : Copy + Ord))] T);

    assert_impl!(Eq, X<u32>);
    assert_impl!(!Eq, X<String>);
}

#[test]
fn eq_eq_bound_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X<T>(#[eq(key = $.0, bound(T))] P<T>);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(Eq, X<u32>);
    assert_impl!(!Eq, X<f64>);
}

#[test]
fn eq_ord_bound_type() {
    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(Eq, PartialEq)]
    struct X<T>(#[ord(key = $.0, bound(T))] P<T>);

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(Eq, X<u32>);
    assert_impl!(!Eq, X<f64>);
}

#[test]
fn derive_macro() {
    #[derive(Debug, derive_ex::Ex)]
    #[derive_ex(Eq, PartialEq)]
    struct X(#[eq(key = $.len())] String);

    assert_impl!(Eq, X);

    assert_eq!(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("A".into()), X("AA".into()));
}
