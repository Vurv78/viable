use std::os::raw::c_int;
use viable::vtable;

#[vtable]
pub(crate) struct Bar { }

mod foo {
	use super::{vtable, c_int};

	#[vtable]
	pub struct Qux {
		internal: c_int,
		br: extern "C" fn(),
	}
}

pub fn main() {}