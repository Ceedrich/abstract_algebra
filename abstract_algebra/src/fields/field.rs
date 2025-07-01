use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::{
    groups::GroupOperation,
    impl_op, impl_op_assign,
    ops::{Addition, BinaryOperation, Commutative, Identity, Multiplication},
};

pub trait FieldOperation:
    GroupOperation<Multiplication, Commutative> + GroupOperation<Addition, Commutative>
{
}

impl<E> FieldOperation for E where
    E: GroupOperation<Multiplication, Commutative> + GroupOperation<Addition, Commutative>
{
}

pub struct Field<E>(E)
where
    E: FieldOperation;

impl<E> Field<E>
where
    E: FieldOperation,
{
    pub fn new(value: E) -> Self {
        Self(value)
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

impl<E> Add<Self> for &Field<E>
where
    E: FieldOperation,
{
    type Output = Field<E>;
    fn add(self, rhs: Self) -> Self::Output {
        let sum = <E as BinaryOperation<Addition, Commutative>>::op(&self.0, &rhs.0);
        Field(sum)
    }
}

impl_op!(impl<E> Add ; add : Field<E> ; where E: FieldOperation);
impl_op_assign!(impl<E> AddAssign ; add ; add_assign: Field<E> ; where E: FieldOperation);

impl<E> Mul<Self> for &Field<E>
where
    E: FieldOperation,
{
    type Output = Field<E>;
    fn mul(self, rhs: Self) -> Self::Output {
        let prod = <E as BinaryOperation<Multiplication, Commutative>>::op(&self.0, &rhs.0);
        Field(prod)
    }
}

impl_op!(impl<E> Mul ; mul : Field<E> ; where E: FieldOperation);
impl_op_assign!(impl<E> MulAssign ; mul ; mul_assign: Field<E> ; where E: FieldOperation);
