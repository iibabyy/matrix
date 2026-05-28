pub trait Scalar:
	Sized
	+ Copy
	+ PartialEq
	+ PartialOrd
	+ std::ops::Neg
	+ std::iter::Sum
	+ num_traits::NumAssign
	+ num_traits::Signed
	+ num_traits::MulAdd<Output = Self>
	+ std::fmt::Display
{}

impl<T> Scalar for T
where
	T: Sized,
	T: Copy,
	T: PartialEq,
	T: PartialOrd,
	T: std::ops::Neg,
	T: std::iter::Sum,
	T: num_traits::NumAssign,
	T: num_traits::Signed,
	T: num_traits::MulAdd<Output = Self>,
	T: std::fmt::Display,
{}
