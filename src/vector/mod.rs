//! basis vector: fundamental unit vector (like i^ or j^​)
//! scalars: used to scale a vector
//! linear combination: sum of multiple vectors, each multiplied by a scalar weight (e.g., av+bw)

#![allow(dead_code)]

use std::ops::{Add, Mul, Neg, Sub};

mod arithmetics;
mod linear_combination;
mod macros;

#[cfg(test)]
mod tests;

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
        // for details, go to src/vector/arithmetics.rs
        *self += other;
    }

    fn sub<T>(&mut self, other: &Vector<T>)
    where
        K: Sub<T, Output = K> + From<T>,
        T: Copy + Neg,
    {
        // for details, go to src/vector/arithmetics.rs
        *self -= other
    }

    fn scl<T>(&mut self, scale: T)
    where
        K: Mul<T, Output = K>,
        T: Copy + Neg,
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
