use std::ops::Add;

use crate::{
    ops::{
        Addition, Associative, BinaryOperation, Commutative, Identity, Invertible, Multiplication,
        NonCommutative, OperationCommutativity, OperationKind,
    },
    primitives::{CyclicNumber, Permutation, Word},
};

pub type C<const N: usize> = Group<CyclicNumber<N>, Addition, Commutative>;
pub type S<const N: usize> = Group<Permutation<N>, Multiplication, NonCommutative>;
pub type F<T> = Group<Word<T>, Multiplication, NonCommutative>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Group<E, K, C> {
    e: E,
    _kind: K,
    _commutativity: C,
}

impl<E, K, C> Group<E, K, C>
where
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
}

impl<E, K, C> Group<E, K, C>
where
    E: BinaryOperation<K, C, Associative> + Identity<K> + Invertible<K>,
    K: OperationKind,
    C: OperationCommutativity,
{
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

impl<E, C> Add for Group<E, Addition, C>
where
    E: BinaryOperation<Addition, C, Associative> + Identity<Addition> + Invertible<Addition>,
    C: OperationCommutativity,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.op(&rhs)
    }
}
