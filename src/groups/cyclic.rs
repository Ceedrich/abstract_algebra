use std::ops::{Add, Mul};

use super::*;

impl<const N: usize> Mul for C<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.op(&rhs)
    }
}

impl<const N: usize> Add for C<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.op(&rhs)
    }
}

pub type C<const N: usize> = CyclicGroupElement<N>;

pub struct CyclicGroup<const N: usize>;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CyclicGroupElement<const N: usize>(usize);

impl<const N: usize> From<usize> for CyclicGroupElement<N> {
    fn from(value: usize) -> Self {
        Self(value % N)
    }
}

impl<const N: usize> GroupElement for CyclicGroupElement<N> {
    type G = CyclicGroup<N>;
}
impl<const N: usize> Group<CyclicGroupElement<N>> for CyclicGroup<N> {
    fn inverse(x: &CyclicGroupElement<N>) -> CyclicGroupElement<N> {
        CyclicGroupElement((N - x.0) % N)
    }
    fn op(x: &CyclicGroupElement<N>, y: &CyclicGroupElement<N>) -> CyclicGroupElement<N> {
        CyclicGroupElement((x.0 + y.0) % N)
    }
    fn identity() -> CyclicGroupElement<N> {
        CyclicGroupElement(0)
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
        assert_eq!(y.op(&z), 1.into())
    }
}
