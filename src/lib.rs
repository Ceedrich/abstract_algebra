//! This crate is an abstract algebra crate allowing you to easily use different predefined groups
//! or implement your own groups using the provided traits
//!
//! # Example Usage
//!
//! Let's say, you need to use a symmetric group of order 3:
//! ```rust
//! use abstract_algebra::groups::{S, GroupElement};
//! use abstract_algebra::sym;
//!
//! let id: S<3> = S::identity();
//! assert_eq!(id, sym![(1 2)(1 2)]);
//! assert_eq!(sym![3; (1 2 3)], sym![(1 2)(2 3)]);
//!
//! ```

pub mod groups;
