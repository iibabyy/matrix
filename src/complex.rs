use crate::scalar::Scalar;

mod arithmetics;
mod num_traits;
mod traits;

#[derive(Debug, Clone, Copy)]
pub struct ComplexNumber {
	real: f32,
	imag: f32,
}

impl ComplexNumber {
	pub fn magnitude(&self) -> f32 {
		self.real.mul_add(self.real, self.imag * self.imag).sqrt()
	}
}
