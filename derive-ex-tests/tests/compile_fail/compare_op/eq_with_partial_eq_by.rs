use derive_ex::derive_ex;

#[derive_ex(Eq, PartialEq)]
struct X(#[partial_eq(by = |l, r| l.len() == r.len())] String);

fn main() {}
