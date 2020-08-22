#![allow(clippy::unit_arg)]
use wll::Result;
use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

#[export]
fn print(n: usize) -> Result<()> {
    eprintln!("Input: {}", n);
    Ok(())
}

fn main() {}
