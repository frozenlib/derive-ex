use derive_ex::derive_ex;
use derive_ex_tests::{assert_eq_hash, assert_impl};
use std::hash::Hash;

#[test]
fn hash_unit() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X;

    assert_impl!(Hash, X);
}

#[test]
fn hash_struct() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X {
        a: u8,
        b: String,
    }
    assert_impl!(Hash, X);
}

#[test]
fn hash_tuple() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X(u8, String);

    assert_impl!(Hash, X);
}

#[test]
fn hash_hash_key() {
    #[derive(Debug)]
    #[derive_ex(Hash)]
    struct X(#[hash(key = $.len())] String);

    assert_eq_hash(X("ABC".into()), X("DEF".into()));
}

#[test]
fn hash_eq_key() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X(#[eq(key = $.len())] String);

    assert_eq_hash(X("ABC".into()), X("DEF".into()));
}

#[test]
fn hash_ord_key() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X(#[ord(key = $.len())] String);

    assert_eq_hash(X("ABC".into()), X("DEF".into()));
}

#[test]
fn hash_key_ref() {
    #[derive(Debug)]
    #[derive_ex(Hash)]
    struct X(#[hash(key = $.as_str())] String);

    assert_impl!(Hash, X);
}

#[test]
fn hash_key_expr() {
    #[derive(Debug)]
    #[derive_ex(Hash)]
    struct X(#[hash(key = $.len() + $.len())] String);

    assert_eq_hash(X("ABC".into()), X("DEF".into()));
}

#[test]
fn hash_hash_by() {
    #[derive(Debug)]
    #[derive_ex(Hash)]
    struct X(#[hash(by = |this, state| this.len().hash(state))] String);

    assert_eq_hash(X("ABC".into()), X("DEF".into()));
}

#[test]
fn hash_hash_ignore() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X(#[hash(ignore)] String);

    assert_eq_hash(X("ABC".into()), X("DEF".into()));
    assert_ne!(X("ABC".into()), X("DEF".into()));
}

#[test]
fn hash_eq_ignore() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X(
        #[eq(ignore)]
        #[allow(dead_code)]
        String,
    );

    assert_eq_hash(X("ABC".into()), X("DEF".into()));
    assert_eq!(X("ABC".into()), X("DEF".into()));
}

#[test]
fn hash_ord_ignore() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X(
        #[ord(ignore)]
        #[allow(dead_code)]
        String,
    );

    assert_eq_hash(X("ABC".into()), X("DEF".into()));
    assert_eq!(X("ABC".into()), X("DEF".into()));
}

#[test]
fn hash_atuo_bounds() {
    #[derive(Debug, Eq, PartialEq)]
    struct NoHash;

    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X<T>(T);

    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<NoHash>);
}

#[test]
fn hash_hash_bound() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X<T>(#[hash(bound(T : Copy + Hash))] T);

    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<String>);
}

#[test]
fn hash_eq_bound() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X<T>(#[eq(bound(T : Copy + Eq + Hash))] T);

    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<String>);
}

#[test]
fn hash_ord_bound() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X<T>(#[ord(bound(T : Copy + Ord + Hash))] T);

    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<String>);
}

#[test]
fn hash_hash_bound_type() {
    #[derive(Debug, Eq, PartialEq)]
    struct NoHash;

    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(Hash)]
    struct X<T>(#[hash(key = $.0, bound(T))] P<T>);

    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<NoHash>);
}

#[test]
fn hash_eq_bound_type() {
    #[derive(Debug, Eq, PartialEq)]
    struct NoHash;

    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    struct X<T>(#[eq(key = $.0, bound(T))] P<T>);

    assert_impl!(Eq, X<u32>);
    assert_impl!(Eq, X<NoHash>);
    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<NoHash>);
}

#[test]
fn hash_ord_bound_type() {
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct NoHash;

    #[derive(Debug)]
    struct P<T>(T, T);

    #[derive(Debug)]
    #[derive_ex(Hash, Ord, PartialOrd, Eq, PartialEq)]
    struct X<T>(#[ord(key = $.0, bound(T))] P<T>);

    assert_impl!(Ord, X<u32>);
    assert_impl!(Ord, X<NoHash>);
    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<NoHash>);
}

#[test]
fn derive_macro() {
    #[derive(Debug, derive_ex::Ex)]
    #[derive_ex(Hash)]
    struct X(#[hash(key = $.len())] String);

    assert_eq_hash(X("ABC".into()), X("DEF".into()));
}
