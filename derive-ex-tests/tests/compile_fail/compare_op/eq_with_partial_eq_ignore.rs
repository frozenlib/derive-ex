use derive_ex::derive_ex;

#[derive_ex(Eq, PartialEq)]
struct X(#[partial_eq(ignore)] String);

fn main() {}
