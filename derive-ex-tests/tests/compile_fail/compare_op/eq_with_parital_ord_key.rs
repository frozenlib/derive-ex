use derive_ex::derive_ex;

#[derive_ex(Eq, PartialEq)]
struct X(#[partial_ord(key = $.len())] String);

fn main() {}
