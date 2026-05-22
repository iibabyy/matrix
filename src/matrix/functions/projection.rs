use crate::{Matrix, matrix};

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix::<f32> {
	// const f = 1 / Math.tan(fovY / 2);
	// const nf = 1 / (near - far);
	
	// // Column-major: each row in the literal is one column of the matrix
	// return new Float32Array([
	//     f / aspect, 0, 0,                     0,
	//     0,          f, 0,                     0,
	//     0,          0, (far + near) * nf,    -1,
	//     0,          0, 2 * far * near * nf,   0,
	// ]);

	let f = 1.0 / (fov / 2.0).tan();
	let f = &f; // to use it multiple times with *f
	let nf = 1.0 / (near - far);

	matrix![
		[*f / ratio, 0., 0., 0.],
		[0., *f, 0., 0.],
		[0., 0., (far + near) * nf, -1.],
		[0., 0., 2. * far * near * nf, 0.],
	]
}
