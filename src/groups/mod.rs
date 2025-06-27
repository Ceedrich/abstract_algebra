mod cyclic;
mod dihedral;
mod free;
mod symmetric;

pub use cyclic::*;
pub use dihedral::*;
pub use free::*;
pub use symmetric::*;

pub trait GroupElement
where
    Self: Sized + Eq,
{
    type G: Group<Self>;
    fn op(&self, rhs: &Self) -> Self {
        Self::G::op(self, rhs)
    }

    fn inverse(&self) -> Self {
        Self::G::inverse(self)
    }
    fn identity() -> Self {
        Self::G::identity()
    }

    fn pow(&self, p: usize) -> Self {
        let mut out = Self::identity();
        for _ in 0..p {
            out = out.op(self);
        }
        out
    }
}

pub trait Group<E>
where
    Self: Sized,
    E: GroupElement,
{
    fn identity() -> E;
    fn op(x: &E, y: &E) -> E;
    fn inverse(x: &E) -> E;
}
