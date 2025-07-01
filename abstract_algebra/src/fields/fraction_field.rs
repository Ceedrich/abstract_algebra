use crate::{
    ops::{
        Addition, Associative, BinaryOperation, Commutative, Identity, Invertible, Multiplication,
    },
    rings::{Factorability, Integral, RingOperation},
};

#[derive(Debug, Clone, Copy)]
pub struct FractionField<E, F>
where
    E: RingOperation<Commutative, Integral, F> + PartialEq + Clone,
    F: Factorability,
{
    num: E,
    den: E,
    _factorability: F,
}

impl<E, F> FractionField<E, F>
where
    E: RingOperation<Commutative, Integral, F> + PartialEq + Clone,
    F: Factorability,
{
    pub fn new(p: E, q: E) -> Self {
        assert!(q != E::zero());
        Self {
            num: p,
            den: q,
            _factorability: Default::default(),
        }
    }
}

impl<E, F> PartialEq for FractionField<E, F>
where
    E: RingOperation<Commutative, Integral, F> + PartialEq + Clone,
    F: Factorability,
{
    fn eq(&self, rhs: &Self) -> bool {
        let Self { num: p, den: q, .. } = self;
        let Self { num: r, den: s, .. } = rhs;
        // p/q = r/s <=> ps = qr
        p.mul(s) == q.mul(r)
    }
}

impl<E, F> BinaryOperation<Addition, Commutative, Associative> for FractionField<E, F>
where
    E: RingOperation<Commutative, Integral, F> + PartialEq + Clone,
    F: Factorability,
{
    fn op(&self, rhs: &Self) -> Self {
        let Self { num: p, den: q, .. } = self;
        let Self { num: r, den: s, .. } = rhs;
        Self::new((p.mul(s)).add(&r.mul(q)), q.mul(s))
    }
}

impl<E, F> BinaryOperation<Multiplication, Commutative, Associative> for FractionField<E, F>
where
    E: RingOperation<Commutative, Integral, F> + PartialEq + Clone,
    F: Factorability,
{
    fn op(&self, rhs: &Self) -> Self {
        let Self { num: p, den: q, .. } = self;
        let Self { num: r, den: s, .. } = rhs;
        Self::new(p.mul(r), q.mul(s))
    }
}

impl<E, F> Identity<Multiplication> for FractionField<E, F>
where
    E: RingOperation<Commutative, Integral, F> + PartialEq + Clone,
    F: Factorability,
{
    fn id() -> Self {
        Self::new(E::one(), E::one())
    }
}

impl<E, F> Identity<Addition> for FractionField<E, F>
where
    E: RingOperation<Commutative, Integral, F> + PartialEq + Clone,
    F: Factorability,
{
    fn id() -> Self {
        Self::new(E::zero(), E::one())
    }
}

impl<E, F> Invertible<Addition> for FractionField<E, F>
where
    E: RingOperation<Commutative, Integral, F> + PartialEq + Clone,
    F: Factorability,
{
    fn inv(&self) -> Self {
        let Self { num, den, .. } = self;
        let p = <E as Invertible<Addition>>::inv(num);
        let q = den.clone();
        Self::new(p, q)
    }
}

impl<E, F> Invertible<Multiplication> for FractionField<E, F>
where
    E: RingOperation<Commutative, Integral, F>,
    F: Factorability,
{
    fn inv(&self) -> Self {
        let Self { num, den, .. } = self;
        Self::new(den.clone(), num.clone())
    }
}
