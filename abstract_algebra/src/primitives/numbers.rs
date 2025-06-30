use std::ops::{Add, AddAssign};

use crate::ops::{Addition, BinaryOperation, Commutative, Identity, Invertible, Multiplication};

pub struct Natural(usize);

impl From<usize> for Natural {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl BinaryOperation<Addition, Commutative> for Natural {
    fn op(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl BinaryOperation<Multiplication, Commutative> for Natural {
    fn op(&self, rhs: &Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Identity<Addition> for Natural {
    fn id() -> Self {
        Self(0)
    }
}

impl Identity<Multiplication> for Natural {
    fn id() -> Self {
        Self(1)
    }
}

impl Add<Self> for &Natural {
    type Output = Natural;
    fn add(self, rhs: Self) -> Self::Output {
        Natural(self.0 + rhs.0)
    }
}

pub struct Integer(isize);

impl BinaryOperation<Addition, Commutative> for Integer {
    fn op(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl BinaryOperation<Multiplication, Commutative> for Integer {
    fn op(&self, rhs: &Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Invertible<Addition> for Integer {
    fn inv(&self) -> Self {
        Self(-self.0)
    }
}

impl Identity<Addition> for Integer {
    fn id() -> Self {
        Self(0)
    }
}

impl Identity<Multiplication> for Integer {
    fn id() -> Self {
        Self(1)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        monoid::Monoid,
        ops::{Addition, Commutative, Multiplication},
        rings::Ring,
    };

    use super::{Integer, Natural};

    #[test]
    fn monoid() {
        let _: Monoid<Natural, Multiplication, Commutative> = Monoid::new(Natural(1));
        let _: Monoid<Natural, Addition, Commutative> = Monoid::new(Natural(1));
    }

    #[test]
    fn ring() {
        let _: Ring<Integer, Commutative> = Ring::new(Integer(1));
    }
}
