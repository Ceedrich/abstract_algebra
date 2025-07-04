use abstract_algebra_macros::Operations;

use crate::ops::{
    Associative, BinaryOperation, Identity, Invertible, Multiplication, NonCommutative,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Operations)]
#[operations("Multiplication")]
pub struct DihedralElement<const N: usize> {
    flipped: bool,
    rotation: usize,
}

impl<const N: usize> DihedralElement<N> {
    pub fn new(rotation: usize, flipped: bool) -> Self {
        Self { flipped, rotation }
    }
}

impl<const N: usize> BinaryOperation<Multiplication, NonCommutative, Associative>
    for DihedralElement<N>
{
    fn op(&self, rhs: &Self) -> Self {
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
