use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::{
    groups::GroupOperation,
    impl_op, impl_op_assign,
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

impl<E, C> Mul<Self> for &Ring<E, C>
where
    E: RingOperation<C>,
    C: OperationCommutativity,
{
    type Output = Ring<E, C>;
    fn mul(self, rhs: Self) -> Self::Output {
        let prod = <E as BinaryOperation<Multiplication, C>>::op(&self.e, &rhs.e);
        Ring::new(prod)
    }
}
impl<E, C> Add<Self> for &Ring<E, C>
where
    E: RingOperation<C>,
    C: OperationCommutativity,
{
    type Output = Ring<E, C>;
    fn add(self, rhs: Self) -> Self::Output {
        let sum = <E as BinaryOperation<Addition, Commutative>>::op(&self.e, &rhs.e);
        Ring::new(sum)
    }
}

impl_op!(impl<E, C> Add ; add : Ring<E, C> ; where E: RingOperation<C>, C: OperationCommutativity);
impl_op!(impl<E, C> Mul ; mul : Ring<E, C> ; where E: RingOperation<C>, C: OperationCommutativity);

impl_op_assign!(impl<E, C> AddAssign ; add ; add_assign : Ring<E, C> ; where E: RingOperation<C>, C: OperationCommutativity);
impl_op_assign!(impl<E, C> MulAssign ; mul ; mul_assign : Ring<E, C> ; where E: RingOperation<C>, C: OperationCommutativity);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integer_ring() {
        let one: Z<4> = Z::one();
        let zero = Z::zero();

        let two = one + one;

        assert_eq!(two + one, one + one + one);
        assert_eq!(one + one + two, zero);
        assert_eq!(one * zero, zero);
        assert_eq!(one + zero, one);
    }
}
