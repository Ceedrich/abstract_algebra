use abstract_algebra_macros::{Blub, Operations};

use crate::{
    groups::GroupOperation,
    ops::{
        Addition, Associative, BinaryOperation, Commutative, Identity, Invertible, Multiplication,
    },
    rings::{Integral, RingOperation, UFD},
};

pub trait FieldOperation:
    GroupOperation<Multiplication, Commutative> + GroupOperation<Addition, Commutative>
{
}

impl<E> FieldOperation for E where
    E: GroupOperation<Multiplication, Commutative> + GroupOperation<Addition, Commutative>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Blub, Operations)]
#[operations("Multiplication", "Addition")]
#[blub(
    accessor(.0: E),
    bin_op(
        BinaryOperation<Addition, Commutative, Associative>,
        BinaryOperation<Multiplication, Commutative, Associative>,
    ),
    inv(Invertible<Addition>, Invertible<Multiplication>),
    id(Identity<Addition>, Identity<Multiplication>),
    ring(RingOperation<Commutative, Integral, UFD>)
)]
pub struct Field<E>(E)
where
    E: FieldOperation;

impl<E> Field<E>
where
    E: FieldOperation,
{
    pub fn new(value: E) -> Self {
        Self(value)
    }

    pub fn one() -> Self {
        let one = <E as Identity<Multiplication>>::id();
        Self::new(one)
    }

    pub fn zero() -> Self {
        let zero = <E as Identity<Addition>>::id();
        Self::new(zero)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        fields::{Field, FieldOperation},
        ops::OperationCommutativity,
        primitives::CyclicNumber,
        rings::{Factorability, Integrality, RingOperation},
    };

    #[test]
    fn field_ops() {
        fn field<T: FieldOperation>(_x: T) {}
        fn ring<T, C, I, F>(_x: T)
        where
            T: RingOperation<C, I, F>,
            C: OperationCommutativity,
            I: Integrality,
            F: Factorability,
        {
        }

        let x: Field<CyclicNumber<5>> = Field::new(2.into());

        field(x);
        ring(x);
    }

    #[test]
    fn operations() {
        let five: Field<CyclicNumber<5>> = Field::new(5.into());
        let two: Field<CyclicNumber<5>> = Field::new(2.into());
        let three: Field<CyclicNumber<5>> = Field::new(3.into());

        assert_eq!(two * five, three * five);
        assert_eq!(two + two + two + two, three);
    }
}
