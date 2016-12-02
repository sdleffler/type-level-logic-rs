//! "Strongly" enforced (non-SFINAE-style) type operators. Here, "strong" means that if you use
//! a type operator from this module, Rust will complain if it can't guarantee an `impl` for it
//! and you haven't listed it as a trait bound in a `where` clause.

pub mod boolean;
pub mod ternary;
pub mod balanced;
