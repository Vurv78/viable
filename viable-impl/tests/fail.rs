use viable::vtable;

// Should fail as can't stack repr transparent with repr C
/*#[repr(transparent)]
#[vtable]
struct Baz {}
*/

pub fn main() {}