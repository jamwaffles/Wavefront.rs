use std::fs::File;
use std::io::{ BufReader, BufRead };

pub mod face;
pub mod point3d;

use self::face::Face;
use self::point3d::Point3D;

pub struct WavefrontModel {
	vertices: Vec<Point3D>,
	normals: Vec<Point3D>,
	faces: Vec<Face>
}

impl WavefrontModel {
	pub fn new(path: &'static str) -> WavefrontModel {
		let file = BufReader::new(File::open(&path).unwrap());

		let mut vertices = Vec::new();
		let mut normals = Vec::new();
		let mut faces = Vec::new();

		for wrapped_line in file.lines()  {
			let line: String = wrapped_line.unwrap();
			let mut tokens: Vec<&str> = line.split(' ').collect();

			let line_type = tokens.remove(0);

			match line_type {
				"v" => vertices.push(WavefrontModel::parse_point(tokens)),
				"vn" => normals.push(WavefrontModel::parse_point(tokens)),
				"f" => faces.push(WavefrontModel::parse_face(tokens)),
				_ => (),
			}
		}

		WavefrontModel {
			vertices: vertices,
			normals: normals,
			faces: faces
		}
	}

	fn parse_point(tokens: Vec<&str>) -> Point3D {
		Point3D { x: tokens[0].parse::<f32>().unwrap(), y: tokens[1].parse::<f32>().unwrap(), z: tokens[2].parse::<f32>().unwrap() }
	}

	fn parse_face(tokens: Vec<&str>) -> Face {
		let v1: Vec<&str> = tokens[0].split('/').collect();
		let v2: Vec<&str> = tokens[1].split('/').collect();
		let v3: Vec<&str> = tokens[2].split('/').collect();

		// Obj files index from `1` hence the `-1` offset
		Face {
			v1: v1[0].parse::<usize>().unwrap() - 1,
			vn1: v1[2].parse::<usize>().unwrap() - 1,

			v2: v2[0].parse::<usize>().unwrap() - 1,
			vn2: v2[2].parse::<usize>().unwrap() - 1,

			v3: v3[0].parse::<usize>().unwrap() - 1,
			vn3: v3[2].parse::<usize>().unwrap() - 1
		}
	}

	pub fn draw(&self) {
		for face in self.faces.iter() {
			// println!("Face [ {}, {}, {} ]", self.vertices[face.v1], self.vertices[face.v2], self.vertices[face.v3]);
			println!("{}", self.vertices[face.v1]);
		}
	}
}