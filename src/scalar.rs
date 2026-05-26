use std::ops::*;

pub trait Scalar:
	Sized
	+ Copy
	+ std::fmt::Display
	+ PartialEq + PartialOrd
	+ std::iter::Sum
	+ num_traits::Zero
	+ num_traits::Signed
	+ num_traits::MulAdd<Output = Self>
	+ Add<Output = Self> + AddAssign
	+ Mul<Output = Self> + MulAssign
	+ Sub<Output = Self> + SubAssign
	+ Div<Output = Self> + DivAssign
	+ Neg<Output = Self>
{}

impl Scalar for f32 {}
impl Scalar for i32 {}
