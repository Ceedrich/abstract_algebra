use crate::private::Seal;

pub trait OperationAssociativity: Seal {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Associative;
impl Seal for Associative {}
impl OperationAssociativity for Associative {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NonAssociative;
impl Seal for NonAssociative {}
impl OperationAssociativity for NonAssociative {}

pub trait OperationCommutativity: Seal {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Commutative;
impl Seal for Commutative {}
impl OperationCommutativity for Commutative {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NonCommutative;
impl Seal for NonCommutative {}
impl OperationCommutativity for NonCommutative {}

pub trait OperationKind: Seal {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Multiplication;
impl Seal for Multiplication {}
impl OperationKind for Multiplication {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Addition;
impl Seal for Addition {}
impl OperationKind for Addition {}

pub trait BinaryOperation<
    Type: OperationKind,
    Commutativity: OperationCommutativity = NonCommutative,
    Associativity: OperationAssociativity = Associative,
>
{
    fn op(&self, rhs: &Self) -> Self;
}

pub trait Invertible<Type: OperationKind> {
    fn inv(&self) -> Self;
}

pub trait Identity<Type: OperationKind> {
    fn id() -> Self;
}
