use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
		let mut file = match File::open(&path) {
			// The `description` method of `io::Error` returns a string that describes the error
			Err(why) => panic!("couldn't open {}", Error::description(&why)),
			Ok(file) => file,
		};

		// Read the file contents into a string, returns `io::Result<usize>`
		let mut s = String::new();

		match file.read_to_string(&mut s) {
			Err(why) => panic!("couldn't read {}", Error::description(&why)),
			Ok(_) => print!("{}", s),
		}
	}

	// fn parseLine(line)
}