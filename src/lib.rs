//! # wll
//!
//! Wolfram [LibraryLink] interface for Rust.
//!
//! [LibraryLink]: http://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html

#[cfg(feature = "macros")]
#[no_link]
#[allow(unused_imports)]
#[macro_use]
extern crate wll_macros;
extern crate wll_sys;

pub use complex::Complex;
pub use errors::{Error, ErrorKind};
#[cfg(feature = "macros")]
pub use wll_macros::*;

pub mod adaptor;
pub mod global;

mod complex;
mod errors;

/// A specialized `std::result::Result` type for wll functions.
pub type Result<T> = std::result::Result<T, Error>;
