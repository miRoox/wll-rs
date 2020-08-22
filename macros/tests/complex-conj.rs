use wll::complex::Complex;
use wll::Result;
use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

#[export(conj)]
fn cconj(z: Complex<f64>) -> Result<Complex<f64>> {
    Ok(z.conj())
}

fn main() {}
