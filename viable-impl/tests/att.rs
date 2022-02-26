// Attributes
use std::os::raw::c_int;
use viable::vtable;

#[vtable]
pub(crate) struct Bar {
	#[offset(1)]
	pub(crate) first: extern "C" fn() -> c_int,

	#[skip(1)]
	#[check(3)]
	/// Doc
	/// Comments
	pub(crate) third: extern "C" fn(i: c_int),
}

pub fn main() {}