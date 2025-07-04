use abstract_algebra_macros::{Blub, Operations};

use crate::{
    ops::{
        Addition, Associative, BinaryOperation, Commutative, Identity, Invertible, Multiplication,
    },
    rings::{Integral, RingOperation, UFD},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Operations)]
#[operations("Addition", "Multiplication")]
pub struct Natural(usize);

impl From<usize> for Natural {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl BinaryOperation<Addition, Commutative, Associative> for Natural {
    fn op(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl BinaryOperation<Multiplication, Commutative, Associative> for Natural {
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Operations, Blub)]
#[blub(
    accessor(.0: isize),
    ring(RingOperation<Commutative, Integral, UFD>)
)]
#[operations("Addition", "Multiplication")]
pub struct Integer(isize);

impl From<isize> for Integer {
    fn from(value: isize) -> Self {
        Self(value)
    }
}

impl BinaryOperation<Addition, Commutative, Associative> for Integer {
    fn op(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl BinaryOperation<Multiplication, Commutative, Associative> for Integer {
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
        rings::{Integral, Ring, UFD},
    };

    use super::{Integer, Natural};

    #[test]
    fn monoid() {
        let _: Monoid<Natural, Multiplication, Commutative> = Monoid::new(Natural(1));
        let _: Monoid<Natural, Addition, Commutative> = Monoid::new(Natural(1));

        let one = Natural(1);
        let two = Natural(2);
        assert_eq!(one * one, one);
        assert_eq!(one + one, two);
        assert_eq!(two * two, two + two);
    }

    #[test]
    fn ring() {
        let _: Ring<Integer, Commutative, Integral, UFD> = Ring::new(Integer(1));
    }
}
