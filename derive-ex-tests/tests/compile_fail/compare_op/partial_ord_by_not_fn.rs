use derive_ex::derive_ex;

#[derive_ex(PartialOrd, PartialEq)]
struct X {
    #[partial_ord(by = ())]
    #[partial_eq(key = $)]
    x: u8,
}

fn main() {}
