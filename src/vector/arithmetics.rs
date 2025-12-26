//! Arithmetic trait implementations for `Vector`.
//!
//! This module provides implementations for standard Rust arithmetic traits
//! (`Add`, `Sub`, `Mul`) and their assignment counterparts (`AddAssign`, `SubAssign`, `MulAssign`).
//!
//! # Supported Operations
//!
//! * **Addition (`+`) & Subtraction (`-`):** Performed **component-wise**. \
//!   Both vectors must have the same dimension (length).
//! * **Scalar Multiplication (`*`):** Scales every component of the vector by a scalar value `K`.
//!
//! # Panics
//!
//! Binary operations acting on two vectors (Addition and Subtraction) will **panic**
//! if the vectors do not have the same number of scalars (dimensions).
//!
//! # Examples
//!
//! ```rust,ignore
//! use crate::vector::Vector;
//!
//! let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
//! let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
//!
//! // 1. Addition (Component-wise)
//! let sum = &v1 + &v2;
//! assert_eq!(sum, Vector::new(vec![5.0, 7.0, 9.0]));
//!
//! // 2. Subtraction (Component-wise)
//! let diff = &v2 - &v1;
//! assert_eq!(diff, Vector::new(vec![3.0, 3.0, 3.0]));
//!
//! // 3. Scalar Multiplication
//! let scaled = v1 * 2.0;
//! assert_eq!(scaled, Vector::new(vec![2.0, 4.0, 6.0]));
//!
//! // 4. Compound Assignment
//! let mut v_mut = Vector::new(vec![10.0, 10.0]);
//! v_mut += Vector::new(vec![5.0, 5.0]);
//! assert_eq!(v_mut, Vector::new(vec![15.0, 15.0]));
//! ```

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{linear_combination::linear_combination, matrix::Matrix, vector::Vector};

// -----------------------------------------------------------------------------
// Addition
// -----------------------------------------------------------------------------

fn add_assign_inner<K>(self_scalars: &mut [K], other_scalars: &[K])
where
    K: Copy + Neg + Add<Output = K>,
{
    assert_eq!(self_scalars.len(), other_scalars.len());

    for i in 0..self_scalars.len() {
        self_scalars[i] = self_scalars[i] + other_scalars[i];
    }
}

fn add_inner<K>(self_scalars: &[K], other_scalars: &[K]) -> Vec<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    let mut new = self_scalars.to_owned();
    add_assign_inner(&mut new, other_scalars);
    new
}

// Vector += Vector
impl<K> AddAssign<Vector<K>> for Vector<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    fn add_assign(&mut self, other: Vector<K>) {
        add_assign_inner(&mut self.scalars, &other.scalars);
    }
}

// Vector += &Vector
impl<K> AddAssign<&Vector<K>> for Vector<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    fn add_assign(&mut self, other: &Vector<K>) {
        add_assign_inner(&mut self.scalars, &other.scalars);
    }
}

// Vector + Vector
impl<K> Add for Vector<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    type Output = Vector<K>;

    fn add(mut self, other: Vector<K>) -> Self::Output {
        self += other;
        self
    }
}

// Vector + &Vector
impl<K> Add<&Vector<K>> for Vector<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    type Output = Vector<K>;

    fn add(mut self, other: &Vector<K>) -> Self::Output {
        self += other;
        self
    }
}

// &Vector + &Vector
impl<K> Add<&Vector<K>> for &Vector<K>
where
    K: Copy + Neg + Add<Output = K>,
{
    type Output = Vector<K>;

    fn add(self, other: &Vector<K>) -> Self::Output {
        let mut new: Vector<K> = self.clone();
        new += other;
        new
    }
}

// -----------------------------------------------------------------------------
// Substraction
// -----------------------------------------------------------------------------

fn sub_assign_inner<K>(self_scalars: &mut [K], other_scalars: &[K])
where
    K: Copy + Neg + Sub<Output = K>,
{
    assert_eq!(self_scalars.len(), other_scalars.len());

    for i in 0..self_scalars.len() {
        self_scalars[i] = self_scalars[i] - other_scalars[i];
    }
}

fn sub_inner<K>(self_scalars: &[K], other_scalars: &[K]) -> Vec<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    let mut new = self_scalars.to_owned();
    sub_assign_inner(&mut new, other_scalars);
    new
}

// Vector -= Vector
impl<K> SubAssign<Vector<K>> for Vector<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    fn sub_assign(&mut self, other: Vector<K>) {
        sub_assign_inner(&mut self.scalars, &other.scalars);
    }
}

