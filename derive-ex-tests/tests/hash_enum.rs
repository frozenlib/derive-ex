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
fn hash_enum_unit_variant() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X {
        A,
        B,
    }
    assert_impl!(Hash, X);
    assert_eq_hash(X::A, X::A);
    assert_eq_hash(X::B, X::B);
    assert_eq_hash(X::A, X::B);
}

#[test]
fn hash_enum_tuple_variant() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X {
        A(),
        B(u8),
        C(u8, String),
    }
    assert_impl!(Hash, X);

    assert_eq_hash(X::A(), X::A());
    assert_eq_hash(X::B(1), X::B(1));
    assert_eq_hash(X::C(1, "A".into()), X::C(1, "A".into()));
}

#[test]
fn hash_enum_record_variant() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X {
        A {},
        B { a: u8 },
        C { a: u8, b: String },
    }
    assert_impl!(Hash, X);

    assert_eq_hash(X::A {}, X::A {});
    assert_eq_hash(X::B { a: 1 }, X::B { a: 1 });
    assert_eq_hash(
        X::C {
            a: 1,
            b: "A".into(),
        },
        X::C {
            a: 1,
            b: "A".into(),
        },
    );
}

#[test]
fn hash_hash_key() {
    #[derive(Debug)]
    #[derive_ex(Hash)]
    enum X {
        A(#[hash(key = $.len())] String),
        B(String),
    }
    assert_impl!(Hash, X);

    assert_eq_hash(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq_hash(X::B("ABC".into()), X::B("ABC".into()));
}

#[test]
fn hash_eq_key() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X {
        A(#[eq(key = $.len())] String),
        B(String),
    }
    assert_impl!(Hash, X);

    assert_eq_hash(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq_hash(X::B("ABC".into()), X::B("ABC".into()));
}

#[test]
fn hash_ord_key() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X {
        A(#[ord(key = $.len())] String),
        B(String),
    }
    assert_impl!(Hash, X);

    assert_eq_hash(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq_hash(X::B("ABC".into()), X::B("ABC".into()));
}

#[test]
fn hash_hash_by() {
    #[derive(Debug)]
    #[derive_ex(Hash)]
    enum X {
        A(#[hash(by = |this, state| this.len().hash(state))] String),
        B(String),
    }
    assert_impl!(Hash, X);

    assert_eq_hash(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq_hash(X::B("ABC".into()), X::B("ABC".into()));
}

#[test]
fn hash_hash_ignore() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X {
        A(#[hash(ignore)] String),
        B(String),
    }
    assert_impl!(Hash, X);

    assert_eq_hash(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq_hash(X::B("ABC".into()), X::B("ABC".into()));

    assert_eq_hash(X::A("A".into()), X::A("ABC".into()));
}

#[test]
fn hash_eq_ignore() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X {
        A(#[eq(ignore)] String),
        B(String),
    }
    assert_impl!(Hash, X);

    assert_eq_hash(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq_hash(X::B("ABC".into()), X::B("ABC".into()));

    assert_eq_hash(X::A("A".into()), X::A("ABC".into()));
}

#[test]
fn hash_ord_ignore() {
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X {
        A(#[ord(ignore)] String),
        B(String),
    }
    assert_impl!(Hash, X);

    assert_eq_hash(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq_hash(X::B("ABC".into()), X::B("ABC".into()));

    assert_eq_hash(X::A("A".into()), X::A("ABC".into()));
}

#[test]
fn hash_auto_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X<T> {
        A(T),
        B(String),
    }

    #[derive(Eq, PartialEq)]
    struct NotHash;

    assert_impl!(Eq, X<u32>);
    assert_impl!(Eq, X<NotHash>);

    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<NotHash>);
}

#[test]
fn hash_hash_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X<T> {
        A(#[hash(bound(T : Copy + Hash))] T),
        B(String),
    }

    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<String>);
}

#[test]
fn hash_eq_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X<T> {
        A(#[eq(bound(T : Copy + Hash + Eq))] T),
        B(String),
    }

    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<String>);
}

#[test]
fn hash_ord_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(Hash, Eq, PartialEq)]
    enum X<T> {
        A(#[ord(bound(T : Copy + Hash + Eq))] T),
        B(String),
    }

    assert_impl!(Hash, X<u32>);
    assert_impl!(!Hash, X<String>);
}
