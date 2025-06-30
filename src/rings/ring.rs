use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::{
    impl_op, impl_op_assign,
    ops::{
        Addition, BinaryOperation, Commutative, Identity, Multiplication, OperationCommutativity,
    },
};

use super::{Factorability, Integrality, NonIntegral, NonUFD, RingOperation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ring<E, C, I = NonIntegral, F = NonUFD>
where
    E: RingOperation<C, I, F>,
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
    e: E,
    _commutativity: C,
    _integrality: I,
    _factorability: F,
}

impl<E, C, I, F> Ring<E, C, I, F>
where
    E: RingOperation<C, I, F>,
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
    pub fn new(value: E) -> Self {
        Self {
            e: value,
            _commutativity: Default::default(),
            _integrality: Default::default(),
            _factorability: Default::default(),
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

impl<E, C, I, F> Mul<Self> for &Ring<E, C, I, F>
where
    E: RingOperation<C, I, F>,
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
    type Output = Ring<E, C, I, F>;
    fn mul(self, rhs: Self) -> Self::Output {
        let prod = <E as BinaryOperation<Multiplication, C>>::op(&self.e, &rhs.e);
        Ring::new(prod)
    }
}
impl<E, C, I, F> Add<Self> for &Ring<E, C, I, F>
where
    E: RingOperation<C, I, F>,
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
    type Output = Ring<E, C, I, F>;
    fn add(self, rhs: Self) -> Self::Output {
        let sum = <E as BinaryOperation<Addition, Commutative>>::op(&self.e, &rhs.e);
        Ring::new(sum)
    }
}

impl_op!(impl<E, C, I, F> Add ; add : Ring<E, C, I, F> ; where E: RingOperation<C, I, F>, C: OperationCommutativity, I: Integrality, F: Factorability);
impl_op!(impl<E, C, I, F> Mul ; mul : Ring<E, C, I, F> ; where E: RingOperation<C, I, F>, C: OperationCommutativity, I: Integrality, F: Factorability);

impl_op_assign!(impl<E, C, I, F> AddAssign ; add ; add_assign : Ring<E, C, I, F> ; where E: RingOperation<C, I, F>, C: OperationCommutativity, I: Integrality, F: Factorability);
impl_op_assign!(impl<E, C, I, F> MulAssign ; mul ; mul_assign : Ring<E, C, I, F> ; where E: RingOperation<C, I, F>, C: OperationCommutativity, I: Integrality, F: Factorability);
