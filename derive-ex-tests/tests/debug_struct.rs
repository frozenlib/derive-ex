use derive_ex::derive_ex;
use test_utils_debug::assert_debug_eq;

mod test_utils_debug;

#[test]
fn unit() {
    let a = {
        #[derive_ex(Debug)]
        struct X;
        X
    };
    let e = {
        #[derive(Debug)]
        struct X;
        X
    };
    assert_debug_eq(a, e);
}

#[test]
fn _struct() {
    let a = {
        #[derive_ex(Debug)]
        struct X {
            a: u32,
            b: u32,
        }
        X { a: 1, b: 2 }
    };
    let e = {
        #[allow(unused)]
        #[derive(Debug)]
        struct X {
            a: u32,
            b: u32,
        }
        X { a: 1, b: 2 }
    };
    assert_debug_eq(a, e);
}

#[test]
fn tuple() {
    let a = {
        #[derive_ex(Debug)]
        struct X(u32, u32);
        X(1, 2)
    };
    let e = {
        #[derive(Debug)]
        struct X(u32, u32);
        X(1, 2)
    };
    assert_debug_eq(a, e);
}

#[test]
fn ignore() {
    let a = {
        #[allow(unused)]
        #[derive_ex(Debug)]
        struct X {
            a: u32,
            #[debug(ignore)]
            b: u32,
        }
        X { a: 1, b: 2 }
    };
    let e = {
        #[allow(unused)]
        struct X {
            a: u32,
            b: u32,
        }
        impl std::fmt::Debug for X {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("X").field("a", &self.a).finish()
            }
        }
        X { a: 1, b: 2 }
    };
    assert_debug_eq(a, e);
}

#[test]
fn non_exhaustive() {
    let a = {
        #[derive_ex(Debug)]
        #[non_exhaustive]
        struct X {
            a: u32,
            b: u32,
        }
        X { a: 1, b: 2 }
    };
    let e = {
        #[allow(unused)]
        #[derive(Debug)]
        #[non_exhaustive]
        struct X {
            a: u32,
            b: u32,
        }
        X { a: 1, b: 2 }
    };
    assert_debug_eq(a, e);
}

#[test]
fn transparent_field() {
    #[derive_ex(Debug)]
    struct X(#[debug(transparent)] u32);
    assert_debug_eq(X(1), 1);
}

#[test]
fn transparent_field_struct() {
    #[allow(unused)]
    #[derive(Debug, Clone, Copy)]
    struct A {
        a: u32,
        b: u32,
    }
    let a = A { a: 10, b: 20 };

    #[derive_ex(Debug)]
    struct X(#[debug(transparent)] A);
    assert_debug_eq(X(a), a);
}
