use crate::{
    groups::{C, CMult},
    ops::{
        Addition, Associative, BinaryOperation, Commutative, Identity, Invertible, Multiplication,
    },
    utils::is_prime,
};

impl<const N: usize> From<usize> for CMult<N> {
    fn from(value: usize) -> Self {
        Self::new(CyclicNumber(value % N))
    }
}

impl<const N: usize> From<usize> for C<N> {
    fn from(value: usize) -> Self {
        Self::new(CyclicNumber(value % N))
    }
}

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
        Self((self.0 * rhs.0) % N)
    }
}

impl<const N: usize> Identity<Multiplication> for CyclicNumber<N> {
    fn id() -> Self {
        Self(1)
    }
}

impl<const N: usize> BinaryOperation<Addition, Commutative, Associative> for CyclicNumber<N> {
    fn op(&self, rhs: &Self) -> Self {
        Self((self.0 + rhs.0) % N)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn addition() {
        let id: C<4> = 0.into();
        let x: C<4> = 4.into();
        let y: C<4> = 3.into();
        let z: C<4> = 2.into();

        assert_eq!(x, id);
        assert_eq!(x + y, 3.into());
        assert_eq!(y + z, 1.into())
    }

    #[test]
    fn multiplication() {
        let id: CMult<3> = CMult::id();
        let two: CMult<3> = 2.into();
        assert_eq!(id.inv(), id);
        assert_eq!(two.inv(), two);

        let id: CMult<5> = CMult::id();
        let two: CMult<5> = 2.into();
        let three = 3.into();

        assert_eq!(two.inv(), three);
        assert_eq!(three.inv(), two);
        assert_eq!(two * three, id);

        let a: CMult<97> = 17.into();
        let b = 40.into();

        assert_eq!(a.inv(), b);
    }
}
