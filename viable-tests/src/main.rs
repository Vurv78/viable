use std::os::raw::c_int;
use viable_impl::vtable;

extern "C" {
	fn getMath(i: c_int) -> *mut Math;
}

#[vtable]
struct Math {
	internal: c_int,

	add: extern "C" fn(a: c_int, b: c_int) -> c_int,
	#[offset(1)] // Completely optional
	add2: extern "C" fn(a: c_int, b: c_int) -> c_int,
}


pub fn main() {
	let iface = unsafe { getMath(10) };
	let iface = unsafe { iface.as_mut().unwrap() };
	assert_eq!( iface.add2(5, 5), 5 + 5 + 10 );
}