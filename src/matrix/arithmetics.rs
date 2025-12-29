//! Arithmetic trait implementations for `Matrix`.
//!
//! This module provides implementations for standard Rust arithmetic traits
//! (`Add`, `Sub`, `Mul`) and their assignment counterparts (`AddAssign`, `SubAssign`, `MulAssign`).
//!
//! # Supported Operations
//!
//! * **Addition (`+`) & Subtraction (`-`):** Performed **component-wise**. \
//!   Both matrices must have the same dimensions (same number of rows and columns).
//! * **Scalar Multiplication (`*`):** Scales every element of the matrix by a scalar value `K`.
//!
//! # Panics
//!
//! Binary operations acting on two matrices (Addition and Subtraction) will **panic**
//! if the matrices do not have the same dimensions (mismatched number of vectors).
//!
//! # Examples
//!
//! ```rust,ignore
//! use crate::matrix::Matrix;
//! use crate::vector::Vector;
//!
//! // Constructing 2x2 matrices (assuming Matrix wraps a Vec of Vectors)
//! let m1 = Matrix::new(vec![
//!     Vector::new(vec![1.0, 2.0]),
//!     Vector::new(vec![3.0, 4.0]),
//! ]);
//!
//! let m2 = Matrix::new(vec![
//!     Vector::new(vec![4.0, 5.0]),
//!     Vector::new(vec![6.0, 7.0]),
//! ]);
//!
//! // 1. Addition (Component-wise)
//! let sum = &m1 + &m2;
//! // Result: [[5.0, 7.0], [9.0, 11.0]]
//!
//! // 2. Subtraction (Component-wise)
//! let diff = &m2 - &m1;
//! // Result: [[3.0, 3.0], [3.0, 3.0]]
//!
//! // 3. Scalar Multiplication
//! let scaled = m1 * 2.0;
//! // Result: [[2.0, 4.0], [6.0, 8.0]]
//!
//! // 4. Compound Assignment
//! let mut m_mut = Matrix::new(vec![
//!     Vector::new(vec![10.0, 10.0]),
//!     Vector::new(vec![10.0, 10.0]),
//! ]);
//! m_mut += Matrix::new(vec![
//!     Vector::new(vec![5.0, 5.0]),
//!     Vector::new(vec![5.0, 5.0]),
//! ]);
//! // Result: [[15.0, 15.0], [15.0, 15.0]]
//! ```

use crate::matrix::Matrix;
use crate::{macros::*, vector::Vector};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// #[cfg(test)]
// mod tests;

// -----------------------------------------------------------------------------
// Addition
// -----------------------------------------------------------------------------

fn add_matrix_matrix<K: Copy + Neg + Add<Output = K>>(a: &Matrix<K>, b: &Matrix<K>) -> Matrix<K> {
    assert_eq!(a.size(), b.size());
    let mut new = a.clone();

    for i in 0..new.size() {
        new[i] += &b[i];
    }

    new
}

impl_add_ops!(
    <K> Matrix<K>, Matrix<K>,
    with add_matrix_matrix,
    where K: Copy + Neg + Add<Output = K>
);

// -----------------------------------------------------------------------------
// Substraction
// -----------------------------------------------------------------------------

fn sub_matrix_matrix<K: Copy + Neg + Sub<Output = K>>(a: &Matrix<K>, b: &Matrix<K>) -> Matrix<K> {
    assert_eq!(a.size(), b.size());
    let mut new = a.clone();

    for i in 0..new.size() {
        new[i] -= &b[i];
    }

    new
}

impl_sub_ops!(
    <K> Matrix<K>, Matrix<K>,
    with sub_matrix_matrix,
    where K: Copy + Neg + Sub<Output = K>
);

// -----------------------------------------------------------------------------
// Coeff Multiplication
// -----------------------------------------------------------------------------

fn mul_matrix_coeff<K: Copy + Neg + Mul<Output = K>>(matrix: &Matrix<K>, coeff: &K) -> Matrix<K> {
    let mut new = matrix.clone();

    for i in 0..new.size() {
        new[i] *= *coeff;
    }

    new
}

impl_mul_ops!(
    <K> Matrix<K>, K,
    with mul_matrix_coeff,
    where K: Copy + Neg + Mul<Output = K>,
);

// -----------------------------------------------------------------------------
// Matrix Multiplication
// -----------------------------------------------------------------------------

