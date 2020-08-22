use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

#[export(do_nothing)]
fn nothing() {}

fn main() {}
