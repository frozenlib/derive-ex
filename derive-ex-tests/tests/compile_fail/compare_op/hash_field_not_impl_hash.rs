use derive_ex::derive_ex;

#[derive(Eq, PartialEq)]
struct NoHash;

#[derive_ex(Hash, Eq, PartialEq)]
struct X {
    x: NoHash,
}

fn main() {}