fn mul_matrix_matrix<K>(a: &Matrix<K>, b: &Matrix<K>) -> Matrix<K>
where
    K: Copy + Neg + Mul<Output = K> + Add<Output = K>,
{
    let mut new: Vec<Vector<K>> = Vec::with_capacity(a.size());

    for i in 0..a.size() {
        new.push(&a[i] * b);
    }

    Matrix::new(new)
}

impl_mul_ops!(
    <K> Matrix<K>, Matrix<K>,
    with mul_matrix_matrix,
    where K: Copy + Neg + Mul<Output = K> + Add<Output = K>,
);
#[cfg(test)]
mod tests {
    use crate::matrix;

    // -------------------------------------------------------------------------
    // TEST: ADDITION (Element-wise)
    // -------------------------------------------------------------------------
    mod addition {
        use super::*;

        #[test]
        fn test_owned_owned() {
            let m1 = matrix![[1, 2], [3, 4]];
            let m2 = matrix![[1, 1], [1, 1]];
            let result = m1 + m2;
            assert_eq!(result.vectors[0].scalars, vec![2, 3]);
            assert_eq!(result.vectors[1].scalars, vec![4, 5]);
        }

        #[test]
        fn test_owned_ref() {
            let m1 = matrix![[10, 20]];
            let m2 = matrix![[1, 2]];
            let result = m1 + &m2;
            assert_eq!(result.vectors[0].scalars, vec![11, 22]);
        }

        #[test]
        fn test_assign_owned() {
            let mut m1 = matrix![[0, 0], [10, 10]];
            let m2 = matrix![[1, 1], [1, 1]];
            m1 += m2;
            assert_eq!(m1.vectors[0].scalars, vec![1, 1]);
            assert_eq!(m1.vectors[1].scalars, vec![11, 11]);
        }

        #[test]
        #[should_panic]
        fn test_panic_dim_mismatch() {
            // Cannot add 1x2 matrix to 2x2 matrix
            let m1 = matrix![[1, 2]];
            let m2 = matrix![[1, 2], [3, 4]];
            let _ = m1 + m2;
        }
    }

    // -------------------------------------------------------------------------
    // TEST: SUBTRACTION (Element-wise)
    // -------------------------------------------------------------------------
    mod subtraction {
        use super::*;

        #[test]
        fn test_owned_owned() {
            let m1 = matrix![[10, 20], [30, 40]];
            let m2 = matrix![[1, 2], [3, 4]];
            let result = m1 - m2;
            assert_eq!(result.vectors[0].scalars, vec![9, 18]);
            assert_eq!(result.vectors[1].scalars, vec![27, 36]);
        }

        #[test]
        fn test_assign() {
            let mut m1 = matrix![[10, 10]];
            let m2 = matrix![[3, 4]];
            m1 -= m2;
            assert_eq!(m1.vectors[0].scalars, vec![7, 6]);
        }

        #[test]
        #[should_panic]
        fn test_panic_dim_mismatch() {
            let m1 = matrix![[1, 2]];
            let m2 = matrix![[1, 2], [3, 4]];
            let _ = &m1 - &m2;
        }
    }

    // -------------------------------------------------------------------------
    // TEST: SCALAR MULTIPLICATION (Element-wise)
    // -------------------------------------------------------------------------
    mod scalar_multiplication {
        use super::*;

        #[test]
        fn test_owned_scalar() {
            let m1 = matrix![[1, -2], [3, 4]];
            let result = m1 * 2;
            assert_eq!(result.vectors[0].scalars, vec![2, -4]);
            assert_eq!(result.vectors[1].scalars, vec![6, 8]);
        }

        #[test]
        fn test_ref_scalar() {
            let m1 = matrix![[10, 20]];
            let result = &m1 * 3;
            assert_eq!(result.vectors[0].scalars, vec![30, 60]);
        }

        #[test]
        fn test_scalar_multiplication_by_zero() {
            // 0 * [1, 2] = [0, 0]
            let m1 = matrix![[1, 2], [3, 4]];

            // Assuming you implemented Mul<K> for Matrix
            let result = m1 * 0;

            assert_eq!(result.vectors[0].scalars, vec![0, 0]);
            assert_eq!(result.vectors[1].scalars, vec![0, 0]);
        }
    }

