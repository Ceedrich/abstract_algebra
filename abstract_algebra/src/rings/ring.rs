use abstract_algebra_macros::{Blub, Operations};

use crate::ops::{Addition, Associative, BinaryOperation, Commutative, Identity, Invertible, Multiplication, OperationCommutativity};

use super::{Factorability, Integrality, RingOperation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Blub, Operations)]
#[blub(
    accessor(.e: E), 
    bin_op(
        BinaryOperation<Addition, Commutative, Associative>, 
        BinaryOperation<Multiplication, C, Associative>,
    ),
    inv(Invertible<Addition>),
    id(Identity<Addition>, Identity<Multiplication>),
    ring(RingOperation<C, I, F>)
)]
#[operations("Multiplication", "Addition")]
pub struct Ring<E, C, I, F>
where
    E: RingOperation<C, I, F>,
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
    e: E,
    _commutativity: C,
    _integrality: I,
    _factorability: F,
}

impl<E, C, I, F> Ring<E, C, I, F>
where
    E: RingOperation<C, I, F>,
    C: OperationCommutativity,
    I: Integrality,
    F: Factorability,
{
    pub fn new(value: E) -> Self {
        Self {
            e: value,
            _commutativity: Default::default(),
            _integrality: Default::default(),
            _factorability: Default::default(),
        }
    }

    pub fn one() -> Self {
        Self::new(E::one())
    }

    pub fn zero() -> Self {
        Self::new(E::zero())
    }
}

#[cfg(test)]
mod test {
    use crate::{ops::OperationCommutativity, rings::{Factorability, Integrality, RingOperation, Z}};

    #[test]
    fn blub() {
        fn test<T, C, I, F>(_op: T) where 
            T: RingOperation<C, I, F>,
            C: OperationCommutativity,
            I: Integrality,
            F: Factorability,
        { }

        test(Z::new(2.into()));

        let two: Z = Z::new(2.into());
        let four = Z::new(4.into());

        assert_eq!(two + two, four);
    }
}
