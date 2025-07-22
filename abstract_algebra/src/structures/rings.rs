use crate::ops::{Addition, BinOp, Commutativity, Identity, Invertible, Multiplication};

use super::{Group, Monoid};

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
