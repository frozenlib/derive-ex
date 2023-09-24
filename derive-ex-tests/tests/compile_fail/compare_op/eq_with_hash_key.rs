use derive_ex::derive_ex;

#[derive_ex(Hash, Eq, PartialEq)]
struct X(#[hash(key = $.len())] String);

fn main() {}
