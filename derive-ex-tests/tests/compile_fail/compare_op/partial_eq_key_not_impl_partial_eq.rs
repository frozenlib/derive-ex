use derive_ex::derive_ex;

struct NotPartialEq;

#[derive_ex(PartialEq)]
struct X {
    #[partial_eq(key = NotPartialEq)]
    x: u32,
}

fn main() {}
