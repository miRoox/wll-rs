use wll::complex::Complex;
use wll::Result;
use wll_macros::{wll_export, wll_setup, wll_teardown};

#[wll_setup]
fn setup() {}

#[wll_teardown]
fn teardown() {}

#[wll_export(round)]
fn cround(z: Complex<f64>) -> Result<Complex<i32>> {
    Ok(Complex::new(z.re.round() as i32, z.im.round() as i32))
}

fn main() {}
