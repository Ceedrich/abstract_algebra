use crate::ops::{
    Addition, Associativity, BinOp, Commutativity, Identity, Invertible, Multiplication,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Natural(usize);

impl From<usize> for Natural {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Associativity<Multiplication> for Natural {}
impl Associativity<Addition> for Natural {}
impl Commutativity<Multiplication> for Natural {}
impl Commutativity<Addition> for Natural {}

impl BinOp<Addition> for Natural {
    fn op(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl BinOp<Multiplication> for Natural {
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Integer(isize);

impl From<isize> for Integer {
    fn from(value: isize) -> Self {
        Self(value)
    }
}

impl Associativity<Addition> for Integer {}
impl Associativity<Multiplication> for Integer {}
impl Commutativity<Addition> for Integer {}
impl Commutativity<Multiplication> for Integer {}

impl BinOp<Addition> for Integer {
    fn op(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl BinOp<Multiplication> for Integer {
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
        ops::{Addition, Multiplication},
        structures::{Monoid, Ring},
    };

    use super::{Integer, Natural};

    #[test]
    fn monoid() {
        fn _f() -> impl Monoid<Multiplication> {
            Natural(1)
        }
        fn _g() -> impl Monoid<Addition> {
            Natural(1)
        }

        let one = Natural(1);
        let two = Natural(2);
        assert_eq!(one * one, one);
        assert_eq!(one + one, two);
        assert_eq!(two * two, two + two);
    }

    #[test]
    fn ring() {
        fn _f() -> impl Ring {
            Integer(1)
        }
    }
}
