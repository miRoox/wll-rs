#[cfg(not(feature = "no-macros"))]
#[no_link]
pub extern crate wll_macros;
extern crate wll_sys;

pub mod adaptor;
pub mod complex;
pub mod errors;
pub mod global;

/// A specialized `std::result::Result` type for wll functions.
pub type Result<T> = std::result::Result<T, errors::Error>;
