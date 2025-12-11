#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod components;
mod pages;
mod route;
mod toast;

pub use pages::UnexpectedErrorPage;
pub use route::Route;
pub use toast::Toast;
