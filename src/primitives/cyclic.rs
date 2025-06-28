use crate::{
    group::C,
    ops::{
        Addition, Associative, BinaryOperation, Commutative, Identity, Invertible, Multiplication,
    },
};

impl<const N: usize> From<usize> for C<N> {
    fn from(value: usize) -> Self {
        Self::new(CyclicNumber(value % N))
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
    use crate::group::C;

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
}
