use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DihedralGroupElement<const N: usize> {
    flipped: bool,
    rotation: usize,
}

pub type D<const N: usize> = Group<DihedralGroupElement<N>>;

impl<const N: usize> GroupElement for DihedralGroupElement<N> {
    fn id() -> Self {
        Self {
            flipped: false,
            rotation: 0,
        }
    }
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
    fn op(&self, y: &Self) -> Self {
        if y.flipped {
            Self {
                flipped: self.flipped ^ y.flipped,
                rotation: (N + y.rotation - self.rotation) % N,
            }
        } else {
            Self {
                flipped: self.flipped,
                rotation: (self.rotation + y.rotation) % N,
            }
        }
    }
}

impl<const N: usize> D<N> {
    pub fn new(flipped: bool, rotation: usize) -> Self {
        Self(DihedralGroupElement { flipped, rotation })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn dihedral_group() {
        let id = D::<5>::id();
        let tau = D::<5>::new(true, 0);
        let sigma = D::<5>::new(false, 1);
        assert_eq!(sigma.pow(3), sigma * sigma * sigma);
        assert_eq!(tau * tau, id);
        assert_eq!(sigma.pow(5), id);
        assert_eq!(tau * sigma * tau, sigma.inv());
        assert_eq!(tau * sigma.pow(3) * tau, sigma.inv().pow(3));
    }
}
