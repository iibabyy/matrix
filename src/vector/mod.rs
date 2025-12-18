//! basis vector: fundamental unit vector (like i^ or j^​)
//! scalars: used to scale a vector
//! linear combination: sum of multiple vectors, each multiplied by a scalar weight (e.g., av+bw)

#![allow(dead_code)]

use std::ops::{Add, Mul, Neg, Sub};

mod linear_combination;
mod macros;

#[derive(Debug)]
pub struct Vector<K = f32>
where
    K: Copy + Neg,
{
    scalars: Vec<K>,
}

// -----------------------------------------------------------------------------
// UTILS FUNCTIONS
// -----------------------------------------------------------------------------
impl<K> Vector<K>
where
    K: Copy + Neg
{
    pub fn new() -> Self {
        Self { scalars: vec![] }
    }

    fn from_elem(elem: K, n: usize) -> Self
    where
        K: Clone,
    {
        Self {
            scalars: vec![elem; n],
        }
    }

    pub const fn len(&self) -> usize {
        self.scalars.len()
    }
}

// -----------------------------------------------------------------------------
// BASIC OPERATIONS
// -----------------------------------------------------------------------------
impl<K> Vector<K>
where
    K: Copy + Neg,
{
    fn add<T>(&mut self, other: &Vector<T>)
    where
        K: Add<T, Output = K> + From<T>,
        T: Copy + Neg,
    {
        *self += other;
    }

    fn sub<T>(&mut self, other: &Vector<T>)
    where
        K: Sub<T, Output = K> + From<T>,
        T: Copy + Neg,
    {
        *self -= other
    }

    fn scl<T>(&mut self, scale: T)
    where
        K: Mul<T, Output = K>,
        T: Copy + Neg,
    {
        *self *= scale
    }
}

// -----------------------------------------------------------------------------
// ARITHMETIC TRAITS
// -----------------------------------------------------------------------------

/* ADD */
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

/* SUB */
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

/* MUL */
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

// -----------------------------------------------------------------------------
// OTHER TRAITS
// -----------------------------------------------------------------------------
impl<K> std::fmt::Display for Vector<K>
where
    K: std::fmt::Display + Copy + Neg,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.scalars.iter().try_for_each(|c| writeln!(f, "[{c}]"))
    }
}

impl<T> std::default::Default for Vector<T>
where
    T: Copy + Neg,
{
    fn default() -> Self {
        Self { scalars: vec![] }
    }
}

impl<K> FromIterator<K> for Vector<K>
where
    K: Copy + Neg,
{
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        Self {
            scalars: Vec::from_iter(iter),
        }
    }
}

impl<K, I> From<I> for Vector<K>
where
    K: Copy + Neg,
    I: IntoIterator<Item = K>,
{
    fn from(iter: I) -> Self {
        Self::from_iter(iter)
    }
}
