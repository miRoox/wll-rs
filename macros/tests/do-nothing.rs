use wll::Result;
use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

#[export(do_nothing)]
fn nothing() -> Result<()> {
    Ok(())
}

fn main() {}
