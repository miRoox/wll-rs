#![allow(clippy::unit_arg)]
use wll::Result;

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export(do_nothing)]
fn nothing() -> Result<()> {
    Ok(())
}

fn main() {}
