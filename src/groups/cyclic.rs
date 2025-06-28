use super::*;

pub type C<const N: usize> = Group<CyclicGroupElement<N>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CyclicGroupElement<const N: usize>(usize);

impl<const N: usize> CyclicGroupElement<N> {
    fn new(value: usize) -> Self {
        Self(value % N)
    }
}

impl<const N: usize> From<usize> for C<N> {
    fn from(value: usize) -> Self {
        Self(CyclicGroupElement::new(value))
    }
}

impl<const N: usize> BinaryOperation for CyclicGroupElement<N> {
    fn op(&self, y: &Self) -> Self {
        Self((self.0 + y.0) % N)
    }
}

impl<const N: usize> GroupElement for CyclicGroupElement<N> {
    fn inv(&self) -> Self {
        Self((N - self.0) % N)
    }
    fn id() -> CyclicGroupElement<N> {
        Self(0)
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
}
