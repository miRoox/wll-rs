#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/empty-setup.rs");
    t.pass("tests/setup-return.rs");
    t.pass("tests/add-two.rs");
    t.pass("tests/factorial.rs");
}
