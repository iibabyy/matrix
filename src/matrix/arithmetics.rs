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

use crate::{matrix::Matrix};
use crate::vector::Vector;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// #[cfg(test)]
// mod tests;

// -----------------------------------------------------------------------------
// Addition
// -----------------------------------------------------------------------------

fn add_assign_inner<K>(self_vectors: &mut [Vector<K>], other_scalars: &[Vector<K>])
where
    K: Copy + Neg + Add<Output = K>,
{
    assert_eq!(self_vectors.len(), other_scalars.len());

    for i in 0..self_vectors.len() {
        self_vectors[i] = &self_vectors[i] + &other_scalars[i];
    }
}

fn add_inner<K>(self_vectors: &[Vector<K>], other_scalars: &[Vector<K>]) -> Vec<Vector<K>>
where
    K: Copy + Neg + Add<Output = K>,
{
    let mut new = self_vectors.to_owned();
    add_assign_inner(&mut new, other_scalars);
    new
}

// Matrix += Matrix
impl<K> AddAssign<Matrix<K>> for Matrix<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    fn add_assign(&mut self, other: Matrix<K>) {
        add_assign_inner(&mut self.vectors, &other.vectors);
    }
}

// Matrix += &Matrix
impl<K> AddAssign<&Matrix<K>> for Matrix<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    fn add_assign(&mut self, other: &Matrix<K>) {
        add_assign_inner(&mut self.vectors, &other.vectors);
    }
}

// Matrix + Matrix
impl<K> Add for Matrix<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    type Output = Matrix<K>;

    fn add(mut self, other: Matrix<K>) -> Self::Output {
        self += other;
        self
    }
}

// Matrix + &Matrix
impl<K> Add<&Matrix<K>> for Matrix<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    type Output = Matrix<K>;

    fn add(mut self, other: &Matrix<K>) -> Self::Output {
        self += other;
        self
    }
}

// &Matrix + &Matrix
impl<K> Add<&Matrix<K>> for &Matrix<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    type Output = Matrix<K>;

    fn add(self, other: &Matrix<K>) -> Self::Output {
        let mut new: Matrix<K> = self.clone();
        new += other;
        new
    }
}

// -----------------------------------------------------------------------------
// Substraction
// -----------------------------------------------------------------------------

fn sub_assign_inner<K>(self_vectors: &mut [Vector<K>], other_scalars: &[Vector<K>])
where
    K: Copy + Neg + Sub<Output = K>,
{
    assert_eq!(self_vectors.len(), other_scalars.len());

    for i in 0..self_vectors.len() {
        self_vectors[i] = &self_vectors[i] - &other_scalars[i];
    }
}

fn sub_inner<K>(self_vectors: &[Vector<K>], other_scalars: &[Vector<K>]) -> Vec<Vector<K>>
where
    K: Copy + Neg + Sub<Output = K>,
{
    let mut new = self_vectors.to_owned();
    sub_assign_inner(&mut new, other_scalars);
    new
}

// Matrix -= Matrix
impl<K> SubAssign<Matrix<K>> for Matrix<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    fn sub_assign(&mut self, other: Matrix<K>) {
        sub_assign_inner(&mut self.vectors, &other.vectors);
    }
}

// Matrix -= &Matrix
impl<K> SubAssign<&Matrix<K>> for Matrix<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    fn sub_assign(&mut self, other: &Matrix<K>) {
        sub_assign_inner(&mut self.vectors, &other.vectors);
    }
}

// Matrix - Matrix
impl<K> Sub for Matrix<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    type Output = Matrix<K>;

    fn sub(self, other: Matrix<K>) -> Self::Output {
        Matrix::new(sub_inner(&self.vectors, &other.vectors))
    }
}

// Matrix - &Matrix
impl<K> Sub<&Matrix<K>> for Matrix<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    type Output = Matrix<K>;

    fn sub(mut self, other: &Matrix<K>) -> Self::Output {
        self -= other;
        self
    }
}

// &Matrix - &Matrix
impl<K> Sub<&Matrix<K>> for &Matrix<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    type Output = Matrix<K>;

    fn sub(self, other: &Matrix<K>) -> Self::Output {
        let mut new: Matrix<K> = self.clone();
        new -= other;
        new
    }
}

// -----------------------------------------------------------------------------
// Coeff Multiplication
// -----------------------------------------------------------------------------

// Since K implements Copy, we don't need to implement the traits for &K

fn coef_mul_assign_inner<K>(self_vectors: &mut [Vector<K>], coeff: K)
where
    K: Copy + Neg,
    Vector<K>: MulAssign<K>,
{
    for scalar in self_vectors {
        *scalar *= coeff;
    }
}

