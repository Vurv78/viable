use std::{os::raw::{c_int, c_char}, ffi::c_void};
use viable::vtable;

extern "C" {
	fn getPug(name: *const c_char, age: c_int) -> *mut Pug;
}


#[vtable]
struct Pug {
	__internal_name: *const c_char,
	__internal_age: c_int,

	#[offset(0)]
	name: extern "C" fn() -> *const c_char,

	#[offset(1)]
	speak: extern "C" fn() -> *const c_char,

	age: extern "C" fn() -> c_int
}

#[test]
pub fn test_inherit() {
	let iface = unsafe { getPug( b"Fido\0".as_ptr() as _, 5 ) };
	let iface = unsafe { iface.as_mut().unwrap() };

	// Internal name var
	assert_eq!( iface.__internal_name, b"Fido\0".as_ptr() as _ );

	// Or through proper function
	let name = iface.name();
	let name = unsafe { std::ffi::CStr::from_ptr(name) };
	assert_eq!( name.to_string_lossy(), "Fido" );

	let speak = iface.speak();
	let speak = unsafe { std::ffi::CStr::from_ptr(speak) };
	println!( "{}", speak.to_string_lossy() );

	let age = iface.age();
	println!("Age: {}", age);
}