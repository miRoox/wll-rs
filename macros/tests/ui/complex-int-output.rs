use wll::complex::Complex;
use wll::Result;
use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

#[export(round)]
fn cround(z: Complex<f64>) -> Result<Complex<i32>> {
    Ok(Complex::new(z.re.round() as i32, z.im.round() as i32))
}

fn main() {}
