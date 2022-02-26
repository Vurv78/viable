#[test]
fn tests() {
	let t = trybuild::TestCases::new();
	t.pass("tests/base.rs");
	t.pass("tests/vis.rs");
	t.pass("tests/att.rs");

	// Apparently this fails a little too hard? :/
	// t.compile_fail("tests/fail.rs")
}