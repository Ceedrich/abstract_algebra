mod fields;
mod rings;

pub use fields::*;
pub use rings::*;

use crate::ops::{Associativity, BinOp, Commutativity, Identity, Invertible, OperationKind};

pub trait Monoid<Op: OperationKind>: BinOp<Op> + Associativity<Op> + Identity<Op> {}
impl<Op, T> Monoid<Op> for T
where
    Op: OperationKind,
    T: BinOp<Op> + Associativity<Op> + Identity<Op>,
{
}

pub trait Group<Op: OperationKind>: Monoid<Op> + Invertible<Op> {
    fn pow(&self, n: usize) -> Self {
        match n {
            0 => Self::id(),
            _ => self.pow(n - 1).op(&self),
        }
    }
}
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

// ------------- Fields ---------------
