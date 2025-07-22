use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign};

use abstract_algebra_macros::Algebra;

use crate::ops::{Addition, BinOp, Identity, Invertible, Multiplication, OperationKind};

#[derive(Algebra, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Wrapper<T>(pub T);

impl<Op: OperationKind, T: BinOp<Op>> BinOp<Op> for Wrapper<T> {
    fn op(&self, rhs: &Self) -> Self {
        Self(T::op(&self.0, &rhs.0))
    }
}
impl<Op: OperationKind, T: Invertible<Op>> Invertible<Op> for Wrapper<T> {
    fn inv(&self) -> Self {
        Self(self.0.inv())
    }
}

impl<Op: OperationKind, T: Identity<Op>> Identity<Op> for Wrapper<T> {
    fn id() -> Self {
        Self(T::id())
    }
}

impl<T> From<T> for Wrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
impl<T> Wrapper<T> {
    pub fn new(x: T) -> Self {
        Self(x)
    }
}
impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// ---------- Addition ----------

impl<T: BinOp<Addition>> Add<Wrapper<T>> for Wrapper<T> {
    type Output = Wrapper<T>;
    fn add(self, rhs: Wrapper<T>) -> Self::Output {
        Wrapper(T::op(&self.0, &rhs.0))
    }
}

impl<T: BinOp<Addition>> Add<&Wrapper<T>> for Wrapper<T> {
    type Output = Wrapper<T>;
    fn add(self, rhs: &Wrapper<T>) -> Self::Output {
        Wrapper(T::op(&self.0, &rhs.0))
    }
}

impl<T: BinOp<Addition>> Add<Wrapper<T>> for &Wrapper<T> {
    type Output = Wrapper<T>;
    fn add(self, rhs: Wrapper<T>) -> Self::Output {
        Wrapper(T::op(&self.0, &rhs.0))
    }
}

impl<T: BinOp<Addition>> Add<&Wrapper<T>> for &Wrapper<T> {
    type Output = Wrapper<T>;
    fn add(self, rhs: &Wrapper<T>) -> Self::Output {
        Wrapper(T::op(&self.0, &rhs.0))
    }
}

impl<T: BinOp<Addition>> AddAssign<Wrapper<T>> for Wrapper<T> {
    fn add_assign(&mut self, rhs: Wrapper<T>) {
        *self = (&*self).add(&rhs);
    }
}

impl<T: BinOp<Addition>> AddAssign<&Wrapper<T>> for Wrapper<T> {
    fn add_assign(&mut self, rhs: &Wrapper<T>) {
        *self = (&*self).add(rhs);
    }
}

// ---------- Multiplication ----------

impl<T: BinOp<Multiplication>> Mul<Wrapper<T>> for Wrapper<T> {
    type Output = Wrapper<T>;
    fn mul(self, rhs: Wrapper<T>) -> Self::Output {
        Wrapper(T::op(&self.0, &rhs.0))
    }
}

impl<T: BinOp<Multiplication>> Mul<&Wrapper<T>> for Wrapper<T> {
    type Output = Wrapper<T>;
    fn mul(self, rhs: &Wrapper<T>) -> Self::Output {
        Wrapper(T::op(&self.0, &rhs.0))
    }
}

impl<T: BinOp<Multiplication>> Mul<Wrapper<T>> for &Wrapper<T> {
    type Output = Wrapper<T>;
    fn mul(self, rhs: Wrapper<T>) -> Self::Output {
        Wrapper(T::op(&self.0, &rhs.0))
    }
}

impl<T: BinOp<Multiplication>> Mul<&Wrapper<T>> for &Wrapper<T> {
    type Output = Wrapper<T>;
    fn mul(self, rhs: &Wrapper<T>) -> Self::Output {
        Wrapper(T::op(&self.0, &rhs.0))
    }
}

impl<T: BinOp<Multiplication>> MulAssign<Wrapper<T>> for Wrapper<T> {
    fn mul_assign(&mut self, rhs: Wrapper<T>) {
        *self = (&*self).mul(&rhs);
    }
}

impl<T: BinOp<Multiplication>> MulAssign<&Wrapper<T>> for Wrapper<T> {
    fn mul_assign(&mut self, rhs: &Wrapper<T>) {
        *self = (&*self).mul(rhs);
    }
}

#[cfg(test)]
mod test {
    use crate::structures::{Group, Ring};

    use super::*;

    #[test]
    fn test() {
        fn _f() -> impl Group<Addition> {
            Wrapper(32)
        }
        fn _g() -> impl Ring {
            Wrapper(42)
        }
    }
}
