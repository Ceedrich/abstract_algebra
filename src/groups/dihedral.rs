use std::ops::Mul;

use super::{Group, GroupElement};

impl<const N: usize> Mul for D<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.op(&rhs)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DihedralGroupElement<const N: usize> {
    flipped: bool,
    rotation: usize,
}

type D<const N: usize> = DihedralGroupElement<N>;

pub struct DihedralGroup<const N: usize>;

impl<const N: usize> GroupElement for D<N> {
    type G = DihedralGroup<N>;
}

impl<const N: usize> D<N> {
    pub fn new(flipped: bool, rotation: usize) -> Self {
        D { flipped, rotation }
    }
}

impl<const N: usize> Group<D<N>> for DihedralGroup<N> {
    fn identity() -> D<N> {
        DihedralGroupElement {
            flipped: false,
            rotation: 0,
        }
    }
    fn inverse(x: &D<N>) -> D<N> {
        if x.flipped {
            D {
                flipped: true,
                rotation: x.rotation,
            }
        } else {
            D {
                flipped: false,
                rotation: (N - x.rotation) % N,
            }
        }
    }
    fn op(x: &D<N>, y: &D<N>) -> D<N> {
        if y.flipped {
            D {
                flipped: x.flipped ^ y.flipped,
                rotation: (N + y.rotation - x.rotation) % N,
            }
        } else {
            D {
                flipped: x.flipped,
                rotation: (x.rotation + y.rotation) % N,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn dihedral_group() {
        let id = D::<5>::identity();
        let tau = D::<5>::new(true, 0);
        let sigma = D::<5>::new(false, 1);
        assert_eq!(sigma.pow(3), sigma * sigma * sigma);
        assert_eq!(tau * tau, id);
        assert_eq!(sigma.pow(5), id);
        assert_eq!(tau * sigma * tau, sigma.inverse());
        assert_eq!(tau * sigma.pow(3) * tau, sigma.inverse().pow(3));
    }
}

