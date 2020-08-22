use wll::Result;
use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

#[export(factorial)]
fn fac(n: usize) -> Result<usize> {
    Ok(if n == 0 { 1 } else { n * fac(n - 1)? })
}

fn main() {}
