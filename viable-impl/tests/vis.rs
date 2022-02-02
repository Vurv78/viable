use std::os::raw::c_int;
use viable::vtable;

#[vtable]
pub(crate) struct Bar { }

mod foo {
	use super::{vtable, c_int};

	#[vtable]
	pub struct Qux {
		pub internal: c_int,
		pub br: extern "C" fn(),
	}
}

pub fn main() {}