use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

#[export]
fn add_two(a: isize, b: isize) -> isize {
    a + b
}

fn main() {}
