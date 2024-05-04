use std::collections::HashSet;

use derive_ex::derive_ex;
use ordered_float::OrderedFloat;

#[test]
fn impl_ord() {
    #[derive_ex(Ord, PartialOrd, PartialEq, Eq)]
    struct X {
        #[ord(key = OrderedFloat($))]
        f: f64,
    }

    let x1 = X { f: 1.0 };
    let x2 = X { f: 2.0 };
    assert!(x1 < x2);
}

#[test]
fn impl_hash() {
    #[derive_ex(Hash, PartialEq, Eq)]
    struct X {
        #[eq(key = OrderedFloat($))]
        f: f64,
    }

    let mut s = HashSet::new();
    s.insert(X { f: 1.0 });
    s.insert(X { f: 2.0 });
}
