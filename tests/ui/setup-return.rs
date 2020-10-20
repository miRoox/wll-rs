#![allow(clippy::unit_arg)]
use wll::{ErrorKind, Result};

#[wll::setup]
fn setup() -> Result<()> {
    Err(ErrorKind::FunctionError.into())
}

#[wll::teardown]
fn teardown() {}

fn main() {}
