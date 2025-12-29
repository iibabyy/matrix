#![allow(dead_code)]

mod arithmetics;

use crate::vector::Vector;
use std::ops::{Add, Mul, Neg};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix<K = f32>
where
    K: Copy + Neg,
{
    pub(crate) vectors: Vec<Vector<K>>,
}

// -----------------------------------------------------------------------------
// BASIC OPERATIONS
// -----------------------------------------------------------------------------
impl<K> Matrix<K>
where
    K: Copy + Neg,
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
impl<K> Matrix<K>
where
    K: Copy + Neg,
{
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

    pub(crate) fn swap_rows(&mut self, a: usize, b: usize) {
        let mut temp;

        for i in 0..self.size() {
            temp = self[i][a];
            self[i][a] = self[i][b];
            self[i][b] = temp;
        }
    }

    pub(crate) fn max_value_in_col(&self, col: usize, from_row: usize) -> usize
    where
        K: PartialOrd,
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

    pub(crate) fn rows<'a>(&'a self) -> impl Iterator<Item = Vec<&'a K>> + 'a {
        let mut col_iter: Vec<_> = self.vectors
            .iter()
            .map(|col| col.scalars.iter())
            .collect();

        let rows = std::iter::from_fn(move || {
            let column: Option<Vec<&K>> = col_iter
                .iter_mut()
                .map(|iter| iter.next())
                .collect();

            column
        });

        rows
    }

    pub(crate) fn rows_mut(&mut self) -> impl Iterator<Item = Vec<&mut K>> {
        let mut vec_iter: Vec<_> = self.vectors
            .iter_mut()
            .map(|vec| vec.scalars.iter_mut())
            .collect();

        let rows = std::iter::from_fn(move || {
            let column: Option<Vec<&mut K>> = vec_iter
                .iter_mut()
                .map(|iter| iter.next())
                .collect();

            column
        });

        rows
    }

}

// -----------------------------------------------------------------------------
// TRAITS IMPLEMENTATION
// -----------------------------------------------------------------------------
impl<K> std::ops::Index<usize> for Matrix<K>
where
    K: Copy + Neg,
{
    type Output = Vector<K>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vectors[index]
    }
}

impl<K> std::ops::IndexMut<usize> for Matrix<K>
where
    K: Copy + Neg,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vectors[index]
    }
}

impl<K> std::fmt::Display for Matrix<K>
where
    K: std::fmt::Display + Copy + Neg,
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