fn coef_mul_inner<K>(self_vectors: &[Vector<K>], coeff: K) -> Vec<Vector<K>>
where
    K: Copy + Neg,
    Vector<K>: Neg + MulAssign<K>,
{
    let mut new = self_vectors.to_owned();
    coef_mul_assign_inner(&mut new, coeff);
    new
}

// Matrix *= coeff
impl<K> MulAssign<K> for Matrix<K>
where
    K: Copy + Neg,
    Vector<K>: Neg + MulAssign<K>,
{
    fn mul_assign(&mut self, coeff: K) {
        coef_mul_assign_inner(&mut self.vectors, coeff);
    }
}

// Matrix * coeff
impl<K> Mul<K> for Matrix<K>
where
    K: Copy + Neg,
    Vector<K>: Neg + MulAssign<K>,
{
    type Output = Matrix<K>;

    fn mul(mut self, coeff: K) -> Self::Output {
        self *= coeff;
        self
    }
}

// &Matrix * coeff
impl<K> Mul<K> for &Matrix<K>
where
    K: Copy + Neg,
    Vector<K>: Neg + MulAssign<K>,
{
    type Output = Matrix<K>;

    fn mul(self, coeff: K) -> Self::Output {
        Matrix::new(coef_mul_inner(&self.vectors, coeff))
    }
}

#[cfg(test)]
mod tests {
    use crate::macros::matrix;

    // -------------------------------------------------------------------------
    // TEST: ADDITION
    // -------------------------------------------------------------------------
    mod addition {
        use super::*;

        #[test]
        fn test_owned_owned() {
            let m1 = matrix![[1, 2], [3, 4]];
            let m2 = matrix![[1, 1], [1, 1]];
            let result = m1 + m2;

            // Check specific rows
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
        fn test_ref_ref() {
            let m1 = matrix![[1, 2], [3, 4]];
            let m2 = matrix![[4, 3], [2, 1]];
            let result = &m1 + &m2;

            assert_eq!(result.vectors[0].scalars, vec![5, 5]);
            assert_eq!(result.vectors[1].scalars, vec![5, 5]);

            // Ensure originals are still valid
            assert_eq!(m1.vectors[0].scalars, vec![1, 2]);
            assert_eq!(m2.vectors[0].scalars, vec![4, 3]);
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
        fn test_assign_ref() {
            let mut m1 = matrix![[5, 5]];
            let m2 = matrix![[1, 1]];

            m1 += &m2;

            assert_eq!(m1.vectors[0].scalars, vec![6, 6]);
        }

        #[test]
        #[should_panic]
        fn test_panic_dim_mismatch_rows() {
            // Panic when adding matrices with different number of rows (vectors)
            let m1 = matrix![[1, 2]];
            let m2 = matrix![[1, 2], [3, 4]];
            let _ = m1 + m2;
        }

        #[test]
        #[should_panic]
        fn test_panic_dim_mismatch_cols() {
            // Panic when adding matrices where rows have different lengths
            // (This panic actually propagates from Vector addition)
            let m1 = matrix![[1, 2]];
            let m2 = matrix![[1, 2, 3]];
            let _ = m1 + m2;
        }
    }

    // -------------------------------------------------------------------------
    // TEST: SUBTRACTION
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
        fn test_ref_ref() {
            let m1 = matrix![[5, 5]];
            let m2 = matrix![[1, 1]];
            let result = &m1 - &m2;

            assert_eq!(result.vectors[0].scalars, vec![4, 4]);
        }

        #[test]
        fn test_assign() {
            let mut m1 = matrix![[10, 10]];
            let m2 = matrix![[3, 4]];

            m1 -= m2;

            assert_eq!(m1.vectors[0].scalars, vec![7, 6]);
        }

        #[test]
        fn test_negative_result() {
            let m1 = matrix![[0, 0]];
            let m2 = matrix![[1, 1]];
            let result = m1 - m2;

            assert_eq!(result.vectors[0].scalars, vec![-1, -1]);
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
    // TEST: SCALAR MULTIPLICATION (SCL)
    // -------------------------------------------------------------------------
    mod multiplication {
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
            // Ensure original is preserved
            assert_eq!(m1.vectors[0].scalars, vec![10, 20]);
        }

        #[test]
        fn test_assign() {
            let mut m1 = matrix![[2, 4], [1, 1]];
            m1 *= 2;

            assert_eq!(m1.vectors[0].scalars, vec![4, 8]);
            assert_eq!(m1.vectors[1].scalars, vec![2, 2]);
        }

        #[test]
        fn test_zero() {
            let m1 = matrix![[1, 2, 3], [4, 5, 6]];
            let result = m1 * 0;

            assert_eq!(result.vectors[0].scalars, vec![0, 0, 0]);
            assert_eq!(result.vectors[1].scalars, vec![0, 0, 0]);
        }
    }
}
