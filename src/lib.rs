//! This crate is an abstract algebra crate allowing you to easily use different predefined groups
//! or implement your own groups using the provided traits
//!
//! # Example Usage
//!
//! Let's say, you need to use a symmetric [group](groups::Group) of order 3:
//! ```rust
//! use abstract_algebra::groups::{S, GroupElement};
//! use abstract_algebra::sym;
//!
//! let id: S<3> = S::identity();
//! // Empty cycles are considered to be the identity
//! assert_eq!(id, sym![()]);
//! // Single values are also considered to be the identity
//! assert_eq!(id, sym![(3)]);
//! // You can combine cycles
//! assert_eq!(id, sym![(1 2)(1 2)]);
//! // If the group is ambiguous, you can specify the size of the symmetric group
//! assert_eq!(sym![3; (1 2 3)], sym![(1 2)(2 3)]);
//! // Group Operations are also implemented
//! assert_eq!(sym![3; (1 2)] * sym![(2 3)], sym![(1 2 3)] );
//! assert_eq!(sym![3; (1 2 3)].inverse(), sym![(1 3 2)] );
//! ```
//!
//! # Group Types
//!
//! The following group types exist:
//! - [symmetric](groups::S): Macro to create elements: `sym![(1 2)(3 4)]`
//! - [cyclic](groups::C)
//! - [free](groups::F): Macro to create elements: `free!['a' 'b' 'a'- 'b' 'a']`

pub mod groups;
