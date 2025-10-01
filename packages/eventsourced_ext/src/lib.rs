#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
// TODO: #![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]
#![doc = include_str!("../README.md")]

mod test;

pub use test::{TestFramework, TestFrameworkResult};
