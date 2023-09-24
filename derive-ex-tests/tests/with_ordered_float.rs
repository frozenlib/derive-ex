use derive_ex::derive_ex;
use ordered_float::OrderedFloat;

#[test]
fn impl_ord() {
    #[derive_ex(Ord, PartialOrd, PartialEq, Eq)]
    struct X {
        #[ord(key = OrderedFloat($))]
        f: f64,
    }
}

#[test]
fn impl_hash() {
    #[derive_ex(Hash, PartialEq, Eq)]
    struct X {
        #[eq(key = OrderedFloat($))]
        f: f64,
    }
}
