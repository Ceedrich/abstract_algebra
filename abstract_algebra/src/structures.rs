use crate::ops::{
    Addition, Associativity, BinOp, Commutativity, Identity, Invertible, Multiplication,
    OperationKind,
};

pub trait Monoid<Op: OperationKind>: BinOp<Op> + Associativity<Op> + Identity<Op> {}
impl<Op, T> Monoid<Op> for T
where
    Op: OperationKind,
    T: BinOp<Op> + Associativity<Op> + Identity<Op>,
{
}

pub trait Group<Op: OperationKind>: Monoid<Op> + Invertible<Op> {}
impl<Op, T> Group<Op> for T
where
    Op: OperationKind,
    T: Monoid<Op> + Invertible<Op>,
{
}

pub trait AbelianGroup<Op: OperationKind>: Group<Op> + Commutativity<Op> {}
impl<Op, T> AbelianGroup<Op> for T
where
    Op: OperationKind,
    T: Group<Op> + Commutativity<Op>,
{
}

// --------------- Rings ----------------
pub trait Integrality: Ring {}

pub trait Ring: Group<Addition> + Monoid<Multiplication> {
    fn one() -> Self {
        <Self as Identity<Multiplication>>::id()
    }
    fn zero() -> Self {
        <Self as Identity<Addition>>::id()
    }
    fn add(&self, rhs: &Self) -> Self {
        <Self as BinOp<Addition>>::op(self, rhs)
    }
    fn mul(&self, rhs: &Self) -> Self {
        <Self as BinOp<Multiplication>>::op(self, rhs)
    }
    fn neg(&self) -> Self {
        <Self as Invertible<Addition>>::inv(self)
    }
}
impl<T> Ring for T where T: Group<Addition> + Monoid<Multiplication> {}

pub trait CommutativeRing: Ring + Commutativity<Multiplication> {}
impl<T> CommutativeRing for T where T: Ring + Commutativity<Multiplication> {}

pub trait IntegralRing: Ring + Integrality {}
impl<T> IntegralRing for T where T: Ring + Integrality {}

// WARN: 1 != 0 must be checked as well
pub trait Field: Ring + Invertible<Multiplication> {}
impl<T> Field for T where T: Ring + Invertible<Multiplication> {}
impl<T> Integrality for T where T: Field {}
