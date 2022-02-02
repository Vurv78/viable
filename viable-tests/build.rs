#[cfg(test)]
pub fn main() {
	let mut build = cc::Build::new();
	build
		.file("tests/cpp/basic.cpp")
		.cpp(true);

	if let Ok(c) = build.try_get_compiler() {
		if !c.is_like_msvc() {
			build.compiler("msvc");
		}
	}

	assert!(
		build.get_compiler().is_like_msvc(),
		"Only MSVC is supported."
	);
	build.compile("basic");

	// Link to "basic"
	println!("cargo:rustc-link-search=native=basic");
	println!("cargo:rerun-if-changed=tests/cpp/basic.cpp");

	println!("cargo:warning=This is a warning");
}

#[cfg(not(test))]
fn main() {}
