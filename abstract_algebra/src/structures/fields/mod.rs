use crate::ops::{Invertible, Multiplication};

use super::Ring;

mod fraction_field;
pub use fraction_field::*;

// WARN: 1 != 0 must be checked as well
pub trait Field: Ring + Invertible<Multiplication> {}
impl<T> Field for T where T: Ring + Invertible<Multiplication> {}
