//! basis vector: fundamental unit vector (like i^ or j^​)
//! scalars: used to scale a vector
//! linear combination: sum of multiple vectors, each multiplied by a scalar weight (e.g., av+bw)

#![allow(dead_code)]

use std::ops::{Add, Mul, Neg, Sub};

mod arithmetics;
mod linear_combination;
mod dot_product;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector<K = f32>
where
    K: Copy + Neg,
{
    pub(crate) scalars: Vec<K>,
}

// -----------------------------------------------------------------------------
// UTILS FUNCTIONS
// -----------------------------------------------------------------------------
impl<K> Vector<K>
where
    K: Copy + Neg,
{
    pub fn new(scalars: Vec<K>) -> Self {
        assert!(scalars.len() > 0);
        Self { scalars }
    }

    fn from_elem(elem: K, n: usize) -> Self
    where
        K: Clone,
    {
        assert!(n > 0);
        Self {
            scalars: vec![elem; n],
        }
    }

    pub const fn len(&self) -> usize {
        self.scalars.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.scalars.is_empty()
    }

    pub const fn scalars(&self) -> &Vec<K> {
        &self.scalars
    }
}

// -----------------------------------------------------------------------------
// BASIC OPERATIONS
// -----------------------------------------------------------------------------
impl<K> Vector<K>
where
    K: Copy + Neg,
{
    /// for details, go to [crate::vector::arithmetics]
    fn add(&mut self, other: &Vector<K>)
    where
        K: Add<Output = K>,
    {
        *self += other;
    }

    /// for details, go to [crate::vector::arithmetics]
    fn sub(&mut self, other: &Vector<K>)
    where
        K: Sub<Output = K>,
    {
        *self -= other
    }

    /// for details, go to [crate::vector::arithmetics]
    fn scl(&mut self, scale: K)
    where
        K: Mul<Output = K>,
    {
        *self *= scale
    }
}

// -----------------------------------------------------------------------------
// TRAITS IMPLEMENTATION
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
        Self::new(vec![])
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

impl<T, K> From<T> for Vector<K>
where
    T: IntoIterator<Item = K>,
    K: Copy + Neg,
{
    fn from(iter: T) -> Self {
        Self::from_iter(iter)
    }
}

impl<K> Neg for Vector<K>
where
    K: Copy + Neg,
    <K as Neg>::Output: Copy + Neg,
{
    type Output = Vector<<K as Neg>::Output>;

    fn neg(self) -> Self::Output {
        let vec: Vec<<K as Neg>::Output> = self.scalars.into_iter().map(Neg::neg).collect();
        Vector::new(vec)
    }
}
