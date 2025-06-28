use std::ops::{Add, AddAssign, Mul, MulAssign};

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

    pub fn pow(&self, n: isize) -> Self {
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

impl<E> Mul<isize> for &Group<E, Addition, Commutative>
where
    E: GroupOperation<Addition, Commutative>,
{
    type Output = Group<E, Addition, Commutative>;
    fn mul(self, rhs: isize) -> Self::Output {
        self.pow(rhs)
    }
}

impl<E> Add<&Group<E, Addition, Commutative>> for &Group<E, Addition, Commutative>
where
    E: GroupOperation<Addition, Commutative>,
{
    type Output = Group<E, Addition, Commutative>;
    fn add(self, rhs: &Group<E, Addition, Commutative>) -> Self::Output {
        let sum = BinaryOperation::<Addition, Commutative, Associative>::op(&self.e, &rhs.e);
        Group::new(sum)
    }
}

impl<E> Add for Group<E, Addition, Commutative>
where
    E: GroupOperation<Addition, Commutative>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<E> Add<Group<E, Addition, Commutative>> for &Group<E, Addition, Commutative>
where
    E: GroupOperation<Addition, Commutative>,
{
    type Output = Group<E, Addition, Commutative>;
    fn add(self, rhs: Group<E, Addition, Commutative>) -> Self::Output {
        self + &rhs
    }
}

impl<E> Add<&Group<E, Addition, Commutative>> for Group<E, Addition, Commutative>
where
    E: GroupOperation<Addition, Commutative>,
{
    type Output = Group<E, Addition, Commutative>;
    fn add(self, rhs: &Group<E, Addition, Commutative>) -> Self::Output {
        &self + rhs
    }
}

impl<E> AddAssign<&Group<E, Addition, Commutative>> for Group<E, Addition, Commutative>
where
    E: GroupOperation<Addition, Commutative>,
{
    fn add_assign(&mut self, rhs: &Group<E, Addition, Commutative>) {
        *self = &*self + rhs;
    }
}

impl<E> AddAssign<Group<E, Addition, Commutative>> for Group<E, Addition, Commutative>
where
    E: GroupOperation<Addition, Commutative>,
{
    fn add_assign(&mut self, rhs: Group<E, Addition, Commutative>) {
        *self = &*self + rhs;
    }
}

impl<E, C> Mul<&Group<E, Multiplication, C>> for &Group<E, Multiplication, C>
where
    E: GroupOperation<Multiplication, C>,
    C: OperationCommutativity,
{
    type Output = Group<E, Multiplication, C>;
    fn mul(self, rhs: &Group<E, Multiplication, C>) -> Self::Output {
        let sum = BinaryOperation::<Multiplication, C, Associative>::op(&self.e, &rhs.e);
        Group::new(sum)
    }
}

impl<E, C> Mul for Group<E, Multiplication, C>
where
    E: GroupOperation<Multiplication, C>,
    C: OperationCommutativity,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<E, C> Mul<Group<E, Multiplication, C>> for &Group<E, Multiplication, C>
where
    E: GroupOperation<Multiplication, C>,
    C: OperationCommutativity,
{
    type Output = Group<E, Multiplication, C>;
    fn mul(self, rhs: Group<E, Multiplication, C>) -> Self::Output {
        self * &rhs
    }
}

impl<E, C> Mul<&Group<E, Multiplication, C>> for Group<E, Multiplication, C>
where
    E: GroupOperation<Multiplication, C>,
    C: OperationCommutativity,
{
    type Output = Group<E, Multiplication, C>;
    fn mul(self, rhs: &Group<E, Multiplication, C>) -> Self::Output {
        &self * rhs
    }
}

impl<E, C> MulAssign<&Group<E, Multiplication, C>> for Group<E, Multiplication, C>
where
    E: GroupOperation<Multiplication, C>,
    C: OperationCommutativity,
{
    fn mul_assign(&mut self, rhs: &Group<E, Multiplication, C>) {
        *self = &*self * rhs;
    }
}

impl<E, C> MulAssign<Group<E, Multiplication, C>> for Group<E, Multiplication, C>
where
    E: GroupOperation<Multiplication, C>,
    C: OperationCommutativity,
{
    fn mul_assign(&mut self, rhs: Group<E, Multiplication, C>) {
        *self = &*self * rhs;
    }
}
