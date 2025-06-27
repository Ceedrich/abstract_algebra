mod cyclic;
mod dihedral;
mod free;
mod symmetric;

use std::ops::{Add, Deref, DerefMut, Mul};

pub use cyclic::*;
pub use dihedral::*;
pub use free::*;
pub use symmetric::*;

pub trait Abelian {}

#[derive(Debug, PartialEq, Eq)]
pub struct Group<E>(E);

impl<E> Group<E>
where
    E: GroupElement,
{
    pub fn id() -> Self {
        Self(E::id())
    }
    pub fn op(&self, other: &Self) -> Self {
        Self(self.0.op(&other.0))
    }
    pub fn inv(&self) -> Self {
        Self(self.0.inv())
    }
    pub fn pow(&self, p: usize) -> Self {
        let mut out = Self::id();
        for _ in 0..p {
            out = out.op(self);
        }
        out
    }
}

impl<E> Deref for Group<E> {
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<E> DerefMut for Group<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<E> Clone for Group<E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<E> Copy for Group<E> where E: Copy {}

impl<E> Add for Group<E>
where
    E: GroupElement + Abelian,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.op(&rhs.0))
    }
}

impl<E> Mul for Group<E>
where
    E: GroupElement,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.op(&rhs.0))
    }
}

pub trait GroupElement
where
    Self: Sized + Eq,
{
    fn op(&self, rhs: &Self) -> Self;
    fn inv(&self) -> Self;
    fn id() -> Self;
}
