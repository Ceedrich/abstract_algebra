use crate::{
    fields::FractionField,
    groups::GroupOperation,
    monoid::MonoidOperation,
    ops::{
        Addition, BinaryOperation, Commutative, Identity, Multiplication, OperationCommutativity,
    },
    primitives::Integer,
    private::{Marker, Seal},
};

pub type Z = Ring<Integer, Integral, UFD>;
pub type Q = Ring<FractionField<Integer, UFD>, Integral, UFD>;

mod ring;
pub use ring::*;

pub trait RingOperation<C, I, F>:
    GroupOperation<Addition, Commutative> + MonoidOperation<Multiplication, C>
where
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
    fn zero() -> Self {
        <Self as Identity<Addition>>::id()
    }

    fn one() -> Self {
        <Self as Identity<Multiplication>>::id()
    }

    fn add(&self, rhs: &Self) -> Self {
        <Self as BinaryOperation<Addition, Commutative, _>>::op(self, rhs)
    }

    fn mul(&self, rhs: &Self) -> Self {
        <Self as BinaryOperation<Multiplication, C, _>>::op(self, rhs)
    }
}

impl<E, C, I, F> RingOperation<C, I, F> for E
where
    E: GroupOperation<Addition, Commutative> + MonoidOperation<Multiplication, C>,
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
}

pub trait Integrality: Seal + Marker {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Integral;
impl Seal for Integral {}
impl Marker for Integral {}
impl Integrality for Integral {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NonIntegral;
impl Seal for NonIntegral {}
impl Marker for NonIntegral {}
impl Integrality for NonIntegral {}

pub trait Factorability: Seal + Marker {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UniqueFactorisationDomain;
impl Seal for UniqueFactorisationDomain {}
impl Marker for UniqueFactorisationDomain {}
impl Factorability for UniqueFactorisationDomain {}
pub type UFD = UniqueFactorisationDomain;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NonUniqueFactorisationDomain;
impl Seal for NonUniqueFactorisationDomain {}
impl Marker for NonUniqueFactorisationDomain {}
impl Factorability for NonUniqueFactorisationDomain {}
pub type NonUFD = UniqueFactorisationDomain;
