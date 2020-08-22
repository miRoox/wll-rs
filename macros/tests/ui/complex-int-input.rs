use wll::{Complex, Result};
use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

#[export(conj)]
fn cconj(z: Complex<i64>) -> Result<Complex<i64>> {
    Ok(z.conj())
}

fn main() {}
