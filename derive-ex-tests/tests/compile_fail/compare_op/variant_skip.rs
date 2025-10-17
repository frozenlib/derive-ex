use derive_ex::derive_ex;

#[derive_ex(PartialEq)]
enum X {
    #[partial_eq(ignore)]
    A,
}

fn main() {}
