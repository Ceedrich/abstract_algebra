use crate::{
    groups::GroupOperation,
    monoid::MonoidOperation,
    ops::{Addition, Commutative, Multiplication, OperationCommutativity},
    private::Seal,
};

mod ring;
pub use ring::*;

pub trait RingOperation<C, I, F>:
    GroupOperation<Addition, Commutative> + MonoidOperation<Multiplication, C>
where
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
}

impl<E, C, I, F> RingOperation<C, I, F> for E
where
    E: GroupOperation<Addition, Commutative> + MonoidOperation<Multiplication, C>,
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
}

pub trait Integrality: Seal + Default {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Integral;
impl Seal for Integral {}
impl Integrality for Integral {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NonIntegral;
impl Seal for NonIntegral {}
impl Integrality for NonIntegral {}

pub trait Factorability: Seal + Default {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UniqueFactorisationDomain;
impl Seal for UniqueFactorisationDomain {}
impl Factorability for UniqueFactorisationDomain {}
pub type UFD = UniqueFactorisationDomain;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NonUniqueFactorisationDomain;
impl Seal for NonUniqueFactorisationDomain {}
impl Factorability for NonUniqueFactorisationDomain {}
pub type NonUFD = UniqueFactorisationDomain;
