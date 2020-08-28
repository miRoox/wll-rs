#![allow(clippy::unit_arg)]
use wll::{Error, ErrorKind, Result};

#[wll::setup]
fn setup() -> Result<()> {
    Err(Error::from(ErrorKind::FunctionError))
}

#[wll::teardown]
fn teardown() {}

fn main() {}
