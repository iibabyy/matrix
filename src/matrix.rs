#![allow(dead_code)]

mod arithmetics;
mod macros;

use std::ops::Neg;
use crate::vector::Vector;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix<K = f32>
where
    K: Copy + Neg,
{
    vectors: Vec<Vector<K>>,
}

// -----------------------------------------------------------------------------
// UTILS FUNCTIONS
// -----------------------------------------------------------------------------
impl<K> Matrix<K>
where
    K: Copy + Neg,
{
    pub fn new(vectors: Vec<Vector<K>>) -> Self {
        let mut matrix = Self::default();
        for vector in vectors {
            // Matrix::push() asserts that the vector is valid
            matrix.push(vector);
        }

        matrix
    }

    fn from_elem(elem: Vector<K>, n: usize) -> Self
    where
        K: Clone,
    {
        Self { vectors: vec![elem; n] }
    }

    pub const fn len(&self) -> usize {
        self.vectors.len()
    }

    pub fn push(&mut self, vector: Vector<K>) {
        self.assert_valid(&vector);
        self.vectors.push(vector);
    }

    fn assert_valid(&self, vector: &Vector<K>) {
        if self.vectors.is_empty() {
            return
        }

        assert_eq!(self.vectors[0].len(), vector.len());
    }

    pub const fn vectors(&self) -> &Vec<Vector<K>> {
        &self.vectors
    }
}

// -----------------------------------------------------------------------------
// TRAITS IMPLEMENTATION
// -----------------------------------------------------------------------------
impl<K> std::fmt::Display for Matrix<K>
where
    K: std::fmt::Display + Copy + Neg,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let vecs_fmt = self.vectors.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n");
		write!(f, "[{vecs_fmt}]")
    }
}

impl<T> std::default::Default for Matrix<T>
where
    T: Copy + Neg,
{
    fn default() -> Self {
        Self { vectors: vec![] }
    }
}

impl<K> FromIterator<Vector<K>> for Matrix<K>
where
    K: Copy + Neg,
{
    fn from_iter<I: IntoIterator<Item = Vector<K>>>(iter: I) -> Self {
        Self {
            vectors: Vec::from_iter(iter),
        }
    }
}

impl<T, K> From<T> for Matrix<K>
where
    T: IntoIterator<Item = Vector<K>>,
    K: Copy + Neg,
{
    fn from(iter: T) -> Self {
        Self::from_iter(iter)
    }
}

impl<K> Neg for Matrix<K>
where
    K: Copy + Neg,
	Vector<K>: Neg<Output = Vector<<K as Neg>::Output>>,
	<K as Neg>::Output: Copy + Neg,
{
    type Output = Matrix<<K as Neg>::Output>;

    fn neg(self) -> Self::Output {
        let vec: Vec<Vector<<K as Neg>::Output>> = self.vectors.into_iter().map(Neg::neg).collect();
        Matrix::new(vec)
    }
}
