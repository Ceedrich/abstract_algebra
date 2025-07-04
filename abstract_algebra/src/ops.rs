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
