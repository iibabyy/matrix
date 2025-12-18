use std::ops::{Add, Mul, Neg, Sub};

use crate::vector::Vector;

// -----------------------------------------------------------------------------
// Addition
// -----------------------------------------------------------------------------

impl<T, K> std::ops::AddAssign<Vector<T>> for Vector<K>
where
    K: Copy + Neg + Add<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    fn add_assign(&mut self, other: Vector<T>) {
        assert_eq!(self.len(), other.len());

        for i in 0..self.len() {
            self.scalars[i] = self.scalars[i] + other.scalars[i];
        }
    }
}

impl<T, K> std::ops::Add<Vector<T>> for Vector<K>
where
    K: Copy + Neg + Add<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    type Output = Self;

    fn add(mut self, other: Vector<T>) -> Self::Output {
        self += other;
        self
    }
}

impl<T, K> std::ops::AddAssign<&Vector<T>> for Vector<K>
where
    K: Copy + Neg + Add<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    fn add_assign(&mut self, other: &Vector<T>) {
        assert_eq!(self.len(), other.len());

        for i in 0..self.len() {
            self.scalars[i] = self.scalars[i] + other.scalars[i];
        }
    }
}

impl<T, K> std::ops::Add<&Vector<T>> for Vector<K>
where
    K: Copy + Neg + Add<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    type Output = Self;

    fn add(mut self, other: &Vector<T>) -> Self::Output {
        self += other;
        self
    }
}

impl<T, K> std::ops::AddAssign<Vector<T>> for &mut Vector<K>
where
    K: Copy + Neg + Add<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    fn add_assign(&mut self, other: Vector<T>) {
        assert_eq!(self.len(), other.len());

        for i in 0..self.len() {
            self.scalars[i] = self.scalars[i] + other.scalars[i];
        }
    }
}

impl<T, K> std::ops::Add<Vector<T>> for &mut Vector<K>
where
    K: Copy + Neg + Add<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    type Output = Self;

    fn add(mut self, other: Vector<T>) -> Self::Output {
        self += other;
        self
    }
}

impl<T, K> std::ops::AddAssign<&Vector<T>> for &mut Vector<K>
where
    K: Copy + Neg + Add<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    fn add_assign(&mut self, other: &Vector<T>) {
        assert_eq!(self.len(), other.len());

        for i in 0..self.len() {
            self.scalars[i] = self.scalars[i] + other.scalars[i];
        }
    }
}

impl<T, K> std::ops::Add<&Vector<T>> for &mut Vector<K>
where
    K: Copy + Neg + Add<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    type Output = Self;

    fn add(mut self, other: &Vector<T>) -> Self::Output {
        self += other;
        self
    }
}

// -----------------------------------------------------------------------------
// Substraction
// -----------------------------------------------------------------------------

impl<T, K> std::ops::SubAssign<Vector<T>> for Vector<K>
where
    K: Copy + Neg + Sub<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    fn sub_assign(&mut self, other: Vector<T>) {
        assert_eq!(self.len(), other.len());

        for i in 0..self.len() {
            self.scalars[i] = self.scalars[i] - other.scalars[i];
        }
    }
}

impl<T, K> std::ops::Sub<Vector<T>> for Vector<K>
where
    K: Copy + Neg + Sub<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    type Output = Self;

    fn sub(mut self, other: Vector<T>) -> Self::Output {
        self -= other;
        self
    }
}

impl<T, K> std::ops::SubAssign<&Vector<T>> for Vector<K>
where
    K: Copy + Neg + Sub<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    fn sub_assign(&mut self, other: &Vector<T>) {
        assert_eq!(self.len(), other.len());

        for i in 0..self.len() {
            self.scalars[i] = self.scalars[i] - other.scalars[i];
        }
    }
}

impl<T, K> std::ops::Sub<&Vector<T>> for Vector<K>
where
    K: Copy + Neg + Sub<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    type Output = Self;

    fn sub(mut self, other: &Vector<T>) -> Self::Output {
        self -= other;
        self
    }
}

impl<T, K> std::ops::SubAssign<Vector<T>> for &mut Vector<K>
where
    K: Copy + Neg + Sub<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    fn sub_assign(&mut self, other: Vector<T>) {
        assert_eq!(self.len(), other.len());

        for i in 0..self.len() {
            self.scalars[i] = self.scalars[i] - other.scalars[i];
        }
    }
}

impl<T, K> std::ops::Sub<Vector<T>> for &mut Vector<K>
where
    K: Copy + Neg + Sub<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    type Output = Self;

    fn sub(mut self, other: Vector<T>) -> Self::Output {
        self -= other;
        self
    }
}

impl<T, K> std::ops::SubAssign<&Vector<T>> for &mut Vector<K>
where
    K: Copy + Neg + Sub<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    fn sub_assign(&mut self, other: &Vector<T>) {
        assert_eq!(self.len(), other.len());

        for i in 0..self.len() {
            self.scalars[i] = self.scalars[i] - other.scalars[i];
        }
    }
}

impl<T, K> std::ops::Sub<&Vector<T>> for &mut Vector<K>
where
    K: Copy + Neg + Sub<T, Output = K> + From<T>,
    T: Copy + Neg,
{
    type Output = Self;

    fn sub(mut self, other: &Vector<T>) -> Self::Output {
        self -= other;
        self
    }
}

// -----------------------------------------------------------------------------
// Multiplication
// -----------------------------------------------------------------------------

impl<T, K> std::ops::MulAssign<T> for Vector<K>
where
    K: Copy + Neg + Mul<T, Output = K>,
    T: Copy,
{
    fn mul_assign(&mut self, scale: T) {
        for i in 0..self.len() {
            self.scalars[i] = self.scalars[i] * scale;
        }
    }
}

impl<T, K> std::ops::Mul<T> for Vector<K>
where
    K: Copy + Neg + Mul<T, Output = K>,
    T: Copy,
{
    type Output = Self;

    fn mul(mut self, scale: T) -> Self::Output {
        self *= scale;
        self
    }
}

impl<T, K> std::ops::MulAssign<T> for &mut Vector<K>
where
    K: Copy + Neg + Mul<T, Output = K>,
    T: Copy,
{
    fn mul_assign(&mut self, scale: T) {
        for i in 0..self.len() {
            self.scalars[i] = self.scalars[i] * scale;
        }
    }
}

impl<T, K> std::ops::Mul<T> for &mut Vector<K>
where
    K: Copy + Neg + Mul<T, Output = K>,
    T: Copy,
{
    type Output = Self;

    fn mul(mut self, scale: T) -> Self::Output {
        self *= scale;
        self
    }
}