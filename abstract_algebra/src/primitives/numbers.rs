use crate::{
    ops::{Addition, Associativity, BinOp, Commutativity, Identity, Invertible, Multiplication},
    structures::{FractionField, Integrality},
    wrapper::Wrapper,
};

pub type Natural = Wrapper<usize>;

pub type Rationals = FractionField<Integer>;

impl Associativity<Multiplication> for usize {}
impl Associativity<Addition> for usize {}
impl Commutativity<Multiplication> for usize {}
impl Commutativity<Addition> for usize {}

impl BinOp<Addition> for usize {
    fn op(&self, rhs: &Self) -> Self {
        self + rhs
    }
}

impl BinOp<Multiplication> for usize {
    fn op(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Identity<Addition> for usize {
    fn id() -> Self {
        0
    }
}

impl Identity<Multiplication> for usize {
    fn id() -> Self {
        1
    }
}

pub type Integer = Wrapper<isize>;

impl Associativity<Addition> for isize {}
impl Associativity<Multiplication> for isize {}
impl Commutativity<Addition> for isize {}
impl Commutativity<Multiplication> for isize {}

impl BinOp<Addition> for isize {
    fn op(&self, rhs: &Self) -> Self {
        self + rhs
    }
}

impl BinOp<Multiplication> for isize {
    fn op(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Invertible<Addition> for isize {
    fn inv(&self) -> Self {
        -self
    }
}

impl Identity<Addition> for isize {
    fn id() -> Self {
        0
    }
}

impl Identity<Multiplication> for isize {
    fn id() -> Self {
        1
    }
}

impl Integrality for isize {}

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
            Natural::new(1)
        }
        fn _g() -> impl Monoid<Addition> {
            Natural::new(1)
        }

        let one = Natural::new(1);
        let two = Natural::new(2);
        assert_eq!(one * one, one);
        assert_eq!(one + one, two);
        assert_eq!(two * two, two + two);
    }

    #[test]
    fn ring() {
        fn _f() -> impl Ring {
            Integer::new(1)
        }
    }
}
