use crate::{
    private::{Marker, Seal},
    utils::MathObject,
};

pub trait OperationAssociativity: Seal + Marker {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Associative;
impl Seal for Associative {}
impl Marker for Associative {}
impl OperationAssociativity for Associative {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NonAssociative;
impl Seal for NonAssociative {}
impl Marker for NonAssociative {}
impl OperationAssociativity for NonAssociative {}

pub trait OperationCommutativity: Seal + Marker {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Commutative;
impl Seal for Commutative {}
impl Marker for Commutative {}
impl OperationCommutativity for Commutative {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NonCommutative;
impl Seal for NonCommutative {}
impl Marker for NonCommutative {}
impl OperationCommutativity for NonCommutative {}

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

pub trait BinaryOperation<
    Type: OperationKind,
    Commutativity: OperationCommutativity,
    Associativity: OperationAssociativity,
>: MathObject
{
    fn op(&self, rhs: &Self) -> Self;
}

pub trait Invertible<Type: OperationKind>: MathObject {
    fn inv(&self) -> Self;
}

pub trait Identity<Type: OperationKind>: MathObject {
    fn id() -> Self;
}

#[cfg(test)]
pub fn test_accociativity<T, K, C>(elems: &[T])
where
    T: BinaryOperation<K, C, Associative>,
    K: OperationKind,
    C: OperationCommutativity,
{
    for elems in elems.windows(3) {
        let a = &elems[0];
        let b = &elems[1];
        let c = &elems[2];
        assert_eq!(a.op(b).op(c), a.op(&b.op(c)), "(a * b) * c = a * (b * c)");
    }
}

#[cfg(test)]
pub fn test_commutativity<T, K, A>(elems: &[T])
where
    T: BinaryOperation<K, Commutative, A>,
    K: OperationKind,
    A: OperationAssociativity,
{
    for elems in elems.windows(2) {
        let a = &elems[0];
        let b = &elems[1];

        assert_eq!(a.op(b), b.op(a), "ab = ba");
    }
}
