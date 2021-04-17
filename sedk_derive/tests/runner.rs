#[test]
fn trybuild_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/primitives.rs");
    t.pass("tests/composition.rs");
}
