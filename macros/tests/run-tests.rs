#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/empty-setup.rs")
}
