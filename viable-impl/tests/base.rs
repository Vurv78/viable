use std::os::raw::c_int;
use viable::vtable;

// Just test syntax

#[vtable]
struct Foo {
	internal: c_int,

	add: extern "C" fn(a: c_int, b: c_int) -> c_int,
	#[offset(1)] // Completely optional
	add2: extern "C" fn(a: c_int, b: c_int) -> c_int,
}

pub fn main() {}