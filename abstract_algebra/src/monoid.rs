use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::{
    impl_op, impl_op_assign,
    ops::{
        Addition, Associative, BinaryOperation, Commutative, Identity, Multiplication,
        OperationCommutativity, OperationKind,
    },
};

pub trait MonoidOperation<K, C>: BinaryOperation<K, C, Associative> + Identity<K>
where
    K: OperationKind,
    C: OperationCommutativity,
{
}

impl<E, K, C> MonoidOperation<K, C> for E
where
    E: BinaryOperation<K, C, Associative> + Identity<K>,
    K: OperationKind,
    C: OperationCommutativity,
{
}

pub struct Monoid<E, K, C> {
    e: E,
    _kind: K,
    _commutativity: C,
}

impl<E, K, C> Monoid<E, K, C>
where
    E: MonoidOperation<K, C>,
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
    pub fn op(&self, rhs: &Self) -> Self {
        Self::new(self.e.op(&rhs.e))
    }

    pub fn id() -> Self {
        Self::new(E::id())
    }

    pub fn pow(&self, n: usize) -> Self {
        let mut x = Self::id();
        for _ in 0..n {
            x = x.op(self);
        }
        x
    }
}

impl<E> Add<Self> for &Monoid<E, Addition, Commutative>
where
    E: MonoidOperation<Addition, Commutative>,
{
    type Output = Monoid<E, Addition, Commutative>;
    fn add(self, rhs: Self) -> Self::Output {
        self.op(rhs)
    }
}

impl_op!(impl<E> Add ; add : Monoid<E, Addition, Commutative> ; where E: MonoidOperation<Addition, Commutative> );
impl_op_assign!(impl<E> AddAssign ; add ; add_assign : Monoid<E, Addition, Commutative> ; where E: MonoidOperation<Addition, Commutative> );

impl<E, C> Mul<Self> for &Monoid<E, Multiplication, C>
where
    E: MonoidOperation<Multiplication, C>,
    C: OperationCommutativity,
{
    type Output = Monoid<E, Multiplication, C>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.op(rhs)
    }
}

impl_op!(impl<E, C> Mul ; mul : Monoid<E, Multiplication, C> ; where E: MonoidOperation<Multiplication, C>, C: OperationCommutativity );
impl_op_assign!(impl<E, C> MulAssign ; mul ; mul_assign : Monoid<E, Multiplication, C> ; where E: MonoidOperation<Multiplication, C>, C: OperationCommutativity );
