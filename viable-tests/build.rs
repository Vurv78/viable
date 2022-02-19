pub fn main() {
	let a = std::fs::read_dir("src").expect("Failed to read src dir");
	let files = a.flatten().map_while(|x| {
		if let Some(ext) = x.path().extension() {
			if ext == "cpp" { return Some(x.path()) }
		}
		None
	});

	let mut build = cc::Build::new();
	build
		.files(files)
		.flag("/std:c++20")
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
	build.compile("tests");

	// Link to "basic"
	println!("cargo:rustc-link-search=native=tests");
}
