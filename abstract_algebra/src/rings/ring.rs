use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::{impl_op, impl_op_assign, ops::OperationCommutativity};

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
        Self::new(E::one())
    }

    pub fn zero() -> Self {
        Self::new(E::zero())
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
        Ring::new(self.e.mul(&rhs.e))
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
        Ring::new(self.e.add(&rhs.e))
    }
}

impl_op!(impl<E, C, I, F> Add ; add : Ring<E, C, I, F> ; where E: RingOperation<C, I, F>, C: OperationCommutativity, I: Integrality, F: Factorability);
impl_op!(impl<E, C, I, F> Mul ; mul : Ring<E, C, I, F> ; where E: RingOperation<C, I, F>, C: OperationCommutativity, I: Integrality, F: Factorability);

impl_op_assign!(impl<E, C, I, F> AddAssign ; add ; add_assign : Ring<E, C, I, F> ; where E: RingOperation<C, I, F>, C: OperationCommutativity, I: Integrality, F: Factorability);
impl_op_assign!(impl<E, C, I, F> MulAssign ; mul ; mul_assign : Ring<E, C, I, F> ; where E: RingOperation<C, I, F>, C: OperationCommutativity, I: Integrality, F: Factorability);
