use std::ops::*;

use num_traits::{MulAdd, Num, NumAssignOps, One, Signed, Zero};

use crate::complex::ComplexNumber;

impl MulAdd for ComplexNumber {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        (self * a) + b
    }
}

impl Signed for ComplexNumber {
	fn abs(&self) -> Self {
    	ComplexNumber {
     		real: self.magnitude(),
       		imag: 0.0,
     	}
	}

	fn abs_sub(&self, other: &Self) -> Self {
    	self.abs() - other.abs()
	}

	fn signum(&self) -> Self {
    	if self == &Self::zero() {
        	Self::zero()
    	} else {
     		Self::one()
     	}
	}

	fn is_negative(&self) -> bool {
    	false
	}

	fn is_positive(&self) -> bool {
    	self > &Self::zero()
	}
}

impl One for ComplexNumber {
    fn one() -> Self {
        ComplexNumber { real: 1.0, imag: 0.0 }
    }
}

impl Zero for ComplexNumber {
    fn zero() -> Self {
        ComplexNumber { real: 0.0, imag: 0.0 }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl Rem for ComplexNumber {
    type Output = Self;
    #[doc(hidden)]
    fn rem(self, other: Self) -> Self {
        // remainder isn't used with complex numbers but Num requires it
        Self {
            real: self.real % other.real,
            imag: self.imag % other.imag,
        }
    }
}

impl RemAssign for ComplexNumber {
    #[doc(hidden)]
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}

#[doc(hidden)]
mod num_impl {
	use super::*;

	// A simple custom error for our parser
	#[derive(Debug, PartialEq)]
	pub struct ComplexParseError;
	
	impl Num for ComplexNumber {
	    type FromStrRadixErr = ComplexParseError;
	
	    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
	        let s = str.trim();
	
	        // 1. Handle complex numbers ending with 'i' (e.g., "3.0+4.5i")
	        if s.ends_with('i') {
	            let s_without_i = &s[..s.len() - 1];
	            
	            // Find where the imaginary part starts (look for the last + or -)
	            if let Some(pos) = s_without_i.rfind('+').or_else(|| s_without_i.rfind('-')) {
	                // If the sign is at index 0, it's just a negative pure imaginary number
	                if pos == 0 {
	                    let imag = <f32 as Num>::from_str_radix(s_without_i, radix)
	                        .map_err(|_| ComplexParseError)?;
	                    return Ok(Self { real: 0.0, imag });
	                }
	
	                // Split the string into real and imaginary parts
	                let real_str = &s_without_i[..pos];
	                let imag_str = &s_without_i[pos..];
	
	                // Delegate radix parsing to the inner f32 type
	                let real = <f32 as Num>::from_str_radix(real_str, radix).map_err(|_| ComplexParseError)?;
	                let imag = <f32 as Num>::from_str_radix(imag_str, radix).map_err(|_| ComplexParseError)?;
	
	                return Ok(Self { real, imag });
	            } else {
	                // Pure imaginary with no sign (e.g., "5.0i")
	                let imag = <f32 as Num>::from_str_radix(s_without_i, radix)
	                    .map_err(|_| ComplexParseError)?;
	                return Ok(Self { real: 0.0, imag });
	            }
	        }
	
	        // 2. Handle pure real numbers (e.g., "4.2")
	        let real = <f32 as Num>::from_str_radix(s, radix).map_err(|_| ComplexParseError)?;
	        Ok(Self { real, imag: 0.0 })
	    }
	}
}
