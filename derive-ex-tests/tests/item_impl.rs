use derive_ex::derive_ex;

#[test]
fn add() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Add)]
    impl Add for X {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);
}

#[test]
#[allow(clippy::op_ref)]
fn add_rhs() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Add)]
    impl Add<u32> for X {
        type Output = Self;
        fn add(self, rhs: u32) -> Self::Output {
            X(self.0 + rhs)
        }
    }

    assert_eq!((X(10) + 2).0, 12);
    assert_eq!((&X(10) + 2).0, 12);
    assert_eq!((X(10) + &2).0, 12);
    assert_eq!((&X(10) + &2).0, 12);
}

#[test]
fn add_output() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Add)]
    impl Add for X {
        type Output = u32;
        fn add(self, rhs: Self) -> Self::Output {
            self.0 + rhs.0
        }
    }

    assert_eq!(X(10) + X(2), 12);
    assert_eq!(&X(10) + X(2), 12);
    assert_eq!(X(10) + &X(2), 12);
    assert_eq!(&X(10) + &X(2), 12);
}

#[test]
fn add_full_path() {
    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Add)]
    impl std::ops::Add for X {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);
}

#[test]
fn add_generics() {
    use std::ops::Add;
    #[derive(Clone)]
    struct X<T>(T);

    #[derive_ex(Add)]
    impl<T> Add for X<T>
    where
        T: Add<Output = T> + Clone,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);
}
#[test]
fn add_generics_where_contains_self() {
    use std::ops::Add;
    #[derive(Clone)]
    struct X<T>(T);

    trait MyTrait {}
    impl<T> MyTrait for X<T> {}

    #[derive_ex(Add)]
    impl<T> Add for X<T>
    where
        T: Add<Output = T> + Clone,
        Self: MyTrait,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);
}

#[test]
fn add_by_ref_value() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Add)]
    impl Add<&X> for X {
        type Output = Self;
        fn add(self, rhs: &Self) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);
}

#[test]
fn add_by_value_ref() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Add)]
    impl Add<X> for &X {
        type Output = X;
        fn add(self, rhs: X) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);
}

#[test]
fn add_by_ref_ref() {
    use std::ops::Add;

    struct X(u32);

    #[derive_ex(Add)]
    impl Add for &X {
        type Output = X;
        fn add(self, rhs: Self) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);
}

#[test]
fn add_assign_by_add() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(AddAssign)]
    impl Add for X {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    let mut x = X(10);
    x += X(2);
    assert_eq!(x.0, 12);
}

#[test]
fn add_assign_by_add_ref() {
    use std::ops::Add;

    struct X(u32);

    #[derive_ex(AddAssign)]
    impl Add<X> for &X {
        type Output = X;
        fn add(self, rhs: X) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    let mut x = X(10);
    x += X(2);
    assert_eq!(x.0, 12);
}

#[test]
fn add_assign_by_add_rhs() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(AddAssign)]
    impl Add<u32> for X {
        type Output = Self;
        fn add(self, rhs: u32) -> Self::Output {
            X(self.0 + rhs)
        }
    }

    let mut x = X(10);
    x += 2;
    assert_eq!(x.0, 12);
}

#[test]
fn add_assign_by_add_generics() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X<T>(T);

    #[derive_ex(AddAssign)]
    impl<T> Add for X<T>
    where
        T: Clone + Add<Output = T>,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    let mut x = X(10);
    x += X(2);
    assert_eq!(x.0, 12);
}

#[test]
fn add_add_assign() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Add, AddAssign)]
    impl Add for X {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);

    let mut x = X(10);
    x += X(2);
    assert_eq!(x.0, 12);
}

#[test]
fn add_add_assign_by_value_ref() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Add, AddAssign)]
    impl Add<X> for &X {
        type Output = X;
        fn add(self, rhs: X) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);

    let mut x = X(10);
    x += X(2);
    assert_eq!(x.0, 12);
}

#[test]
fn add_add_assign_by_ref_value() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Add, AddAssign)]
    impl Add<&X> for X {
        type Output = Self;
        fn add(self, rhs: &X) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);

    let mut x = X(10);
    x += X(2);
    assert_eq!(x.0, 12);
}

#[test]
fn add_add_assign_by_ref_ref() {
    use std::ops::Add;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Add, AddAssign)]
    impl Add for &X {
        type Output = X;
        fn add(self, rhs: Self) -> Self::Output {
            X(self.0 + rhs.0)
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
    assert_eq!((&X(10) + X(2)).0, 12);
    assert_eq!((X(10) + &X(2)).0, 12);
    assert_eq!((&X(10) + &X(2)).0, 12);

    let mut x = X(10);
    x += X(2);
    assert_eq!(x.0, 12);
}

#[test]
fn sub() {
    use std::ops::Sub;

    #[derive(Clone)]
    struct X(u32);

    #[derive_ex(Sub)]
    impl Sub for X {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            X(self.0 - rhs.0)
        }
    }

    assert_eq!((X(10) - X(2)).0, 8);
    assert_eq!((&X(10) - X(2)).0, 8);
    assert_eq!((X(10) - &X(2)).0, 8);
    assert_eq!((&X(10) - &X(2)).0, 8);
}

#[test]
fn add_by_add_assign_rhs_value() {
    use std::ops::AddAssign;

    struct X(u32);

    #[derive_ex(Add)]
    impl AddAssign for X {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }

    assert_eq!((X(10) + X(2)).0, 12);
}

#[test]
fn add_by_add_assign_rhs_ref() {
    use std::ops::AddAssign;

    struct X(u32);

    #[derive_ex(Add)]
    impl AddAssign<&X> for X {
        fn add_assign(&mut self, rhs: &X) {
            self.0 += rhs.0;
        }
    }

    assert_eq!((X(10) + &X(2)).0, 12);
}
