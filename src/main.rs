mod wavefront;

fn main() {
	let model = wavefront::WavefrontModel::new("../models/cube.obj");

	model.draw();
}