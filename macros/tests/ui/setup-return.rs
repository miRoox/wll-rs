#![allow(clippy::unit_arg)]
use wll::{Error, ErrorKind, Result};
use wll_macros::{setup, teardown};

#[setup]
fn setup() -> Result<()> {
    Err(Error::from(ErrorKind::FunctionError))
}

#[teardown]
fn teardown() {}

fn main() {}
