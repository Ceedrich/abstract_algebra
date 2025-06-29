use std::ops::{Mul, MulAssign};

use crate::{
    impl_op, impl_op_assign,
    ops::{BinaryOperation, Identity, Invertible, Multiplication},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DihedralElement<const N: usize> {
    flipped: bool,
    rotation: usize,
}

impl<const N: usize> DihedralElement<N> {
    pub fn new(rotation: usize, flipped: bool) -> Self {
        Self { flipped, rotation }
    }
}

impl<const N: usize> BinaryOperation<Multiplication> for DihedralElement<N> {
    fn op(&self, y: &Self) -> Self {
        self * y
    }
}

impl<const N: usize> Identity<Multiplication> for DihedralElement<N> {
    fn id() -> Self {
        Self {
            flipped: false,
            rotation: 0,
        }
    }
}

impl<const N: usize> Invertible<Multiplication> for DihedralElement<N> {
    fn inv(&self) -> Self {
        if self.flipped {
            Self {
                flipped: true,
                rotation: self.rotation,
            }
        } else {
            Self {
                flipped: false,
                rotation: (N - self.rotation) % N,
            }
        }
    }
}

impl<const N: usize> Mul<Self> for &DihedralElement<N> {
    type Output = DihedralElement<N>;
    fn mul(self, rhs: Self) -> Self::Output {
        if rhs.flipped {
            DihedralElement {
                flipped: self.flipped ^ rhs.flipped,
                rotation: (N + rhs.rotation - self.rotation) % N,
            }
        } else {
            DihedralElement {
                flipped: self.flipped,
                rotation: (self.rotation + rhs.rotation) % N,
            }
        }
    }
}

impl_op!(impl<const N: usize> Mul ; mul : DihedralElement<N>);
impl_op_assign!(impl<const N: usize> MulAssign ; mul ; mul_assign : DihedralElement<N>);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn dihedral() {
        let id = DihedralElement::<5>::id();
        let tau = DihedralElement::<5>::new(0, true);
        let sigma = DihedralElement::<5>::new(1, false);
        assert_eq!(sigma * sigma * sigma * sigma, sigma.inv());
        assert_eq!(tau * tau, id);
        assert_eq!(tau * sigma * tau, sigma.inv());
    }
}
