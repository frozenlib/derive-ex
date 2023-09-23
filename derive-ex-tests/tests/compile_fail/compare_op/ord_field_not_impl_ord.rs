use derive_ex::derive_ex;

#[derive(PartialOrd, Eq, PartialEq)]
struct NotOrd;

#[derive_ex(Ord, PartialOrd, Eq, PartialEq)]
struct X {
    x: NotOrd,
}

fn main() {}
