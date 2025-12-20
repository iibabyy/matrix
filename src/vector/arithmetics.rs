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

use crate::vector::Vector;

#[cfg(test)]
mod tests;

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
// Multiplication
// -----------------------------------------------------------------------------

// Since Vector<K> is only multipliable by K, and K implements Copy,
// we don't need to implement the traits for &K

fn mul_assign_inner<K>(self_scalars: &mut [K], coeff: K)
where
    K: Copy + Neg + Mul<Output = K>,
{
    for scalar in self_scalars {
        *scalar = *scalar * coeff;
    }
}

fn mul_inner<K>(self_scalars: &[K], coeff: K) -> Vec<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    let mut new = self_scalars.to_owned();
    mul_assign_inner(&mut new, coeff);
    new
}

// Vector *= coeff
impl<K> MulAssign<K> for Vector<K>
where
    K: Copy + Neg + Mul<Output = K>,
{
    fn mul_assign(&mut self, coeff: K) {
        mul_assign_inner(&mut self.scalars, coeff);
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
        Vector::new(mul_inner(&self.scalars, coeff))
    }
}
