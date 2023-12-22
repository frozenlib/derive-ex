use derive_ex::derive_ex;

#[derive_ex(Eq, PartialEq)]
struct X(#[partial_ord(by = |l, r| l.len().partial_cmp(&r.len()))] String);

fn main() {}
