use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::{
    impl_op, impl_op_assign,
    ops::{
        Addition, Associative, BinaryOperation, Commutative, Identity, Invertible, Multiplication,
    },
    utils::is_prime,
};

impl<const P: usize> Invertible<Multiplication> for CyclicNumber<P> {
    /// using [Fermat's little theorem](https://en.wikipedia.org/wiki/Fermat%27s_little_theorem)
    fn inv(&self) -> Self {
        const { assert!(is_prime(P), "Inversion only works for prime numbers") }
        let x = self.0;
        if x == 0 {
            panic!("Division by zero");
        }
        let mut base = x % P;
        let mut exp = P - 2;
        let mut result = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % P
            }
            base = (base * base) % P;
            exp /= 2;
        }
        Self(result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CyclicNumber<const N: usize>(usize);

impl<const N: usize> BinaryOperation<Multiplication, Commutative, Associative> for CyclicNumber<N> {
    fn op(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl<const N: usize> Identity<Multiplication> for CyclicNumber<N> {
    fn id() -> Self {
        Self(1)
    }
}

impl<const N: usize> BinaryOperation<Addition, Commutative, Associative> for CyclicNumber<N> {
    fn op(&self, rhs: &Self) -> Self {
        self + rhs
    }
}

impl<const N: usize> Identity<Addition> for CyclicNumber<N> {
    fn id() -> Self {
        Self(0)
    }
}

impl<const N: usize> Invertible<Addition> for CyclicNumber<N> {
    fn inv(&self) -> Self {
        Self((N - self.0) % N)
    }
}

impl<const N: usize> From<usize> for CyclicNumber<N> {
    fn from(value: usize) -> Self {
        Self(value % N)
    }
}

impl<const N: usize> Add<Self> for &CyclicNumber<N> {
    type Output = CyclicNumber<N>;
    fn add(self, rhs: Self) -> Self::Output {
        CyclicNumber((self.0 + rhs.0) % N)
    }
}

impl_op!(impl<const N: usize> Add ; add : CyclicNumber<N>);
impl_op_assign!(impl<const N: usize> AddAssign ; add ; add_assign : CyclicNumber<N>);

impl<const N: usize> Mul<Self> for &CyclicNumber<N> {
    type Output = CyclicNumber<N>;
    fn mul(self, rhs: Self) -> Self::Output {
        CyclicNumber((self.0 * rhs.0) % N)
    }
}

impl_op!(impl<const N: usize> Mul ; mul : CyclicNumber<N>);
impl_op_assign!(impl<const N: usize> MulAssign ; mul ; mul_assign : CyclicNumber<N>);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let zero: CyclicNumber<4> = 0.into();
        let four: CyclicNumber<4> = 4.into();
        let three: CyclicNumber<4> = 3.into();
        let two: CyclicNumber<4> = 2.into();
        let one: CyclicNumber<4> = 1.into();

        assert_eq!(four, zero);
        assert_eq!(two * three, two);
        assert_eq!(two + four, two);
        assert_eq!(three + three, one);
    }
}
