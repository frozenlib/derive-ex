use derive_ex::derive_ex;

#[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
struct X {
    #[ord(by = ())]
    #[partial_ord(key = $)]
    #[eq(key = $)]
    x: u8,
}

fn main() {}
