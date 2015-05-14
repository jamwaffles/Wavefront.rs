use std::fs::File;
use std::io::{ BufReader, BufRead };

struct Vertex {
	x: f32,
	y: f32,
	z: f32
}

pub struct WavefrontModel {
	vertices: Vec<Vertex>
}

impl WavefrontModel {
	pub fn new(path: &'static str) {
		let file = BufReader::new(File::open(&path).unwrap());

		for line in file.lines()  {
			println!("{}", line.unwrap())
		}
	}
}