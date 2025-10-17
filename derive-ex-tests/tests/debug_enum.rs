use derive_ex::derive_ex;
use derive_ex_tests::assert_debug_eq;

#[test]
fn unit() {
    let a = {
        #[allow(unused)]
        #[derive_ex(Debug)]
        enum X {
            A,
            B,
        }
        X::A
    };
    let e = {
        #[allow(unused)]
        #[derive(Debug)]
        enum X {
            A,
            B,
        }
        X::A
    };
    assert_debug_eq(a, e);
}

#[test]
fn _struct() {
    let a = {
        #[allow(unused)]
        #[derive_ex(Debug)]
        enum X {
            A { x: u32, y: u32 },
            B { x: u32 },
        }
        X::A { x: 1, y: 2 }
    };
    let e = {
        #[allow(unused)]
        #[derive(Debug)]
        enum X {
            A { x: u32, y: u32 },
            B { x: u32 },
        }
        X::A { x: 1, y: 2 }
    };
    assert_debug_eq(a, e);
}

#[test]
fn tuple() {
    let a = {
        #[allow(unused)]
        #[derive_ex(Debug)]
        enum X {
            A(u32, u32),
            B(u32),
        }
        X::A(1, 2)
    };
    let e = {
        #[allow(unused)]
        #[derive(Debug)]
        enum X {
            A(u32, u32),
            B(u32),
        }
        X::A(1, 2)
    };
    assert_debug_eq(a, e);
}

#[test]
fn skip() {
    let a = {
        #[allow(unused)]
        #[derive_ex(Debug)]
        enum X {
            A {
                #[debug(skip)]
                x: u32,
                y: u32,
            },
            B {
                x: u32,
            },
        }
        X::A { x: 1, y: 2 }
    };
    let e = {
        #[allow(unused)]
        enum X {
            A { x: u32, y: u32 },
            B { x: u32 },
        }
        impl std::fmt::Debug for X {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    X::A { y, .. } => f.debug_struct("A").field("y", y).finish(),
                    X::B { .. } => unreachable!(),
                }
            }
        }
        X::A { x: 1, y: 2 }
    };
    assert_debug_eq(a, e);
}

#[test]
fn transparent() {
    let a = {
        #[allow(unused)]
        #[derive_ex(Debug)]
        enum X {
            A(#[debug(transparent)] u32),
            B(u32),
        }
        X::A(1)
    };
    let e = 1;
    assert_debug_eq(a, e);
}