// Vector -= &Vector
impl<K> SubAssign<&Vector<K>> for Vector<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    fn sub_assign(&mut self, other: &Vector<K>) {
        sub_assign_inner(&mut self.scalars, &other.scalars);
    }
}

// Vector - Vector
impl<K> Sub for Vector<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    type Output = Vector<K>;

    fn sub(self, other: Vector<K>) -> Self::Output {
        Vector::new(sub_inner(&self.scalars, &other.scalars))
    }
}

// Vector - &Vector
impl<K> Sub<&Vector<K>> for Vector<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    type Output = Vector<K>;

    fn sub(mut self, other: &Vector<K>) -> Self::Output {
        self -= other;
        self
    }
}

// &Vector - &Vector
impl<K> Sub<&Vector<K>> for &Vector<K>
where
    K: Copy + Neg + Sub<Output = K>,
{
    type Output = Vector<K>;

    fn sub(self, other: &Vector<K>) -> Self::Output {
        let mut new: Vector<K> = self.clone();
        new -= other;
        new
    }
}

// -----------------------------------------------------------------------------
// Coeff Multiplication
// -----------------------------------------------------------------------------

// Since Vector<K> is only multipliable by K, and K implements Copy,
// we don't need to implement the traits for &K

fn coeff_mul_assign_inner<K>(self_scalars: &mut [K], coeff: K)
where
    K: Copy + Neg + Mul<Output = K>,
{
    for scalar in self_scalars {
        *scalar = *scalar * coeff;
    }
}

fn coeff_mul_inner<K>(self_scalars: &[K], coeff: K) -> Vec<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    let mut new = self_scalars.to_owned();
    coeff_mul_assign_inner(&mut new, coeff);
    new
}

// Vector *= coeff
impl<K> MulAssign<K> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    fn mul_assign(&mut self, coeff: K) {
        coeff_mul_assign_inner(&mut self.scalars, coeff);
    }
}

// Vector * coeff
impl<K> Mul<K> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    type Output = Vector<K>;

    fn mul(mut self, coeff: K) -> Self::Output {
        self *= coeff;
        self
    }
}

// &Vector * coeff
impl<K> Mul<K> for &Vector<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    type Output = Vector<K>;

    fn mul(self, coeff: K) -> Self::Output {
        Vector::new(coeff_mul_inner(&self.scalars, coeff))
    }
}

// -----------------------------------------------------------------------------
// Vector Multiplication
// -----------------------------------------------------------------------------

fn vector_mul_assign_inner<K>(self_scalars: &mut [K], other_scalars: &[K])
where
    K: Copy + Neg + Mul<Output = K>,
{
    assert_eq!(self_scalars.len(), other_scalars.len());

    for i in 0..self_scalars.len() {
        self_scalars[i] = self_scalars[i] * other_scalars[i];
    }
}

fn vector_mul_inner<K>(self_scalars: &[K], other_scalars: &[K]) -> Vec<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    let mut new = self_scalars.to_owned();
    vector_mul_assign_inner(&mut new, other_scalars);
    new
}

// Vector *= Vector
impl<K> MulAssign<Vector<K>> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    fn mul_assign(&mut self, other: Vector<K>) {
        vector_mul_assign_inner(&mut self.scalars, &other.scalars);
    }
}

// Vector *= &Vector
impl<K> MulAssign<&Vector<K>> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    fn mul_assign(&mut self, other: &Vector<K>) {
        vector_mul_assign_inner(&mut self.scalars, &other.scalars);
    }
}

// Vector * Vector
impl<K> Mul<Vector<K>> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    type Output = Vector<K>;

    fn mul(mut self, other: Vector<K>) -> Self::Output {
        self *= other;
        self
    }
}

// Vector * &Vector
impl<K> Mul<&Vector<K>> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    type Output = Vector<K>;

    fn mul(self, other: &Vector<K>) -> Self::Output {
        Vector::new(vector_mul_inner(&self.scalars, &other.scalars))
    }
}

// &Vector * Vector
impl<K> Mul<Vector<K>> for &Vector<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    type Output = Vector<K>;

    fn mul(self, other: Vector<K>) -> Self::Output {
        Vector::new(vector_mul_inner(&self.scalars, &other.scalars))
    }
}

// &Vector * &Vector
impl<K> Mul<&Vector<K>> for &Vector<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    type Output = Vector<K>;

    fn mul(self, other: &Vector<K>) -> Self::Output {
        Vector::new(vector_mul_inner(&self.scalars, &other.scalars))
    }
}

// -----------------------------------------------------------------------------
// Matrix Multiplication
// -----------------------------------------------------------------------------

