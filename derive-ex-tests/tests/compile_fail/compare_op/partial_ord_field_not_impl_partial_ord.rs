use derive_ex::derive_ex;

#[derive(PartialEq)]
struct NotPartialOrd;

#[derive_ex(PartialOrd, PartialEq)]
struct X {
    x: NotPartialOrd,
}

fn main() {}
