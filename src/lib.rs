//! # `type-level-logic`: Primitives for type-level logic in Rust
//!
//! This crate contains some useful primitive types and traits for writing type-level logic in
//! Rust. This includes signed and unsigned numbers implemented as balanced and unbalanced ternary
//! representations, as well as basic boolean logic. Planned features include SFINAE-style removal
//! of trait bounds using specialization.
//!
//! The `type-level-logic` crate forms the base for several other crates, all offering various
//! sorts of static verification. For example, the `tll-iterator` crate offers statically sized
//! iterators, which can be used to construct and convert between statically sized data structures
//! like those offered by the `tll-array` crate.
//!
//! SFINAE-like functionality will eventually be offered under the `weak` module, hence the
//! existence of the `strong` module.
//!
//! `type-level-logic` depends heavily on the [`type-operators`](https://crates.io/type-operators)
//! crate for defining type-level functionality. If you are interested in contributing, modifying,
//! or just plain curious, you should look at the `type-operators` crate to get a good idea of
//! what's going on.
//!
//! If questions are had, I may be found either at my email (which is listed on GitHub) or on the `#rust` IRC, where I go by
//! the nick `sleffy`.

#![cfg_attr(feature = "specialization", feature(specialization))]

#[macro_use]
extern crate type_operators;

pub mod types;
pub mod strong;

pub use strong::*;
