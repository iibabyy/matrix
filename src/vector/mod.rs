//! basis vector: fundamental unit vector (like i^ or j^​)
//! scalars: used to scale a vector
//! linear combination: sum of multiple vectors, each multiplied by a scalar weight (e.g., av+bw)

#![allow(dead_code)]

use std::ops::{Add, Mul, Neg, Sub};

mod arithmetics;
mod linear_combination;
mod macros;

#[derive(Debug, Clone)]
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
    K: Copy + Neg,
{
    pub fn new() -> Self {
        Self::from(vec![])
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
    fn add(&mut self, other: &Vector<K>)
    where
        K: Add<Output = K>,
    {
        // for details, go to src/vector/arithmetics.rs
        *self += other;
    }

    fn sub(&mut self, other: &Vector<K>)
    where
        K: Sub<Output = K>,
    {
        // for details, go to src/vector/arithmetics.rs
        *self -= other
    }

    fn scl(&mut self, scale: K)
    where
        K: Mul<Output = K>,
    {
        // for details, go to src/vector/arithmetics.rs
        *self *= scale
    }
}

// -----------------------------------------------------------------------------
// TRAITS Implementation
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
        Self::new()
    }
}

impl<K> FromIterator<K> for Vector<K>
where
    K: Copy + Neg,
{
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl<K> From<Vec<K>> for Vector<K>
where
    K: Copy + Neg,
{
    fn from(scalars: Vec<K>) -> Self {
        Self { scalars }
    }
}