    // -------------------------------------------------------------------------
    // TEST: MATRIX MULTIPLICATION (Row x Col)
    // -------------------------------------------------------------------------
    mod matrix_multiplication {
        use crate::vector;

        use super::*;

        #[test]
        fn test_2x2_matrix_multiplication() {
            // A (2x2) * B (2x2)
            // [1, 2]   [5, 6]
            // [3, 4] * [7, 8]
            //
            // Row 0: [1*5 + 2*7, 1*6 + 2*8] = [5+14, 6+16] = [19, 22]
            // Row 1: [3*5 + 4*7, 3*6 + 4*8] = [15+28, 18+32] = [43, 50]
            let m1 = matrix![[1, 2], [3, 4]];
            let m2 = matrix![[5, 6], [7, 8]];

            let result = &m1 * &m2;

            assert_eq!(result.vectors[0].scalars, vec![19, 22]);
            assert_eq!(result.vectors[1].scalars, vec![43, 50]);
        }

        #[test]
        fn test_3x3_matrix_multiplication() {
            // [1, 2, 3]   [9, 8, 7]
            // [4, 5, 6] * [6, 5, 4]
            // [7, 8, 9]   [3, 2, 1]
            //
            // Row 0:
            // c0: 1*9 + 2*6 + 3*3 = 9 + 12 + 9 = 30
            // c1: 1*8 + 2*5 + 3*2 = 8 + 10 + 6 = 24
            // c2: 1*7 + 2*4 + 3*1 = 7 + 8 + 3 = 18
            let m1 = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
            let m2 = matrix![[9, 8, 7], [6, 5, 4], [3, 2, 1]];

            let result = m1 * m2;

            assert_eq!(result.vectors[0].scalars, vec![30, 24, 18]);
            // Sanity check another row just to be safe
            // Row 1, Col 0: 4*9 + 5*6 + 6*3 = 36 + 30 + 18 = 84
            assert_eq!(result.vectors[1].scalars[0], 84);
        }

        #[test]
        fn test_identity_matrix_post_multiply() {
            // A * I = A
            let m1 = matrix![[5, 3], [2, 4]];
            let identity = matrix![[1, 0], [0, 1]];

            let result = m1 * &identity;

            assert_eq!(result.vectors[0].scalars, vec![5, 3]);
            assert_eq!(result.vectors[1].scalars, vec![2, 4]);
        }

        #[test]
        fn test_identity_matrix_pre_multiply() {
            // I * A = A
            let identity = matrix![[1, 0], [0, 1]];
            let m2 = matrix![[5, 3], [2, 4]];

            let result = identity * &m2;

            assert_eq!(result.vectors[0].scalars, vec![5, 3]);
            assert_eq!(result.vectors[1].scalars, vec![2, 4]);
        }

        #[test]
        fn test_matrix_multiplication_ref_owned() {
            // [2, 1]   [3, 1]
            // [1, 2] * [1, 3]
            //
            // Row 0: [2*3 + 1*1, 2*1 + 1*3] = [7, 5]
            // Row 1: [1*3 + 2*1, 1*1 + 2*3] = [5, 7]
            let m1 = matrix![[2, 1], [1, 2]];
            let m2 = matrix![[3, 1], [1, 3]];

            let result = &m1 * m2;

            assert_eq!(result.vectors[0].scalars, vec![7, 5]);
            assert_eq!(result.vectors[1].scalars, vec![5, 7]);
        }

        #[test]
        fn test_assignment_swap_columns() {
            // Multiplying A by a permutation matrix on the RIGHT swaps COLUMNS.
            // A = [1, 2]  Swap Matrix = [0, 1]
            //     [3, 4]                [1, 0]
            //
            // Row 0: [1*0 + 2*1, 1*1 + 2*0] = [2, 1]
            // Row 1: [3*0 + 4*1, 3*1 + 4*0] = [4, 3]
            // Result: Cols are swapped.
            let mut m1 = matrix![[1, 2], [3, 4]];
            let m2 = matrix![[0, 1], [1, 0]];

            m1 *= m2;

            assert_eq!(m1.vectors[0].scalars, vec![2, 1]);
            assert_eq!(m1.vectors[1].scalars, vec![4, 3]);
        }

