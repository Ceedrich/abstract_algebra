use std::ops::{Add, AddAssign, Mul, MulAssign};

use abstract_algebra_macros::Blub;

use crate::{
    impl_op, impl_op_assign,
    ops::{
        Addition, Associative, BinaryOperation, Commutative, Identity, Invertible, Multiplication,
        NonCommutative, OperationCommutativity, OperationKind,
    },
    primitives::{CyclicNumber, Permutation, Word},
};

pub type Cyclic<const N: usize> = Group<CyclicNumber<N>, Addition, Commutative>;
pub type Symmetric<const N: usize> = Group<Permutation<N>, Multiplication, NonCommutative>;
pub type Free<T> = Group<Word<T>, Multiplication, NonCommutative>;

pub trait GroupOperation<K, C>:
    BinaryOperation<K, C, Associative> + Identity<K> + Invertible<K>
where
    K: OperationKind,
    C: OperationCommutativity,
{
    fn pow(&self, n: isize) -> Self {
        let mut x = Self::id();
        if n > 0 {
            for _ in 0..n {
                x = x.op(self);
            }
        } else if n < 0 {
            let inv = self.inv();
            for _ in 0..n {
                x = x.op(&inv)
            }
        }
        x
    }
}

impl<E, K, C> GroupOperation<K, C> for E
where
    E: BinaryOperation<K, C, Associative> + Identity<K> + Invertible<K>,
    K: OperationKind,
    C: OperationCommutativity,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Blub)]
#[blub(
    accessor(.e : E),
    bin_op(BinaryOperation<K, C, Associative>),
    inv(Invertible<K>),
    id(Identity<K>),
)]
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

    pub fn pow(&self, n: isize) -> Self {
        Self::new(self.e.pow(n))
    }
}

impl<E> Mul<isize> for &Group<E, Addition, Commutative>
where
    E: GroupOperation<Addition, Commutative>,
{
    type Output = Group<E, Addition, Commutative>;
    fn mul(self, rhs: isize) -> Self::Output {
        self.pow(rhs)
    }
}

impl<E> Add<Self> for &Group<E, Addition, Commutative>
where
    E: GroupOperation<Addition, Commutative>,
{
    type Output = Group<E, Addition, Commutative>;
    fn add(self, rhs: Self) -> Self::Output {
        let sum = BinaryOperation::<Addition, Commutative, Associative>::op(&self.e, &rhs.e);
        Group::new(sum)
    }
}

impl_op! { impl<E> Add ; add : Group<E, Addition, Commutative> ; where E: GroupOperation<Addition, Commutative> }
impl_op_assign! { impl<E> AddAssign ; add ; add_assign : Group<E, Addition, Commutative> ; where E: GroupOperation<Addition, Commutative> }

impl<E, C> Mul<Self> for &Group<E, Multiplication, C>
where
    E: GroupOperation<Multiplication, C>,
    C: OperationCommutativity,
{
    type Output = Group<E, Multiplication, C>;
    fn mul(self, rhs: Self) -> Self::Output {
        let sum = BinaryOperation::<Multiplication, C, Associative>::op(&self.e, &rhs.e);
        Group::new(sum)
    }
}

impl_op! { impl<E, C> Mul ; mul : Group<E, Multiplication, C> ; where E: GroupOperation<Multiplication, C>, C: OperationCommutativity }
impl_op_assign! { impl<E, C> MulAssign ; mul ; mul_assign: Group<E, Multiplication, C> ; where E: GroupOperation<Multiplication, C>, C: OperationCommutativity }

#[cfg(test)]
pub fn test_group_axioms<G, K, C>(elems: &[G])
where
    G: GroupOperation<K, C>,
    K: OperationKind,
    C: OperationCommutativity,
{
    for elems in elems.windows(2) {
        let a = &elems[0];
        let b = &elems[1];
        assert_eq!(
            a.op(b).inv(),
            b.inv().op(&a.inv()),
            "(ab)^(-1) = b^(-1)a^(-1)"
        );
        assert_eq!(
            b.op(a).inv(),
            a.inv().op(&b.inv()),
            "(ba)^(-1) = a^(-1)b^(-1)"
        );
    }

    for a in elems {
        assert_eq!(a.op(&a.inv()), G::id(), "a * a^(-1) = id");
        assert_eq!(a.inv().op(a), G::id(), "a^(-1) * a = id");
    }
}
