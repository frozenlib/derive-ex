use derive_ex::derive_ex;

#[derive_ex(Default)]
enum X {
    #[default(10)]
    A,
}
fn main() {}
