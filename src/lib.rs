//! This crate is an abstract algebra crate allowing you to easily use different predefined groups
//! or implement your own groups using the provided traits
//!
//! # Example Usage
//!
//! ```rust
//! use abstract_algebra::primitives::Permutation;
//! use abstract_algebra::perm;
//! use abstract_algebra::ops::{Identity, Invertible};
//!
//! let id: Permutation<3> = Permutation::id();
//! // Empty cycles are considered to be the identity
//! assert_eq!(id, perm![()]);
//! // Single values are also considered to be the identity
//! assert_eq!(id, perm![(3)]);
//! // You can combine cycles
//! assert_eq!(id, perm![(1 2)(1 2)]);
//! // If the group is ambiguous, you can specify the size of the symmetric group
//! assert_eq!(perm![3; (1 2 3)], perm![(1 2)(2 3)]);
//! // Group Operations are also implemented
//! assert_eq!(perm![3; (1 2)] * perm![(2 3)], perm![(1 2 3)] );
//! assert_eq!(perm![3; (1 2 3)].inv(), perm![(1 3 2)] );
//! ```
//!
//! # Group Types
//!
//! The following group types exist:
//! - [symmetric](group::S): Macro to create elements: `perm![(1 2)(3 4)]`
//! - [cyclic](group::C)
//! - [free](group::F): Macro to create elements: `word!['a' 'b' 'a'- 'b' 'a']`

pub(crate) mod private {
    pub trait Seal {}
}

pub mod groups;
pub mod ops;
pub mod primitives;
pub mod rings;
mod utils;
