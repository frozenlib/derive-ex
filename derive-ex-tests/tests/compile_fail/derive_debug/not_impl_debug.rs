fn main() {}

struct NotImplDebug;
#[derive(derive_ex::Ex)]
#[derive_ex(Debug)]
struct X {
    field: NotImplDebug,
}
