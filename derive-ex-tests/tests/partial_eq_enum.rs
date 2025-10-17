use derive_ex::derive_ex;
use derive_ex_tests::assert_impl;

#[test]
fn partial_eq_empty() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {}

    #[allow(dead_code)]
    fn f(l: X, r: X) -> bool {
        l == r
    }
}

#[test]
fn partial_eq_enum_unit_variant() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A,
        B,
    }
    assert_eq!(X::A, X::A);
    assert_eq!(X::B, X::B);
    assert_ne!(X::A, X::B);
}

#[test]
fn partial_eq_enum_tuple_variant() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(),
        B(u8),
        C(u8, String),
    }
    assert_eq!(X::A(), X::A());
    assert_eq!(X::B(1), X::B(1));
    assert_eq!(X::C(1, "A".into()), X::C(1, "A".into()));
    assert_ne!(X::A(), X::B(1));
    assert_ne!(X::B(1), X::B(2));
}

#[test]
fn partial_eq_enum_record_variant() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A {},
        B { a: u8 },
        C { a: u8, b: String },
    }
    assert_eq!(X::A {}, X::A {});
    assert_eq!(X::B { a: 1 }, X::B { a: 1 });
    assert_eq!(
        X::C {
            a: 1,
            b: "A".into()
        },
        X::C {
            a: 1,
            b: "A".into()
        }
    );
    assert_ne!(X::A {}, X::B { a: 1 });
    assert_ne!(X::B { a: 1 }, X::B { a: 2 });
    assert_ne!(
        X::C {
            a: 1,
            b: "A".into()
        },
        X::C {
            a: 1,
            b: "B".into()
        }
    );
    assert_ne!(
        X::C {
            a: 1,
            b: "A".into()
        },
        X::C {
            a: 2,
            b: "A".into()
        }
    );
}

#[test]
fn partial_eq_partial_eq_key() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[partial_eq(key = $.len())] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_ne!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_eq_key() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[eq(key = $.len())] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_ne!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_partial_ord_key() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[partial_ord(key = $.len())] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_ne!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_ord_key() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[ord(key = $.len())] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_ne!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_partial_eq_by() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[partial_eq(by = |this, other| this.len() == other.len())] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_ne!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_eq_by() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[eq(by = |this, other| this.len() == other.len())] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_ne!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_partial_ord_by() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[partial_ord(by = |this, other| this.len().partial_cmp(&other.len()))] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_ne!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_ord_by() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[ord(by = |this, other| this.len().cmp(&other.len()))] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_ne!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_partial_eq_skip() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[partial_eq(skip)] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_eq!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_eq_skip() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[eq(skip)] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_eq!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_partial_ord_skip() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[partial_ord(skip)] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_eq!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_ord_skip() {
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X {
        A(#[ord(skip)] String),
        B(String),
    }
    assert_eq!(X::A("ABC".into()), X::A("DEF".into()));
    assert_eq!(X::B("ABC".into()), X::B("ABC".into()));

    assert_eq!(X::A("A".into()), X::A("ABC".into()));
    assert_ne!(X::B("ABC".into()), X::B("DEF".into()));
    assert_ne!(X::A("ABC".into()), X::B("ABC".into()));
}

#[test]
fn partial_eq_auto_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X<T> {
        A(T),
        B(String),
    }

    struct NotPartialEq;

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(!PartialEq, X<NotPartialEq>);
}

#[test]
fn partial_eq_partial_eq_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X<T> {
        A(#[partial_eq(bound(T : Copy + PartialEq))] T),
        B(String),
    }

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_eq_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X<T> {
        A(#[eq(bound(T : Copy + PartialEq))] T),
        B(String),
    }

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_partial_ord_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X<T> {
        A(#[partial_ord(bound(T : Copy + PartialEq))] T),
        B(String),
    }

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_ord_bound() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X<T> {
        A(#[ord(bound(T : Copy + PartialEq))] T),
        B(String),
    }

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_partial_eq_bound_type_at_field() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X<T> {
        A(#[partial_eq(bound(T : Copy + PartialEq))] T),
        B(String),
    }

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_partial_eq_bound_type_at_variant() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X<T> {
        #[partial_eq(bound(T : Copy + PartialEq))]
        A(T),
        B(String),
    }

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_partial_eq_bound_type_at_type() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    #[partial_eq(bound(T : Copy + PartialEq))]
    enum X<T> {
        A(T),
        B(String),
    }

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_eq_bound_type_at_field() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X<T> {
        A(#[eq(bound(T : Copy + PartialEq))] T),
        B(String),
    }

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_eq_bound_type_at_variant() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    enum X<T> {
        #[eq(bound(T : Copy + PartialEq))]
        A(T),
        B(String),
    }

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(!PartialEq, X<String>);
}

#[test]
fn partial_eq_eq_bound_type_at_type() {
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive_ex(PartialEq)]
    #[eq(bound(T : Copy + PartialEq))]
    enum X<T> {
        A(T),
        B(String),
    }

    assert_impl!(PartialEq, X<u32>);
    assert_impl!(PartialEq, X<f64>);
    assert_impl!(!PartialEq, X<String>);
}
