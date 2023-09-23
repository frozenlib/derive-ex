use derive_ex::derive_ex;

#[derive_ex(PartialEq)]
struct X {
    #[partial_eq(by = ())]
    x: u8,
}

fn main() {}
