use wll::{Complex, Result};

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export(round)]
fn cround(z: Complex<f64>) -> Result<Complex<i32>> {
    Ok(Complex::new(z.re.round() as i32, z.im.round() as i32))
}

fn main() {}
