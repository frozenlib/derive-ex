use derive_ex::derive_ex;

#[derive(PartialEq)]
struct NotPartialOrd;

#[derive_ex(PartialOrd, PartialEq)]
struct X {
    #[partial_ord(key = NotPartialOrd)]
    x: u8,
}

fn main() {}
