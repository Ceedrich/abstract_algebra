use crate::private::{Marker, Seal};

pub trait OperationKind: Seal + Marker {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Multiplication;
impl Seal for Multiplication {}
impl Marker for Multiplication {}
impl OperationKind for Multiplication {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Addition;
impl Seal for Addition {}
impl Marker for Addition {}
impl OperationKind for Addition {}

pub trait BinOp<Op: OperationKind>: Sized + PartialEq + Eq + Clone {
    fn op(&self, rhs: &Self) -> Self;
}
pub trait Associativity<Op: OperationKind>: BinOp<Op> {}
pub trait Commutativity<Op: OperationKind>: BinOp<Op> {}

pub trait Invertible<Op: OperationKind>: BinOp<Op> {
    fn inv(&self) -> Self;
}

pub trait Identity<Op: OperationKind>: BinOp<Op> {
    fn id() -> Self;
}

#[cfg(test)]
pub fn test_accociativity<T, K>(elems: &[T])
where
    T: Associativity<K>,
    K: OperationKind,
{
    for elems in elems.windows(3) {
        let a = &elems[0];
        let b = &elems[1];
        let c = &elems[2];
        assert_eq!(a.op(b).op(c), a.op(&b.op(c)), "(a * b) * c = a * (b * c)");
    }
}

#[cfg(test)]
pub fn test_commutativity<T, K>(elems: &[T])
where
    T: BinOp<K>,
    K: OperationKind,
{
    for elems in elems.windows(2) {
        let a = &elems[0];
        let b = &elems[1];

        assert_eq!(a.op(b), b.op(a), "ab = ba");
    }
}