fn matrix_mul_assign_inner<K>(self_scalars: &mut [K], matrix_vectors: &[Vector<K>])
where
    K: Copy + Neg + Mul<Output = K> + Add<Output = K>,
{
    let combination = linear_combination(matrix_vectors, self_scalars).scalars;
    debug_assert_eq!(combination.len(), self_scalars.len());
    for i in 0..combination.len() {
        self_scalars[i] = combination[i];
    }
}

fn matrix_mul_inner<K>(scalars: &[K], matrix: &[Vector<K>]) -> Vector<K>
where
    K: Copy + Neg + Add<Output = K> + Mul<Output = K>,
{
    let combination = linear_combination(matrix, scalars).scalars;
    debug_assert_eq!(combination.len(), scalars.len());
    Vector::new(combination)
}

// Vector *= Matrix
impl<K> MulAssign<Matrix<K>> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K> + Add<Output = K>,
{
    fn mul_assign(&mut self, matrix: Matrix<K>) {
        matrix_mul_assign_inner(&mut self.scalars, &matrix.vectors);
    }
}

// Vector *= &Matrix
impl<K> MulAssign<&Matrix<K>> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K> + Add<Output = K>,
{
    fn mul_assign(&mut self, matrix: &Matrix<K>) {
        matrix_mul_assign_inner(&mut self.scalars, &matrix.vectors);
    }
}

// Vector * Matrix
impl<K> Mul<Matrix<K>> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K> + Add<Output = K>,
{
    type Output = Vector<K>;

    fn mul(mut self, other: Matrix<K>) -> Self::Output {
        self *= other;
        self
    }
}

// Vector * &Matrix
impl<K> Mul<&Matrix<K>> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K> + Add<Output = K>,
{
    type Output = Vector<K>;

    fn mul(self, other: &Matrix<K>) -> Self::Output {
        matrix_mul_inner(&self.scalars, &other.vectors)
    }
}

