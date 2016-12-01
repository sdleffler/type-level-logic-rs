#![feature(trace_macros)]
#![cfg_attr(feature = "specialization", feature(specialization))]

#[macro_use]
extern crate type_operators;

pub mod types;
pub mod strong;

#[cfg(not(feature = "specialization"))]
pub use strong::*;

#[cfg(feature = "specialization")]
pub mod weak;
