//! The `types` module contains the base types for type-level primitives, as well as the "kind"
//! traits, which are used to emulate the idea of a kind (a "type-of-types".) The type operators
//! themselves are kept in the `strong` module, in preparation for the eventual support of both
//! "weak" (SFINAE-style) and "strong" (trait bounds *always* required) type operators.

pub mod boolean;
pub mod ternary;
pub mod balanced;
