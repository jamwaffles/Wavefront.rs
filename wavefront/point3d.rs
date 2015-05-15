use std::fmt;

pub struct Point3D {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl fmt::Display for Point3D {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Point3D({:.5}, {:.5}, {:.5})", self.x, self.y, self.z)
	}
}