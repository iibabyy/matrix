use crate::traits::scalar::Scalar;

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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::functions::{cross_product, lerp, linear_combination};
	use crate::matrix::Matrix;
	use crate::vector::Vector;
	use crate::{vector};
	use ::num_traits::{MulAdd, One, Zero};

	// Local constructor — `real`/`imag` are private to this module,
	// so tests can build instances directly without changing the public API.
	fn c(real: f32, imag: f32) -> ComplexNumber {
		ComplexNumber { real, imag }
	}

	// Magnitude / determinant inherit f32 precision constraints, so a
	// tolerance-based check is safer than strict equality on arithmetic results.
	fn approx_eq_complex(a: ComplexNumber, b: ComplexNumber) {
		let eps = 1e-4;
		assert!(
			(a.real - b.real).abs() < eps && (a.imag - b.imag).abs() < eps,
			"expected {b}, got {a}"
		);
	}

	fn approx_eq_f32(a: f32, b: f32) {
		let eps = 1e-4;
		assert!((a - b).abs() < eps, "expected {b}, got {a}");
	}

	// `matrix!([..], [..])` stores each `[..]` as a COLUMN (column-major).
	// Tests are easier to read when written in row form; this helper bridges that.
	fn from_rows(rows: Vec<Vec<ComplexNumber>>) -> Matrix<ComplexNumber> {
		let n_rows = rows.len();
		let n_cols = rows[0].len();
		let mut cols = Vec::with_capacity(n_cols);
		for j in 0..n_cols {
			let mut col = Vec::with_capacity(n_rows);
			for row in &rows {
				col.push(row[j]);
			}
			cols.push(Vector::new(col));
		}
		Matrix::new(cols)
	}

	// Element-wise tolerance comparison of two complex matrices (matrix indexing
	// is column-major: `m[col][row]`).
	fn approx_eq_matrix(a: &Matrix<ComplexNumber>, b: &Matrix<ComplexNumber>) {
		assert_eq!(a.cols(), b.cols(), "column count differs");
		assert_eq!(a.rows(), b.rows(), "row count differs");
		for col in 0..a.cols() {
			for row in 0..a.rows() {
				approx_eq_complex(a[col][row], b[col][row]);
			}
		}
	}

	// =========================================================================
	// MAGNITUDE
	// =========================================================================

	#[test]
	fn magnitude_zero() {
		approx_eq_f32(c(0., 0.).magnitude(), 0.0);
	}

	#[test]
	fn magnitude_pure_real() {
		approx_eq_f32(c(3., 0.).magnitude(), 3.0);
		approx_eq_f32(c(-3., 0.).magnitude(), 3.0);
	}

	#[test]
	fn magnitude_pure_imag() {
		approx_eq_f32(c(0., 4.).magnitude(), 4.0);
		approx_eq_f32(c(0., -4.).magnitude(), 4.0);
	}

	#[test]
	fn magnitude_3_4_5() {
		approx_eq_f32(c(3., 4.).magnitude(), 5.0);
		approx_eq_f32(c(-3., -4.).magnitude(), 5.0);
	}

	// =========================================================================
	// ARITHMETIC: Add / Sub / Mul / Div / Neg
	// =========================================================================

	#[test]
	fn add_complex_complex() {
		approx_eq_complex(c(1., 2.) + c(3., 4.), c(4., 6.));
	}

	#[test]
	fn add_zero_is_identity() {
		approx_eq_complex(c(2., 3.) + ComplexNumber::zero(), c(2., 3.));
	}

	#[test]
	fn add_complex_real() {
		approx_eq_complex(c(2., 5.) + 3.0_f32, c(5., 5.));
	}

	#[test]
	fn sub_complex_complex() {
		approx_eq_complex(c(1., 2.) - c(3., 4.), c(-2., -2.));
	}

	#[test]
	fn sub_complex_real() {
		approx_eq_complex(c(5., 7.) - 2.0_f32, c(3., 7.));
	}

	#[test]
	fn mul_complex_complex() {
		// (3 + 2i)(1 + 7i) = (3·1 − 2·7) + (3·7 + 2·1)i = -11 + 23i
		approx_eq_complex(c(3., 2.) * c(1., 7.), c(-11., 23.));
	}

	#[test]
	fn mul_i_squared_is_minus_one() {
		approx_eq_complex(c(0., 1.) * c(0., 1.), c(-1., 0.));
	}

	#[test]
	fn mul_by_one_is_identity() {
		approx_eq_complex(c(2., 3.) * ComplexNumber::one(), c(2., 3.));
	}

	#[test]
	fn mul_by_zero_is_zero() {
		approx_eq_complex(c(7., -4.) * ComplexNumber::zero(), c(0., 0.));
	}

	#[test]
	fn mul_complex_real() {
		approx_eq_complex(c(2., 3.) * 4.0_f32, c(8., 12.));
	}

	#[test]
	fn div_complex_complex() {
		// (4 + 2i)/(3 − i) = ((4·3 + 2·(-1)) + (2·3 − 4·(-1))i) / (9 + 1)
		//                 = (10 + 10i) / 10 = 1 + i
		approx_eq_complex(c(4., 2.) / c(3., -1.), c(1., 1.));
	}

	#[test]
	fn div_by_one_is_identity() {
		approx_eq_complex(c(5., -3.) / ComplexNumber::one(), c(5., -3.));
	}

	#[test]
	fn div_complex_real() {
		approx_eq_complex(c(8., 12.) / 4.0_f32, c(2., 3.));
	}

	#[test]
	fn neg_negates_both_parts() {
		approx_eq_complex(-c(3., -4.), c(-3., 4.));
		approx_eq_complex(-c(0., 0.), c(0., 0.));
	}

	// =========================================================================
	// TRAITS: Display / Zero / One / Sum / PartialEq / MulAdd
	// =========================================================================

	#[test]
	fn display_positive_parts() {
		assert_eq!(format!("{}", c(3., 4.)), "3 + 4i");
	}

	#[test]
	fn display_negative_imag_uses_plus_minus() {
		// Current impl always prints `a + bi` literally, even for negative b.
		// This pins the existing behaviour; change it if the format changes.
		assert_eq!(format!("{}", c(3., -4.)), "3 + -4i");
	}

	#[test]
	fn zero_is_additive_identity() {
		assert_eq!(ComplexNumber::zero(), c(0., 0.));
		assert!(ComplexNumber::zero().is_zero());
	}

	#[test]
	fn one_is_multiplicative_identity() {
		assert_eq!(ComplexNumber::one(), c(1., 0.));
	}

	#[test]
	fn sum_of_complex_iter() {
		let s: ComplexNumber = [c(1., 1.), c(2., 2.), c(3., 3.)].into_iter().sum();
		approx_eq_complex(s, c(6., 6.));
	}

	#[test]
	fn partial_eq_equal() {
		assert_eq!(c(1.5, -2.5), c(1.5, -2.5));
	}

	#[test]
	fn partial_eq_differs_on_either_component() {
		assert_ne!(c(1.5, -2.5), c(1.5, 2.5));
		assert_ne!(c(1.5, -2.5), c(0.0, -2.5));
	}

	#[test]
	fn mul_add_trait() {
		// (2 + i)·(3 + 0i) + (1 + i) = (6 + 3i) + (1 + i) = 7 + 4i
		approx_eq_complex(c(2., 1.).mul_add(c(3., 0.), c(1., 1.)), c(7., 4.));
	}

	// =========================================================================
	// Vector<ComplexNumber>
	// =========================================================================

	#[test]
	fn vector_add() {
		let mut u = vector![c(1., 1.), c(2., 0.)];
		let v = vector![c(0., 1.), c(1., 0.)];
		u.add(&v);
		approx_eq_complex(u[0], c(1., 2.));
		approx_eq_complex(u[1], c(3., 0.));
	}

	#[test]
	fn vector_sub() {
		let mut u = vector![c(2., 3.), c(5., 7.)];
		let v = vector![c(1., 1.), c(2., 2.)];
		u.sub(&v);
		approx_eq_complex(u[0], c(1., 2.));
		approx_eq_complex(u[1], c(3., 5.));
	}

	#[test]
	fn vector_scl_by_imaginary_rotates_90deg() {
		// Multiplying by i rotates each component 90° in the complex plane:
		// [1+0i, 0+1i] · i = [0+1i, -1+0i]
		let mut u = vector![c(1., 0.), c(0., 1.)];
		u.scl(c(0., 1.));
		approx_eq_complex(u[0], c(0., 1.));
		approx_eq_complex(u[1], c(-1., 0.));
	}

	#[test]
	fn vector_dot_is_bilinear_not_hermitian() {
		// `dot` computes Σ aᵢ·bᵢ — the bilinear form, NOT the Hermitian inner product
		// (which would be Σ aᵢ·conj(bᵢ)). Over ℂ this means dot(u, u) is NOT |u|².
		// u = [1+2i, 3+4i]
		// (1+2i)² + (3+4i)² = (-3 + 4i) + (-7 + 24i) = -10 + 28i
		let u = vector![c(1., 2.), c(3., 4.)];
		approx_eq_complex(u.clone().dot(u), c(-10., 28.));
	}

	#[test]
	fn vector_dot_real_basis_orthogonal() {
		let u = vector![c(1., 0.), c(0., 0.)];
		let v = vector![c(0., 0.), c(1., 0.)];
		approx_eq_complex(u.dot(v), c(0., 0.));
	}

	#[test]
	fn vector_linear_combination_with_complex_coeffs() {
		// i · [1, 0] + 1 · [0, 1] = [i, 1]
		let e1 = vector![c(1., 0.), c(0., 0.)];
		let e2 = vector![c(0., 0.), c(1., 0.)];
		let result = linear_combination(&[e1, e2], &[c(0., 1.), c(1., 0.)]);
		approx_eq_complex(result[0], c(0., 1.));
		approx_eq_complex(result[1], c(1., 0.));
	}

	#[test]
	fn vector_cross_product_complex_basis() {
		// Standard real cross-product lifted into ℂ³: e_x × e_y = e_z
		let ex = vector![c(1., 0.), c(0., 0.), c(0., 0.)];
		let ey = vector![c(0., 0.), c(1., 0.), c(0., 0.)];
		let ez = cross_product(&ex, &ey);
		approx_eq_complex(ez[0], c(0., 0.));
		approx_eq_complex(ez[1], c(0., 0.));
		approx_eq_complex(ez[2], c(1., 0.));
	}

	// =========================================================================
	// Matrix<ComplexNumber>
	// =========================================================================

	#[test]
	fn matrix_add() {
		let a = from_rows(vec![
			vec![c(1., 1.), c(2., 0.)],
			vec![c(0., 0.), c(3., 1.)],
		]);
		let b = from_rows(vec![
			vec![c(0., 1.), c(2., 2.)],
			vec![c(1., 0.), c(0., 1.)],
		]);
		let expected = from_rows(vec![
			vec![c(1., 2.), c(4., 2.)],
			vec![c(1., 0.), c(3., 2.)],
		]);
		assert_eq!(a + b, expected);
	}

	#[test]
	fn matrix_sub() {
		let a = from_rows(vec![
			vec![c(2., 2.), c(5., 3.)],
			vec![c(1., 1.), c(4., 0.)],
		]);
		let b = from_rows(vec![
			vec![c(1., 1.), c(2., 1.)],
			vec![c(0., 0.), c(1., 0.)],
		]);
		let expected = from_rows(vec![
			vec![c(1., 1.), c(3., 2.)],
			vec![c(1., 1.), c(3., 0.)],
		]);
		assert_eq!(a - b, expected);
	}

	#[test]
	fn matrix_mul_vec() {
		// A = [[1, i],
		//      [i, 1]]    v = [1, 1]
		// Av = [1·1 + i·1, i·1 + 1·1] = [1+i, 1+i]
		let a = from_rows(vec![
			vec![c(1., 0.), c(0., 1.)],
			vec![c(0., 1.), c(1., 0.)],
		]);
		let v = vector![c(1., 0.), c(1., 0.)];
		let r = a.mul_vec(&v);
		approx_eq_complex(r[0], c(1., 1.));
		approx_eq_complex(r[1], c(1., 1.));
	}

	#[test]
	fn matrix_mul_mat_i_times_identity_squared_is_minus_identity() {
		// A = iI₂ → A·A = -I₂
		let a = from_rows(vec![
			vec![c(0., 1.), c(0., 0.)],
			vec![c(0., 0.), c(0., 1.)],
		]);
		let expected = from_rows(vec![
			vec![c(-1., 0.), c(0., 0.)],
			vec![c(0., 0.), c(-1., 0.)],
		]);
		assert_eq!(a.clone().mul_mat(&a), expected);
	}

	#[test]
	fn matrix_transpose_is_not_hermitian() {
		// transpose() swaps (i, j) ↔ (j, i) WITHOUT conjugating.
		// For a true Hermitian adjoint we'd need a `conjugate()` method
		// (currently unimplemented — see audit notes).
		let a = from_rows(vec![
			vec![c(1., 1.), c(2., 0.)],
			vec![c(3., 0.), c(4., -1.)],
		]);
		let expected = from_rows(vec![
			vec![c(1., 1.), c(3., 0.)],
			vec![c(2., 0.), c(4., -1.)],
		]);
		assert_eq!(a.transpose(), expected);
	}

	#[test]
	fn matrix_trace() {
		// tr(diag(1+2i, 3-4i)) = 4 - 2i
		let a = from_rows(vec![
			vec![c(1., 2.), c(0., 0.)],
			vec![c(0., 0.), c(3., -4.)],
		]);
		approx_eq_complex(a.trace(), c(4., -2.));
	}

	#[test]
	fn matrix_determinant_2x2() {
		// A = [[1+i, 2],
		//      [3,   4-i]]
		// det = (1+i)(4-i) − 2·3 = (5 + 3i) − 6 = -1 + 3i
		let a = from_rows(vec![
			vec![c(1., 1.), c(2., 0.)],
			vec![c(3., 0.), c(4., -1.)],
		]);
		approx_eq_complex(a.determinant(), c(-1., 3.));
	}

	#[test]
	fn matrix_determinant_3x3_diagonal() {
		// det(diag(1+i, 1, 2)) = (1+i)·1·2 = 2 + 2i
		let a = from_rows(vec![
			vec![c(1., 1.), c(0., 0.), c(0., 0.)],
			vec![c(0., 0.), c(1., 0.), c(0., 0.)],
			vec![c(0., 0.), c(0., 0.), c(2., 0.)],
		]);
		approx_eq_complex(a.determinant(), c(2., 2.));
	}

	#[test]
	fn matrix_rank_linearly_dependent() {
		// Row 1 = i · Row 0  →  rank drops to 1.
		let a = from_rows(vec![
			vec![c(1., 0.), c(2., 0.)],
			vec![c(0., 1.), c(0., 2.)],
		]);
		assert_eq!(a.rank(), 1);
	}

	#[test]
	fn matrix_rank_full() {
		let a = from_rows(vec![
			vec![c(1., 1.), c(0., 0.)],
			vec![c(0., 0.), c(2., -1.)],
		]);
		assert_eq!(a.rank(), 2);
	}

	#[test]
	fn matrix_row_echelon_preserves_dimensions() {
		// `is_row_echelon_form()` uses exact `== K::zero()`, which f32 row reduction
		// rarely produces (residuals ~1e-7 on real *and* imag parts over ℂ).
		// We only assert shape preservation and panic-freeness here; rank correctness
		// is covered by `matrix_rank_*` above.
		let a = from_rows(vec![
			vec![c(1., 1.), c(2., 0.), c(0., 1.)],
			vec![c(3., 0.), c(1., -1.), c(2., 0.)],
			vec![c(0., 1.), c(4., 0.), c(1., 1.)],
		]);
		let r = a.row_echelon();
		assert_eq!(r.rows(), 3);
		assert_eq!(r.cols(), 3);
	}

	#[test]
	fn matrix_scl_by_imaginary() {
		// Matrix scaling is the `Mul<K>` operator (Matrix has no `scl` method, unlike
		// Vector). Scaling by i rotates every entry 90° in the complex plane.
		let a = from_rows(vec![
			vec![c(1., 0.), c(0., 1.)],
			vec![c(2., 0.), c(0., 2.)],
		]);
		let scaled = a * c(0., 1.);
		let expected = from_rows(vec![
			vec![c(0., 1.), c(-1., 0.)],
			vec![c(0., 2.), c(-2., 0.)],
		]);
		approx_eq_matrix(&scaled, &expected);
	}

	// =========================================================================
	// lerp over ℂ (Ex02): values are complex scalars, the blend factor t stays real
	// =========================================================================

	#[test]
	fn lerp_complex_midpoint() {
		approx_eq_complex(lerp(c(0., 0.), c(2., 4.), 0.5), c(1., 2.));
	}

	#[test]
	fn lerp_complex_endpoints() {
		approx_eq_complex(lerp(c(1., 1.), c(3., 5.), 0.), c(1., 1.));
		approx_eq_complex(lerp(c(1., 1.), c(3., 5.), 1.), c(3., 5.));
	}

	// =========================================================================
	// norm over ℂ (Ex04): always returns a real f32, computed from component moduli
	// =========================================================================

	#[test]
	fn norm_of_complex_vector() {
		// moduli: |3+4i| = 5, |0+5i| = 5
		let v = vector![c(3., 4.), c(0., 5.)];
		approx_eq_f32(v.norm_1(), 10.0); // 5 + 5
		approx_eq_f32(v.norm(), 50.0_f32.sqrt()); // sqrt(25 + 25)
		approx_eq_f32(v.norm_inf(), 5.0); // max(5, 5)
	}

	// =========================================================================
	// inverse over ℂ (Ex12): verified by the defining property A·A⁻¹ = A⁻¹·A = I
	// =========================================================================

	#[test]
	fn inverse_of_identity_is_identity() {
		let id = Matrix::<ComplexNumber>::identity(3);
		let inv = id.clone().inverse().unwrap();
		approx_eq_matrix(&inv, &id);
	}

	#[test]
	fn inverse_of_diagonal_complex() {
		// diag(2, 4i)⁻¹ = diag(1/2, 1/(4i)) = diag(0.5, -0.25i) — exactly representable.
		let a = from_rows(vec![
			vec![c(2., 0.), c(0., 0.)],
			vec![c(0., 0.), c(0., 4.)],
		]);
		let inv = a.clone().inverse().unwrap();
		let expected = from_rows(vec![
			vec![c(0.5, 0.), c(0., 0.)],
			vec![c(0., 0.), c(0., -0.25)],
		]);
		approx_eq_matrix(&inv, &expected);
		approx_eq_matrix(&a.clone().mul_mat(&inv), &Matrix::identity(2));
	}

	#[test]
	fn inverse_general_2x2_complex_multiply_back() {
		// det = (1+i)(4-i) − 2·3 = -1 + 3i ≠ 0, so A is invertible.
		let a = from_rows(vec![
			vec![c(1., 1.), c(2., 0.)],
			vec![c(3., 0.), c(4., -1.)],
		]);
		let inv = a.clone().inverse().unwrap();
		let id = Matrix::<ComplexNumber>::identity(2);
		approx_eq_matrix(&a.clone().mul_mat(&inv), &id);
		approx_eq_matrix(&inv.clone().mul_mat(&a), &id);
	}

	#[test]
	fn inverse_of_singular_complex_is_err() {
		// Row 1 = i · Row 0  →  rows are linearly dependent, matrix is singular.
		let mut a = from_rows(vec![
			vec![c(1., 0.), c(2., 0.)],
			vec![c(0., 1.), c(0., 2.)],
		]);
		assert!(a.inverse().is_err());
	}
}
