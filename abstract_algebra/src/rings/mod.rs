use crate::{
    fields::FractionField,
    groups::GroupOperation,
    monoid::MonoidOperation,
    ops::{
        Addition, BinaryOperation, Commutative, Identity, Invertible, Multiplication,
        OperationCommutativity,
    },
    primitives::Integer,
    private::{Marker, Seal},
};

pub type Z = Ring<Integer, Commutative, Integral, UFD>;
pub type Q = Ring<FractionField<Integer, UFD>, Commutative, Integral, UFD>;

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

    fn neg(&self) -> Self {
        <Self as Invertible<Addition>>::inv(self)
    }

    fn add(&self, rhs: &Self) -> Self {
        <Self as BinaryOperation<Addition, Commutative, _>>::op(self, rhs)
    }

    fn mul(&self, rhs: &Self) -> Self {
        <Self as BinaryOperation<Multiplication, C, _>>::op(self, rhs)
    }
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

#[cfg(test)]
pub fn test_ring_axioms<E, C, I, F>(elems: &[E])
where
    E: RingOperation<C, I, F>,
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
    for elems in elems.windows(3) {
        let a = &elems[0];
        let b = &elems[1];
        let c = &elems[2];

        assert_eq!(
            a.mul(&b.add(c)),
            a.mul(b).add(&a.mul(c)),
            "a(b + c) = ab + ac"
        );
        assert_eq!(
            a.add(b).mul(c),
            a.mul(c).add(&b.mul(c)),
            "(a + b)c = ac + bc"
        );
    }
}

#[cfg(test)]
pub fn test_integrability<E, C, F>(non_zero_elems: &[E])
where
    E: RingOperation<C, Integral, F> + core::fmt::Debug,
    C: OperationCommutativity,
    F: Factorability,
{
    for e in non_zero_elems {
        if e == &E::zero() {
            panic!("Only non-zero elements should be passed into this function, given: {e:?}")
        }
    }
    for elems in non_zero_elems.windows(2) {
        let a = &elems[0];
        let b = &elems[0];
        assert_ne!(a.mul(b), E::zero());
        assert_ne!(b.mul(a), E::zero());
    }
}
