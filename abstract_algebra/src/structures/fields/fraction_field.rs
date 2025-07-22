use std::fmt::Display;

use crate::{
    ops::{Addition, Associativity, BinOp, Commutativity, Identity, Invertible, Multiplication},
    primitives::Integer,
    structures::IntegralRing,
};

pub type Rationals = FractionField<Integer>;

#[derive(Debug, Clone, Copy)]
pub struct FractionField<T: IntegralRing> {
    p: T,
    q: T,
}

impl<T: IntegralRing> FractionField<T> {
    /// # Panics
    /// if q == T::zero()
    fn new(p: impl Into<T>, q: impl Into<T>) -> Self {
        let (p, q) = (p.into(), q.into());
        assert_ne!(q, T::zero());
        Self { p, q }
    }
}

impl<T: IntegralRing> BinOp<Addition> for FractionField<T> {
    fn op(&self, rhs: &Self) -> Self {
        let Self { p: a, q: b } = self;
        let Self { p, q } = rhs;
        Self::new(
            a.mul(q).add(&p.mul(b)), //
            b.mul(q),
        )
    }
}

impl<T: IntegralRing> BinOp<Multiplication> for FractionField<T> {
    fn op(&self, rhs: &Self) -> Self {
        let Self { p, q } = self;
        let Self { p: r, q: s } = rhs;

        Self::new(p.mul(r), q.mul(s))
    }
}

impl<T: IntegralRing> Identity<Addition> for FractionField<T> {
    fn id() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl<T: IntegralRing> Identity<Multiplication> for FractionField<T> {
    fn id() -> Self {
        Self::new(T::one(), T::one())
    }
}

impl<T: IntegralRing> Invertible<Addition> for FractionField<T> {
    fn inv(&self) -> Self {
        let Self { p, q } = self;
        Self::new(p.neg(), q.clone())
    }
}

impl<T: IntegralRing> Invertible<Multiplication> for FractionField<T> {
    fn inv(&self) -> Self {
        let Self { p, q } = self;
        Self::new(q.clone(), p.clone())
    }
}

impl<T: IntegralRing> Associativity<Addition> for FractionField<T> {}
impl<T: IntegralRing> Associativity<Multiplication> for FractionField<T> {}
impl<T: IntegralRing> Commutativity<Addition> for FractionField<T> {}
impl<T: IntegralRing> Commutativity<Multiplication> for FractionField<T> {}

impl<T: IntegralRing> PartialEq for FractionField<T> {
    fn eq(&self, other: &Self) -> bool {
        let Self { p, q } = self;
        let Self { p: r, q: s } = other;
        p.mul(s) == q.mul(r)
    }
}
impl<T: IntegralRing> Eq for FractionField<T> {}

impl<T> core::fmt::Display for FractionField<T>
where
    T: IntegralRing + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { p, q } = self;
        write!(f, "{p}/{q}")
    }
}

#[cfg(test)]
mod test {
    use crate::structures::{Field, Ring};

    use super::*;

    #[test]
    fn fraction_field() {
        fn _f<T: IntegralRing>(p: T, q: T) -> impl Field {
            FractionField { p, q }
        }
    }

    #[test]
    fn rationals() {
        let x = Rationals::new(4, 5);
        let y = Rationals::new(1, 5);
        assert_eq!(x.add(&y), Rationals::one())
    }
}