// &Vector * &Matrix
impl<K> Mul<&Matrix<K>> for &Vector<K>
where
    K: Copy + Neg + Mul<Output = K> + Add<Output = K>,
{
    type Output = Vector<K>;

    fn mul(self, matrix: &Matrix<K>) -> Self::Output {
        matrix_mul_inner(&self.scalars, &matrix.vectors)
    }
}

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::macros::vector;

    // -------------------------------------------------------------------------
    // TEST: ADDITION
    // -------------------------------------------------------------------------
    mod addition {
        use super::*;

        #[test]
        fn test_owned_owned() {
            let v1 = vector![1, 2, 3];
            let v2 = vector![4, 5, 6];
            let result = v1 + v2;
            assert_eq!(result.scalars, vec![5, 7, 9]);
        }

        #[test]
        fn test_owned_ref() {
            let v1 = vector![10, 20];
            let v2 = vector![1, 2];
            let result = v1 + &v2;
            assert_eq!(result.scalars, vec![11, 22]);
        }

        #[test]
        fn test_ref_ref() {
            let v1 = vector![1, 1];
            let v2 = vector![2, 2];
            let result = &v1 + &v2;
            assert_eq!(result.scalars, vec![3, 3]);
            // Ensure originals are still valid
            assert_eq!(v1.scalars, vec![1, 1]);
            assert_eq!(v2.scalars, vec![2, 2]);
        }

        #[test]
        fn test_assign_owned() {
            let mut v1 = vector![0, 10];
            v1 += vector![1, 1];
            assert_eq!(v1.scalars, vec![1, 11]);
        }

        #[test]
        fn test_assign_ref() {
            let mut v1 = vector![5, 5];
            let v2 = vector![1, 2];
            v1 += &v2;
            assert_eq!(v1.scalars, vec![6, 7]);
        }

        #[test]
        #[should_panic]
        fn test_panic_dim_mismatch() {
            let v1 = vector![1, 2];
            let v2 = vector![1, 2, 3];
            let _ = v1 + v2; // Should panic
        }
    }

    // -------------------------------------------------------------------------
    // TEST: SUBTRACTION
    // -------------------------------------------------------------------------
    mod subtraction {
        use super::*;

        #[test]
        fn test_owned_owned() {
            let v1 = vector![10, 20];
            let v2 = vector![1, 2];
            let result = v1 - v2;
            assert_eq!(result.scalars, vec![9, 18]);
        }

        #[test]
        fn test_ref_ref() {
            let v1 = vector![5, 5, 5];
            let v2 = vector![1, 1, 1];
            let result = &v1 - &v2;
            assert_eq!(result.scalars, vec![4, 4, 4]);
        }

        #[test]
        fn test_assign() {
            let mut v1 = vector![10, 10];
            let v2 = vector![3, 4];
            v1 -= v2;
            assert_eq!(v1.scalars, vec![7, 6]);
        }

        #[test]
        fn test_negative_result() {
            let v1 = vector![0, 0];
            let v2 = vector![1, 1];
            let result = v1 - v2;
            assert_eq!(result.scalars, vec![-1, -1]);
        }

        #[test]
        #[should_panic]
        fn test_panic_dim_mismatch() {
            let v1 = vector![1];
            let v2 = vector![1, 2];
            let _ = &v1 - &v2; // Should panic
        }
    }

    // -------------------------------------------------------------------------
    // TEST: SCALAR MULTIPLICATION (SCL)
    // -------------------------------------------------------------------------
    mod multiplication {
        use super::*;

        #[test]
        fn test_owned_scalar() {
            let v1 = vector![1, -2, 3];
            let result = v1 * 2;
            assert_eq!(result.scalars, vec![2, -4, 6]);
        }

        #[test]
        fn test_ref_scalar() {
            let v1 = vector![10, 20];
            let result = &v1 * 3;
            assert_eq!(result.scalars, vec![30, 60]);
            // Ensure original is preserved
            assert_eq!(v1.scalars, vec![10, 20]);
        }

        #[test]
        fn test_assign() {
            let mut v1 = vector![2, 4];
            v1 *= 2;
            assert_eq!(v1.scalars, vec![4, 8]);
        }

        #[test]
        fn test_zero() {
            let v1 = vector![1, 2, 3];
            let result = v1 * 0;
            assert_eq!(result.scalars, vec![0, 0, 0]);
        }
    }

    // -------------------------------------------------------------------------
    // TEST: VECTOR MULTIPLICATION (COMPONENT-WISE)
    // -------------------------------------------------------------------------
    mod vector_multiplication {
        use super::*;

        #[test]
        fn test_owned_owned() {
            let v1 = vector![1, 2, 3];
            let v2 = vector![4, 5, 6];
            let result = v1 * v2;
            assert_eq!(result.scalars, vec![4, 10, 18]);
        }

        #[test]
        fn test_owned_ref() {
            let v1 = vector![2, 3];
            let v2 = vector![4, 5];
            let result = v1 * &v2;
            assert_eq!(result.scalars, vec![8, 15]);
        }

        #[test]
        fn test_ref_owned() {
            let v1 = vector![1, 2];
            let v2 = vector![3, 4];
            let result = &v1 * v2;
            assert_eq!(result.scalars, vec![3, 8]);
        }

        #[test]
        fn test_ref_ref() {
            let v1 = vector![2, 3, 4];
            let v2 = vector![5, 6, 7];
            let result = &v1 * &v2;
            assert_eq!(result.scalars, vec![10, 18, 28]);
        }

        #[test]
        fn test_assign_owned() {
            let mut v1 = vector![1, 2, 3];
            v1 *= vector![2, 3, 4];
            assert_eq!(v1.scalars, vec![2, 6, 12]);
        }

        #[test]
        fn test_assign_ref() {
            let mut v1 = vector![1, 2];
            let v2 = vector![3, 4];
            v1 *= &v2;
            assert_eq!(v1.scalars, vec![3, 8]);
        }

        #[test]
        fn test_with_negatives() {
            let v1 = vector![-1, 2, -3];
            let v2 = vector![2, -3, 4];
            let result = v1 * v2;
            assert_eq!(result.scalars, vec![-2, -6, -12]);
        }

        #[test]
        fn test_with_zeros() {
            let v1 = vector![0, 5, 10];
            let v2 = vector![7, 0, 3];
            let result = v1 * v2;
            assert_eq!(result.scalars, vec![0, 0, 30]);
        }

        #[test]
        #[should_panic]
        fn test_panic_dim_mismatch() {
            let v1 = vector![1, 2];
            let v2 = vector![1, 2, 3];
            let _ = v1 * v2; // Should panic
        }
    }

    // -------------------------------------------------------------------------
    // TEST: MATRIX MULTIPLICATION
    // -------------------------------------------------------------------------
    mod matrix_multiplication {
        use crate::macros::matrix;

        use super::*;

        #[test]
        fn test_vector_matrix_multiplication() {
            // Test: [1, 2] * [[1, 2], [3, 4]] = [1*1 + 2*3, 1*2 + 2*4] = [7, 10]
            let vector = vector![1, 2];
            let matrix = matrix![[1, 2], [3, 4]];
            let result = vector * matrix;
            assert_eq!(result.scalars, vec![7, 10]);
        }

        #[test]
        fn test_vector_ref_matrix_multiplication() {
            // Test: &[1, 2] * &[[1, 2], [3, 4]] = [7, 10]
            let vector = vector![1, 2];
            let matrix = matrix![[1, 2], [3, 4]];
            let result = &vector * &matrix;
            assert_eq!(result.scalars, vec![7, 10]);
            // Ensure original values are preserved
            assert_eq!(vector.scalars, vec![1, 2]);
            assert_eq!(matrix.vectors[0].scalars, vec![1, 2]);
            assert_eq!(matrix.vectors[1].scalars, vec![3, 4]);
        }

        #[test]
        fn test_vector_matrix_multiplication_3x3() {
            // Test 3x3 matrix multiplication
            // [1, 2, 3] * [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            // = [1*1 + 2*4 + 3*7, 1*2 + 2*5 + 3*8, 1*3 + 2*6 + 3*9]
            // = [1 + 8 + 21, 2 + 10 + 24, 3 + 12 + 27] = [30, 36, 42]
            let vector = vector![1, 2, 3];
            let matrix = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
            let result = vector * matrix;
            assert_eq!(result.scalars, vec![30, 36, 42]);
        }

        #[test]
        fn test_vector_matrix_multiplication_with_zeros() {
            // Test with zero elements
            let vector = vector![0, 1];
            let matrix = matrix![[2, 3], [4, 5]];
            let result = vector * matrix;
            // [0, 1] * [[2, 3], [4, 5]] = [0*2 + 1*4, 0*3 + 1*5] = [4, 5]
            assert_eq!(result.scalars, vec![4, 5]);
        }

        #[test]
        fn test_vector_matrix_multiplication_with_negatives() {
            // Test with negative numbers
            let vector = vector![-1, 2];
            let matrix = matrix![[-2, 3], [4, -5]];
            let result = vector * matrix;
            // [-1, 2] * [[-2, 3], [4, -5]] = [-1*(-2) + 2*4, -1*3 + 2*(-5)] = [2 + 8, -3 - 10] = [10, -13]
            assert_eq!(result.scalars, vec![10, -13]);
        }

        #[test]
        fn test_vector_assign_matrix_multiplication() {
            // Test Vector *= Matrix assignment
            let mut vector = vector![1, 2];
            let matrix = matrix![[1, 2], [3, 4]];
            vector *= matrix;
            assert_eq!(vector.scalars, vec![7, 10]);
        }

        #[test]
        fn test_vector_assign_ref_matrix_multiplication() {
            // Test Vector *= &Matrix assignment
            let mut vector = vector![1, 2];
            let matrix = matrix![[1, 2], [3, 4]];
            vector *= &matrix;
            assert_eq!(vector.scalars, vec![7, 10]);
        }

        #[test]
        fn test_identity_matrix_multiplication() {
            // Test with identity matrix - should return the same vector
            let vector = vector![5, 7, 2];
            let identity_matrix = matrix![[1, 0, 0], [0, 1, 0], [0, 0, 1]];
            let result = vector * identity_matrix;
            assert_eq!(result.scalars, vec![5, 7, 2]);
        }

        #[test]
        fn test_simple_1x1_matrix_multiplication() {
            // Test 1x1 matrix (essentially scalar multiplication)
            let vector = vector![5];
            let matrix = matrix![[3]];
            let result = vector * matrix;
            assert_eq!(result.scalars, vec![15]);
        }

        #[test]
        fn test_matrix_vector_multiplication_linear_combination() {
            // Test that matrix multiplication works as a linear combination
            // Vector [a, b] * Matrix [[c, d], [e, f]] should equal a*[c, d] + b*[e, f]
            let vector = vector![2, 3];
            let matrix = matrix![[1, 4], [2, 5]];  // [[1, 4], [2, 5]]
            // Result should be 2*[1, 4] + 3*[2, 5] = [2, 8] + [6, 15] = [8, 23]
            let result = vector * matrix;
            assert_eq!(result.scalars, vec![8, 23]);
        }

        #[test]
        #[should_panic(expected = "assertion `left == right` failed")]
        fn test_matrix_vector_dimension_mismatch() {
            // Test that matrix and vector with incompatible dimensions cause a panic
            // Vector has 2 elements but matrix has 3 column vectors - this should panic
            let vector = vector![1, 2];
            let matrix = matrix![[1, 2], [3, 4], [5, 6]];  // 3x2 matrix but vector has 2 elements
            let _result = vector * matrix; // This should panic due to dimension mismatch in linear_combination
        }
    }
}
