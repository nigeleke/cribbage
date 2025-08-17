#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]
#![doc = include_str!("../README.md")]

mod macros;
mod reactor;
mod test;

pub use self::{
    reactor::Reactor,
    test::{TestFramework, TestFrameworkResult},
};
