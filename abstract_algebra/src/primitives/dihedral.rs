use crate::ops::{Associativity, BinOp, Identity, Invertible, Multiplication};

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
impl<const N: usize> Associativity<Multiplication> for DihedralElement<N> {}

impl<const N: usize> BinOp<Multiplication> for DihedralElement<N> {
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
        assert_eq!(sigma.op(&sigma).op(&sigma).op(&sigma), sigma.inv());
        assert_eq!(tau.op(&tau), id);
        assert_eq!(tau.op(&sigma).op(&tau), sigma.inv());
    }
}
