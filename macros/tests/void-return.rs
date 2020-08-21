use wll::Result;
use wll_macros::{wll_export, wll_setup, wll_teardown};

#[wll_setup]
fn setup() {}

#[wll_teardown]
fn teardown() {}

#[wll_export]
fn print(n: usize) -> Result<()> {
    eprintln!("Input: {}", n);
    Ok(())
}

fn main() {}
