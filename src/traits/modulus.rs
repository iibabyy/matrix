pub trait Modulus {
	fn modulus(&self) -> f32;
}

impl Modulus for f32 { fn modulus(&self) -> f32 { self.abs() } }
impl Modulus for i32 { fn modulus(&self) -> f32 { self.abs() as f32 } }
