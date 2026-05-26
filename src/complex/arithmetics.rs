use crate::{complex::ComplexNumber, macros::*};

fn add_complex_complex(a: &ComplexNumber, b: &ComplexNumber) -> ComplexNumber {
	ComplexNumber {
		real: a.real + b.real,
		imag: a.imag + b.imag
	}
}

impl_add_ops!(
    <> ComplexNumber, ComplexNumber,
    with add_complex_complex,
);

fn add_complex_f32(a: &ComplexNumber, b: &f32) -> ComplexNumber {
	ComplexNumber {
		real: a.real + b,
		imag: a.imag
	}
}

impl_add_ops!(
    <> ComplexNumber, f32,
    with add_complex_f32,
);

fn sub_complex_complex(a: &ComplexNumber, b: &ComplexNumber) -> ComplexNumber {
	ComplexNumber {
		real: a.real - b.real,
		imag: a.imag - b.imag
	}
}

impl_sub_ops!(
    <> ComplexNumber, ComplexNumber,
    with sub_complex_complex,
);

fn sub_complex_f32(a: &ComplexNumber, b: &f32) -> ComplexNumber {
	ComplexNumber {
		real: a.real - b,
		imag: a.imag
	}
}

impl_sub_ops!(
    <> ComplexNumber, f32,
    with sub_complex_f32,
);

fn mul_complex_complex(a: &ComplexNumber, b: &ComplexNumber) -> ComplexNumber {
	let real = (a.real * b.real) - (a.imag * b.imag); // imag * imag == real
	let imag = (a.real * b.imag) + (a.imag * b.real);
	ComplexNumber {
		real,
		imag
	}
}

impl_mul_ops!(
    <> ComplexNumber, ComplexNumber,
    with mul_complex_complex,
);

fn mul_complex_f32(a: &ComplexNumber, b: &f32) -> ComplexNumber {
	ComplexNumber {
		real: a.real * b,
		imag: a.imag * b
	}
}

impl_mul_ops!(
    <> ComplexNumber, f32,
    with mul_complex_f32,
);

fn div_complex_complex(numerator: &ComplexNumber, denominator: &ComplexNumber) -> ComplexNumber {
    // a + bi   ac + bd   bc - ad
    // ------ = ------- + -------i
    // c + di   c² + d²   c² + d²

    let a = numerator.real;
    let b = numerator.imag;
    let c = denominator.real;
    let d = denominator.imag;

    let denominator = c.mul_add(c, d * d);
    let real = a.mul_add(c, b * d) / denominator;
    let imag = b.mul_add(c, -(a * d)) / denominator;

    ComplexNumber { real, imag }
}

impl_div_ops!(
    <> ComplexNumber, ComplexNumber,
    with div_complex_complex,
);

fn div_complex_f32(a: &ComplexNumber, b: &f32) -> ComplexNumber {
	ComplexNumber {
		real: a.real / b,
		imag: a.imag / b
	}
}

impl_div_ops!(
    <> ComplexNumber, f32,
    with div_complex_f32,
);
