use derive_ex::derive_ex;

#[derive_ex(Default)]
enum X {
    #[default]
    A,
    B,
    #[default]
    C,
}
fn main() {}
