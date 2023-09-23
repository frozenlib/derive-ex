use derive_ex::derive_ex;

#[derive_ex(Eq, PartialEq)]
struct X {
    #[eq(key = $.0)]
    x: (f64, f64),
}

fn main() {}
