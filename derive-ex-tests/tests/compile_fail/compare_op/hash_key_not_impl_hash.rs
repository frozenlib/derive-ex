use derive_ex::derive_ex;

#[derive(Eq, PartialEq)]
struct NoHash;

#[derive_ex(Hash)]
struct X {
    #[hash(key = NoHash)]
    x: u8,
}

fn main() {}
