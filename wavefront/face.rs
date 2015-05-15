use std::fmt;

pub struct Face {
	pub v1: usize,
	pub vn1: usize,

	pub v2: usize,
	pub vn2: usize,

	pub v3: usize,
	pub vn3: usize
}

impl fmt::Display for Face {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Face({}, {}, {})", self.v1, self.v2, self.v3)
	}
}