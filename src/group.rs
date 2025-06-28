use std::ops::Add;

use crate::{
    ops::{
        Addition, Associative, BinaryOperation, Commutative, Identity, Invertible, Multiplication,
        NonCommutative, OperationCommutativity, OperationKind,
    },
    primitives::{CyclicNumber, Permutation, Word},
};

pub type CMult<const N: usize> = Group<CyclicNumber<N>, Multiplication, Commutative>;

pub type C<const N: usize> = Group<CyclicNumber<N>, Addition, Commutative>;
pub type S<const N: usize> = Group<Permutation<N>, Multiplication, NonCommutative>;
pub type F<T> = Group<Word<T>, Multiplication, NonCommutative>;

pub trait GroupOperation<K, C>:
    BinaryOperation<K, C, Associative> + Identity<K> + Invertible<K>
where
    K: OperationKind,
    C: OperationCommutativity,
{
}

impl<E, K, C> GroupOperation<K, C> for E
where
    E: BinaryOperation<K, C, Associative> + Identity<K> + Invertible<K>,
    K: OperationKind,
    C: OperationCommutativity,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Group<E, K, C>
where
    E: GroupOperation<K, C>,
    K: OperationKind,
    C: OperationCommutativity,
{
    e: E,
    _kind: K,
    _commutativity: C,
}

impl<E, K, C> Group<E, K, C>
where
    E: GroupOperation<K, C>,
    K: OperationKind,
    C: OperationCommutativity,
{
    pub fn new(e: E) -> Self {
        Self {
            e,
            _kind: Default::default(),
            _commutativity: Default::default(),
        }
    }
    pub fn inv(&self) -> Self {
        Self::new(self.e.inv())
    }

    pub fn op(&self, rhs: &Self) -> Self {
        Self::new(self.e.op(&rhs.e))
    }

    pub fn id() -> Self {
        Self::new(E::id())
    }
}

impl<E> Add for Group<E, Addition, Commutative>
where
    E: GroupOperation<Addition, Commutative>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let sum = BinaryOperation::<Addition, Commutative, Associative>::op(&self.e, &rhs.e);
        Self::new(sum)
    }
}