        #[test]
        fn test_with_zeros_and_negatives() {
            // [ 1,  2]   [ 0, -1]
            // [-3,  4] * [ 5,  2]
            //
            // Row 0: [1*0 + 2*5,  1*-1 + 2*2]   = [10, 3]
            // Row 1: [-3*0 + 4*5, -3*-1 + 4*2]  = [20, 11]
            let m1 = matrix![[1, 2], [-3, 4]];
            let m2 = matrix![[0, -1], [5, 2]];

            let result = m1 * m2;

            assert_eq!(result.vectors[0].scalars, vec![10, 3]);
            assert_eq!(result.vectors[1].scalars, vec![20, 11]);
        }

        #[test]
        fn test_rectangular_valid() {
            // (1x2) * (2x2) is VALID
            // [1, 2] * [3, 4]
            //          [5, 6]
            // Result is (1x2)
            // [1*3 + 2*5, 1*4 + 2*6] = [3+10, 4+12] = [13, 16]
            let m1 = matrix![[1, 2]];
            let m2 = matrix![[3, 4], [5, 6]];
            let result = m1 * m2;

            assert_eq!(result.vectors[0].scalars, vec![13, 16]);
        }

        #[test]
        #[should_panic]
        fn test_panic_dim_mismatch() {
            // (2x2) * (1x2) is INVALID
            // Cols of A (2) != Rows of B (1)
            let m1 = matrix![[1, 2], [3, 4]]; // 2x2
            let m2 = matrix![[1, 2]]; // 1x2 (1 row)
            let _ = m1 * m2;
        }

        #[test]
        fn test_rectangular_tall_by_wide() {
            // (3x2) * (2x3) -> Result (3x3)
            // [1, 2]   [7, 8, 9]
            // [3, 4] * [1, 0, 1]
            // [5, 6]
            //
            // R0: [1*7+2*1, 1*8+2*0, 1*9+2*1] = [9, 8, 11]
            // R1: [3*7+4*1, 3*8+4*0, 3*9+4*1] = [25, 24, 31]
            // R2: [5*7+6*1, 5*8+6*0, 5*9+6*1] = [41, 40, 51]
            let m1 = matrix![[1, 2], [3, 4], [5, 6]];
            let m2 = matrix![[7, 8, 9], [1, 0, 1]];

            let result = m1 * m2;

            assert_eq!(result.vectors[0].scalars, vec![9, 8, 11]);
            assert_eq!(result.vectors[1].scalars, vec![25, 24, 31]);
            assert_eq!(result.vectors[2].scalars, vec![41, 40, 51]);
        }

        #[test]
        fn test_multiplication_by_zero_matrix() {
            // A * 0 = 0
            // [1, 2]   [0, 0]   [0, 0]
            // [3, 4] * [0, 0] = [0, 0]
            let m1 = matrix![[1, 2], [3, 4]];
            let zero_matrix = matrix![[0, 0], [0, 0]];

            let result = m1 * zero_matrix;

            assert_eq!(result.vectors[0].scalars, vec![0, 0]);
            assert_eq!(result.vectors[1].scalars, vec![0, 0]);
        }

        #[test]
        fn test_rectangular_vector_matrix() {
            // Vector (1x2) * Matrix (2x3) -> Result (1x3)
            // [1, 2] * [ [1, 2, 3],
            //            [4, 5, 6] ]
            //
            // This is a linear combination of the rows:
            // 1 * [1, 2, 3] + 2 * [4, 5, 6]
            // = [1, 2, 3] + [8, 10, 12]
            // = [9, 12, 15]

            let vector = vector![1, 2];
            let matrix = matrix![[1, 2, 3], [4, 5, 6]];

            let result = vector * matrix;

            assert_eq!(result.scalars, vec![9, 12, 15]);
        }

        #[test]
        fn test_rectangular_zero_multiplication() {
            // (3x2) * (2x1) Zero Matrix -> Result (3x1) Zero Matrix
            // [1, 2]   [0]   [0]
            // [3, 4] * [0] = [0]
            // [5, 6]         [0]
            let m1 = matrix![[1, 2], [3, 4], [5, 6]];
            let zero_col = matrix![[0], [0]];

            let result = m1 * zero_col;

            assert_eq!(result.vectors.len(), 3); // 3 rows
            assert_eq!(result.vectors[0].scalars.len(), 1); // 1 column
            assert_eq!(result.vectors[0].scalars[0], 0);
            assert_eq!(result.vectors[1].scalars[0], 0);
            assert_eq!(result.vectors[2].scalars[0], 0);
        }
    }
}
