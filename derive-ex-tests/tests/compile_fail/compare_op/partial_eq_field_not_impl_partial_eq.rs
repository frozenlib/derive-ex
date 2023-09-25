use derive_ex::derive_ex;

struct NotPartialEq;

#[derive_ex(PartialEq)]
struct X {
    pub x: NotPartialEq,
}

fn main() {}
