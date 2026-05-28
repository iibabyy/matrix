use std::{iter::Sum, ops::Neg};

use num_traits::Zero;

use crate::{complex::ComplexNumber, traits::modulus::Modulus};

impl Modulus for ComplexNumber {
    fn modulus(&self) -> f32 {
        self.magnitude()
    }
}

impl Eq for ComplexNumber {}
impl PartialEq for ComplexNumber {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl Ord for ComplexNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.magnitude().total_cmp(&other.magnitude())
    }
}

impl PartialOrd for ComplexNumber {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    	Some(self.cmp(other))
	}
}

impl Neg for ComplexNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ComplexNumber { real: -self.real, imag: -self.imag }
    }
}

impl std::fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl Sum for ComplexNumber {
	fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    	iter.fold(ComplexNumber::zero(), |acc, x| acc + x)
	}
}
