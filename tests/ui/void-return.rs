#![allow(clippy::unit_arg)]
use wll::Result;

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export]
fn print(n: usize) -> Result<()> {
    eprintln!("Input: {}", n);
    Ok(())
}

fn main() {}
