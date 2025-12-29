//! basis vector: fundamental unit vector (like i^ or j^​)
//! scalars: used to scale a vector
//! linear combination: sum of multiple vectors, each multiplied by a scalar weight (e.g., av+bw)

#![allow(dead_code)]

use std::ops::{Add, Mul, Neg, Sub};

mod arithmetics;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector<K: Copy = f32> {
    pub(crate) scalars: Vec<K>,
}

// -----------------------------------------------------------------------------
// BASIC OPERATIONS
// -----------------------------------------------------------------------------
impl<K: Copy> Vector<K> {
    /// for details, go to [crate::vector::arithmetics]
    pub fn add(&mut self, other: &Vector<K>)
    where
        K: Add<Output = K>,
    {
        *self += other;
    }

    /// for details, go to [crate::vector::arithmetics]
    pub fn sub(&mut self, other: &Vector<K>)
    where
        K: Sub<Output = K>,
    {
        *self -= other
    }

    /// for details, go to [crate::vector::arithmetics]
    pub fn scl(&mut self, scale: K)
    where
        K: Mul<Output = K>,
    {
        *self *= scale
    }
}

// -----------------------------------------------------------------------------
// UTILS FUNCTIONS
// -----------------------------------------------------------------------------
impl<K: Copy> Vector<K> {
    pub const fn new(scalars: Vec<K>) -> Self {
        assert!(!scalars.is_empty());
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

    pub const fn dimension(&self) -> usize {
        self.scalars.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.scalars.is_empty()
    }

    pub const fn scalars(&self) -> &Vec<K> {
        &self.scalars
    }

    pub fn push(&mut self, value: K) {
        self.scalars.push(value)
    }
}

// -----------------------------------------------------------------------------
// TRAITS IMPLEMENTATION
// -----------------------------------------------------------------------------
impl<K: Copy> std::ops::Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.scalars[index]
    }
}

impl<K: Copy> std::ops::IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.scalars[index]
    }
}

impl<K: Copy> std::fmt::Display for Vector<K>
where
    K: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.scalars.iter().try_for_each(|c| writeln!(f, "[{c}]"))
    }
}

impl<K: Copy> std::default::Default for Vector<K> {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl<K: Copy> FromIterator<K> for Vector<K> {
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        Self {
            scalars: Vec::from_iter(iter),
        }
    }
}

impl<T, K: Copy> From<T> for Vector<K>
where
    T: IntoIterator<Item = K>,
{
    fn from(iter: T) -> Self {
        Self::from_iter(iter)
    }
}

impl<K: Copy> Neg for Vector<K>
where
    K: Neg,
    <K as Neg>::Output: Copy,
{
    type Output = Vector<<K as Neg>::Output>;

    fn neg(self) -> Self::Output {
        let vec: Vec<<K as Neg>::Output> = self.scalars.into_iter().map(Neg::neg).collect();
        Vector::new(vec)
    }
}
