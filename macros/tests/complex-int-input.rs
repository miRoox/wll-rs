use wll::complex::Complex;
use wll::Result;
use wll_macros::{wll_export, wll_setup, wll_teardown};

#[wll_setup]
fn setup() {}

#[wll_teardown]
fn teardown() {}

#[wll_export(conj)]
fn cconj(z: Complex<i64>) -> Result<Complex<i64>> {
    Ok(z.conj())
}

fn main() {}
