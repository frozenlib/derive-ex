use derive_ex::derive_ex;

#[derive(PartialOrd, Eq, PartialEq)]
struct NotOrd;

#[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
struct X {
    #[ord(key = NotOrd)]
    x: u8,
}

fn main() {}
