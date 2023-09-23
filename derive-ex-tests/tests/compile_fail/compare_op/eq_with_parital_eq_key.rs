use derive_ex::derive_ex;

#[derive_ex(Eq, PartialEq)]
struct X(#[partial_eq(key = $.len())] String);

fn main() {}
