#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export]
fn add_two(a: isize, b: isize) -> isize {
    a + b
}

fn main() {}
