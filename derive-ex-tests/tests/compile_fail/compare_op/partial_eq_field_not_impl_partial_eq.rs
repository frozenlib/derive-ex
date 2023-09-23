use derive_ex::derive_ex;

struct NotPartialEq;

#[derive_ex(PartialEq)]
struct X {
    x: NotPartialEq,
}

fn main() {}
