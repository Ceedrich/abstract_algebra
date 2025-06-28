use std::ops::{Add, Mul};

use crate::{
    group::GroupOperation,
    ops::{
        Addition, BinaryOperation, Commutative, Identity, Multiplication, OperationCommutativity,
    },
    primitives::CyclicNumber,
};

pub type Z<const N: usize> = Ring<CyclicNumber<N>, Commutative>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ring<E, C>
where
    E: RingOperation<C>,
    C: OperationCommutativity,
{
    e: E,
    _commutativity: C,
}

impl<E, C> Ring<E, C>
where
    E: RingOperation<C>,
    C: OperationCommutativity,
{
    pub fn new(value: E) -> Self {
        Self {
            e: value,
            _commutativity: Default::default(),
        }
    }

    pub fn one() -> Self {
        let one = <E as Identity<Multiplication>>::id();
        Self::new(one)
    }

    pub fn zero() -> Self {
        let zero = <E as Identity<Addition>>::id();
        Self::new(zero)
    }
}

impl<E, C> Add for Ring<E, C>
where
    E: RingOperation<C>,
    C: OperationCommutativity,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let sum = <E as BinaryOperation<Addition, Commutative>>::op(&self.e, &rhs.e);
        Self::new(sum)
    }
}

impl<E, C> Mul for Ring<E, C>
where
    E: RingOperation<C>,
    C: OperationCommutativity,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let prod = <E as BinaryOperation<Multiplication, C>>::op(&self.e, &rhs.e);
        Self::new(prod)
    }
}

pub trait RingOperation<C>:
    GroupOperation<Addition, Commutative>
    + BinaryOperation<Multiplication, C>
    + Identity<Multiplication>
where
    C: OperationCommutativity,
{
}

impl<E, C> RingOperation<C> for E
where
    E: GroupOperation<Addition, Commutative>
        + BinaryOperation<Multiplication, C>
        + Identity<Multiplication>,
    C: OperationCommutativity,
{
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integer_ring() {
        let one: Z<4> = Z::one();
        let zero = Z::zero();

        assert_eq!(one + one + one + one, zero);
        assert_eq!(one * zero, zero);
        assert_eq!(one + zero, one);
    }
}
