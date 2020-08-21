use wll_macros::{wll_export, wll_setup, wll_teardown};

#[wll_setup]
fn setup() {}

#[wll_teardown]
fn teardown() {}

#[wll_export]
fn add_two(a: isize, b: isize) -> isize {
    a + b
}

fn main() {}
