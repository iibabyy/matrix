#![allow(dead_code)]

mod arithmetics;

use crate::vector::Vector;
use std::{
    ops::{Add, Index, IndexMut, Mul, Neg},
    slice::SliceIndex,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix<K: Copy = f32> {
    pub(crate) vectors: Vec<Vector<K>>,
}

// -----------------------------------------------------------------------------
// BASIC OPERATIONS
// -----------------------------------------------------------------------------
impl<K: Copy> Matrix<K>
where
    K: Copy,
{
    /// for details, go to [crate::matrix::arithmetics]
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K>
    where
        K: Mul<Output = K> + Add<Output = K>,
    {
        self * vec
    }

    /// for details, go to [crate::matrix::arithmetics]
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K>
    where
        K: Mul<Output = K> + Add<Output = K>,
    {
        self * mat
    }
}

// -----------------------------------------------------------------------------
// UTILS FUNCTIONS
// -----------------------------------------------------------------------------
impl<K: Copy> Matrix<K> {
    pub fn new(vectors: Vec<Vector<K>>) -> Self {
        let mut matrix = Self::default();
        for vector in vectors {
            // no verifications needed:
            // Matrix::push() already asserts that the vector is valid
            matrix.push(vector);
        }

        matrix
    }

    fn from_elem(elem: Vector<K>, n: usize) -> Self
    where
        K: Clone,
    {
        Self {
            vectors: vec![elem; n],
        }
    }

    pub const fn size(&self) -> usize {
        self.vectors.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn push(&mut self, vector: Vector<K>) {
        self.assert_valid(&vector);
        self.vectors.push(vector);
    }

    fn assert_valid(&self, vector: &Vector<K>) {
        if self.vectors.is_empty() {
            return;
        }

        assert_eq!(self.vectors[0].dimension(), vector.dimension());
    }

    pub const fn vectors(&self) -> &Vec<Vector<K>> {
        &self.vectors
    }

    pub(crate) fn swap_rows(&mut self, a: usize, b: usize)
    where
        K: Copy,
    {
        let mut temp;

        for i in 0..self.size() {
            temp = self[i][a];
            self[i][a] = self[i][b];
            self[i][b] = temp;
        }
    }

    pub(crate) fn max_value_in_col(&self, col: usize, from_row: usize) -> usize
    where
        K: PartialOrd + Copy,
    {
        let mut max_value = self[col][from_row];
        let mut max_value_row = from_row;

        for i in from_row + 1..self[0].dimension() {
            if max_value < self[col][i] {
                max_value = self[col][i];
                max_value_row = i;
            }
        }

        max_value_row
    }
}

// -----------------------------------------------------------------------------
// TRAITS IMPLEMENTATION
// -----------------------------------------------------------------------------
impl<K: Copy, I: SliceIndex<[Vector<K>]>> Index<I> for Matrix<K> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.vectors[index]
    }
}

impl<K: Copy, I: SliceIndex<[Vector<K>]>> IndexMut<I> for Matrix<K> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.vectors[index]
    }
}

impl<K: Copy> std::fmt::Display for Matrix<K>
where
    K: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vecs_fmt = self
            .vectors
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "[{vecs_fmt}]")
    }
}

impl<T: Copy> std::default::Default for Matrix<T> {
    fn default() -> Self {
        Self { vectors: vec![] }
    }
}

impl<K: Copy> FromIterator<Vector<K>> for Matrix<K> {
    fn from_iter<I: IntoIterator<Item = Vector<K>>>(iter: I) -> Self {
        Self {
            vectors: Vec::from_iter(iter),
        }
    }
}

impl<T, K: Copy> From<T> for Matrix<K>
where
    T: IntoIterator<Item = Vector<K>>,
{
    fn from(iter: T) -> Self {
        Self::from_iter(iter)
    }
}

impl<K: Copy> Neg for Matrix<K>
where
    K: Neg,
    Vector<K>: Neg<Output = Vector<<K as Neg>::Output>>,
    <K as Neg>::Output: Copy,
{
    type Output = Matrix<<K as Neg>::Output>;

    fn neg(self) -> Self::Output {
        let vec: Vec<Vector<<K as Neg>::Output>> = self.vectors.into_iter().map(Neg::neg).collect();
        Matrix::new(vec)
    }
}
