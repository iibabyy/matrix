use crate::macros::{impl_add_ops, impl_mul_ops, impl_sub_ops};

#[derive(Debug, Clone, Copy)]
pub struct ComplexNumber {
	real: f32,
	imag: f32,
}

impl ComplexNumber {
	pub fn new(real: f32, imag: f32) -> Self {
		Self { real, imag }
	}
}

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

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Matrix, Vector, matrix, vector};

    const EPS: f32 = 1e-5;

    fn c(real: f32, imag: f32) -> ComplexNumber {
        ComplexNumber::new(real, imag)
    }

    fn assert_complex_eq(actual: ComplexNumber, real: f32, imag: f32) {
        assert!(
            (actual.real - real).abs() < EPS,
            "real mismatch: got {}, expected {}",
            actual.real,
            real
        );
        assert!(
            (actual.imag - imag).abs() < EPS,
            "imag mismatch: got {}, expected {}",
            actual.imag,
            imag
        );
    }

    // -------------------------------------------------------------------------
    // TEST: SCALAR COMPLEX ARITHMETIC
    // -------------------------------------------------------------------------
    mod scalar {
        use super::*;

        #[test]
        fn add_complex_complex() {
            // (1 + 2i) + (3 + 4i) = (4 + 6i)
            let a = c(1.0, 2.0);
            let b = c(3.0, 4.0);
            assert_complex_eq(a + b, 4.0, 6.0);
        }

        #[test]
        fn add_complex_real() {
            // (1 + 2i) + 5 = (6 + 2i)  -- adding a real only affects the real part
            let a = c(1.0, 2.0);
            assert_complex_eq(a + 5.0, 6.0, 2.0);
        }

        #[test]
        fn sub_complex_complex() {
            // (5 + 3i) - (2 + 1i) = (3 + 2i)
            let a = c(5.0, 3.0);
            let b = c(2.0, 1.0);
            assert_complex_eq(a - b, 3.0, 2.0);
        }

        #[test]
        fn sub_complex_real() {
            // (5 + 3i) - 2 = (3 + 3i)
            let a = c(5.0, 3.0);
            assert_complex_eq(a - 2.0, 3.0, 3.0);
        }

        #[test]
        fn mul_complex_complex() {
            // (1 + 2i)(3 + 4i) = (1*3 - 2*4) + (1*4 + 2*3)i = -5 + 10i
            let a = c(1.0, 2.0);
            let b = c(3.0, 4.0);
            assert_complex_eq(a * b, -5.0, 10.0);
        }

        #[test]
        fn mul_complex_complex_i_squared() {
            // i * i = -1 + 0i
            let i = c(0.0, 1.0);
            assert_complex_eq(i * i, -1.0, 0.0);
        }

        #[test]
        fn mul_complex_real() {
            // (1 + 2i) * 3 = (3 + 6i)
            let a = c(1.0, 2.0);
            assert_complex_eq(a * 3.0, 3.0, 6.0);
        }
    }

    // -------------------------------------------------------------------------
    // TEST: Vector<ComplexNumber>
    // -------------------------------------------------------------------------
    mod vector_ops {
        use super::*;

        #[test]
        fn add() {
            let v1: Vector<ComplexNumber> = vector![c(1.0, 2.0), c(3.0, 4.0)];
            let v2: Vector<ComplexNumber> = vector![c(5.0, 6.0), c(7.0, 8.0)];
            let result = &v1 + &v2;
            assert_complex_eq(result[0], 6.0, 8.0);
            assert_complex_eq(result[1], 10.0, 12.0);
        }

        #[test]
        fn add_assign() {
            let mut v1: Vector<ComplexNumber> = vector![c(1.0, 1.0), c(2.0, 2.0)];
            let v2: Vector<ComplexNumber> = vector![c(3.0, 4.0), c(5.0, 6.0)];
            v1 += v2;
            assert_complex_eq(v1[0], 4.0, 5.0);
            assert_complex_eq(v1[1], 7.0, 8.0);
        }

        #[test]
        fn sub() {
            let v1: Vector<ComplexNumber> = vector![c(5.0, 6.0), c(7.0, 8.0)];
            let v2: Vector<ComplexNumber> = vector![c(1.0, 2.0), c(3.0, 4.0)];
            let result = &v1 - &v2;
            assert_complex_eq(result[0], 4.0, 4.0);
            assert_complex_eq(result[1], 4.0, 4.0);
        }

        #[test]
        fn mul_componentwise() {
            // [(1+2i), (3+4i)] * [(0+1i), (2+0i)]
            //   = [(1+2i)(0+1i), (3+4i)(2+0i)]
            //   = [(-2 + 1i), (6 + 8i)]
            let v1: Vector<ComplexNumber> = vector![c(1.0, 2.0), c(3.0, 4.0)];
            let v2: Vector<ComplexNumber> = vector![c(0.0, 1.0), c(2.0, 0.0)];
            let result = &v1 * &v2;
            assert_complex_eq(result[0], -2.0, 1.0);
            assert_complex_eq(result[1], 6.0, 8.0);
        }

        #[test]
        fn mul_by_scalar_complex() {
            // [(1+1i), (2+0i)] * (0+1i) = [(-1+1i), (0+2i)]
            let v: Vector<ComplexNumber> = vector![c(1.0, 1.0), c(2.0, 0.0)];
            let scalar = c(0.0, 1.0);
            let result = &v * scalar;
            assert_complex_eq(result[0], -1.0, 1.0);
            assert_complex_eq(result[1], 0.0, 2.0);
        }

        #[test]
        fn vector_times_matrix() {
            // v = [(1+0i), (0+1i)]
            // M = [[(1+0i), (0+0i)],
            //      [(0+0i), (1+0i)]]   -- 2x2 identity (as columns)
            // v * I = v
            let v: Vector<ComplexNumber> = vector![c(1.0, 0.0), c(0.0, 1.0)];
            let m: Matrix<ComplexNumber> = matrix![
                [c(1.0, 0.0), c(0.0, 0.0)],
                [c(0.0, 0.0), c(1.0, 0.0)]
            ];
            let result = &v * &m;
            assert_complex_eq(result[0], 1.0, 0.0);
            assert_complex_eq(result[1], 0.0, 1.0);
        }

        #[test]
        fn vector_times_matrix_nontrivial() {
            // v = [(1+1i), (2+0i)]
            // M (as columns) = [[(1+0i), (0+1i)],
            //                   [(0+1i), (1+0i)]]
            //
            // linear_combination of columns: (1+1i)*col0 + (2+0i)*col1
            //   col0 scaled: [(1+1i)*(1+0i), (1+1i)*(0+1i)] = [(1+1i), (-1+1i)]
            //   col1 scaled: [(2+0i)*(0+1i), (2+0i)*(1+0i)] = [(0+2i), (2+0i)]
            //   sum:         [(1+3i),        (1+1i)]
            let v: Vector<ComplexNumber> = vector![c(1.0, 1.0), c(2.0, 0.0)];
            let m: Matrix<ComplexNumber> = matrix![
                [c(1.0, 0.0), c(0.0, 1.0)],
                [c(0.0, 1.0), c(1.0, 0.0)]
            ];
            let result = &v * &m;
            assert_complex_eq(result[0], 1.0, 3.0);
            assert_complex_eq(result[1], 1.0, 1.0);
        }
    }

    // -------------------------------------------------------------------------
    // TEST: Matrix<ComplexNumber>
    // -------------------------------------------------------------------------
    mod matrix_ops {
        use super::*;

        #[test]
        fn add() {
            let m1: Matrix<ComplexNumber> = matrix![
                [c(1.0, 1.0), c(2.0, 2.0)],
                [c(3.0, 3.0), c(4.0, 4.0)]
            ];
            let m2: Matrix<ComplexNumber> = matrix![
                [c(5.0, 5.0), c(6.0, 6.0)],
                [c(7.0, 7.0), c(8.0, 8.0)]
            ];
            let result = &m1 + &m2;
            assert_complex_eq(result[0][0], 6.0, 6.0);
            assert_complex_eq(result[0][1], 8.0, 8.0);
            assert_complex_eq(result[1][0], 10.0, 10.0);
            assert_complex_eq(result[1][1], 12.0, 12.0);
        }

        #[test]
        fn sub() {
            let m1: Matrix<ComplexNumber> = matrix![
                [c(10.0, 10.0), c(20.0, 20.0)],
                [c(30.0, 30.0), c(40.0, 40.0)]
            ];
            let m2: Matrix<ComplexNumber> = matrix![
                [c(1.0, 2.0), c(3.0, 4.0)],
                [c(5.0, 6.0), c(7.0, 8.0)]
            ];
            let result = &m1 - &m2;
            assert_complex_eq(result[0][0], 9.0, 8.0);
            assert_complex_eq(result[0][1], 17.0, 16.0);
            assert_complex_eq(result[1][0], 25.0, 24.0);
            assert_complex_eq(result[1][1], 33.0, 32.0);
        }

        #[test]
        fn matrix_times_vector() {
            // Identity columns -> v unchanged
            let m: Matrix<ComplexNumber> = matrix![
                [c(1.0, 0.0), c(0.0, 0.0)],
                [c(0.0, 0.0), c(1.0, 0.0)]
            ];
            let v: Vector<ComplexNumber> = vector![c(3.0, 2.0), c(1.0, 5.0)];
            let result = m.mul_vec(&v);
            assert_complex_eq(result[0], 3.0, 2.0);
            assert_complex_eq(result[1], 1.0, 5.0);
        }

        #[test]
        fn matrix_times_matrix_identity() {
            // I * I = I
            let i: Matrix<ComplexNumber> = matrix![
                [c(1.0, 0.0), c(0.0, 0.0)],
                [c(0.0, 0.0), c(1.0, 0.0)]
            ];
            let result = i.mul_mat(&i);
            assert_complex_eq(result[0][0], 1.0, 0.0);
            assert_complex_eq(result[0][1], 0.0, 0.0);
            assert_complex_eq(result[1][0], 0.0, 0.0);
            assert_complex_eq(result[1][1], 1.0, 0.0);
        }

        #[test]
        fn matrix_times_matrix_nontrivial() {
            // A = [[(1+0i), (0+1i)],
            //      [(0+1i), (1+0i)]]  (as columns)
            // B = [[(1+1i), (0+0i)],
            //      [(0+0i), (1+1i)]]  (as columns)
            //
            // A * B treats A's columns as basis weighted by B's columns.
            // Result columns:
            //   col0 of result = A * B.col0 = (1+1i)*A.col0 + (0)*A.col1
            //                  = [(1+1i)*(1+0i), (1+1i)*(0+1i)]
            //                  = [(1+1i), (-1+1i)]
            //   col1 of result = A * B.col1 = (0)*A.col0 + (1+1i)*A.col1
            //                  = [(1+1i)*(0+1i), (1+1i)*(1+0i)]
            //                  = [(-1+1i), (1+1i)]
            let a: Matrix<ComplexNumber> = matrix![
                [c(1.0, 0.0), c(0.0, 1.0)],
                [c(0.0, 1.0), c(1.0, 0.0)]
            ];
            let b: Matrix<ComplexNumber> = matrix![
                [c(1.0, 1.0), c(0.0, 0.0)],
                [c(0.0, 0.0), c(1.0, 1.0)]
            ];
            let result = a.mul_mat(&b);
            assert_complex_eq(result[0][0], 1.0, 1.0);
            assert_complex_eq(result[0][1], -1.0, 1.0);
            assert_complex_eq(result[1][0], -1.0, 1.0);
            assert_complex_eq(result[1][1], 1.0, 1.0);
        }
    }

    // -------------------------------------------------------------------------
    // TEST: linear_combination over ComplexNumber
    // -------------------------------------------------------------------------
    mod linear_combination {
        use super::*;
        use crate::linear_combination;

        #[test]
        fn weighted_sum_of_basis() {
            // 2 * [(1+0i), (0+0i)] + (0+3i) * [(0+0i), (1+0i)]
            //   = [(2+0i), (0+3i)]
            let e1: Vector<ComplexNumber> = vector![c(1.0, 0.0), c(0.0, 0.0)];
            let e2: Vector<ComplexNumber> = vector![c(0.0, 0.0), c(1.0, 0.0)];
            let vectors = vec![e1, e2];
            let coeffs = vec![c(2.0, 0.0), c(0.0, 3.0)];
            let result = linear_combination(&vectors, &coeffs);
            assert_complex_eq(result[0], 2.0, 0.0);
            assert_complex_eq(result[1], 0.0, 3.0);
        }
    }
}
