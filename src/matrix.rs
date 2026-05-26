pub mod arithmetics;
pub mod functions;

use crate::{scalar::Scalar, vector::Vector};
use std::{
    ops::{Index, IndexMut, Neg},
    slice::SliceIndex,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix<K: Scalar = f32> {
    pub(crate) vectors: Vec<Vector<K>>,
}

// -----------------------------------------------------------------------------
// BASIC OPERATIONS
// -----------------------------------------------------------------------------
impl<K: Scalar> Matrix<K>
{
    /// for details, go to [crate::matrix::arithmetics]
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        self * vec
    }

    /// for details, go to [crate::matrix::arithmetics]
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        self * mat
    }
}

// -----------------------------------------------------------------------------
// UTILS FUNCTIONS
// -----------------------------------------------------------------------------
impl<K: Scalar> Matrix<K> {
    pub fn new(vectors: Vec<Vector<K>>) -> Self {
        let mut matrix = Self::default();
        for vector in vectors {
            // no verifications needed:
            // Matrix::push() already asserts that the vector is valid
            matrix.push(vector);
        }

        matrix
    }

    pub fn rows(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.vectors[0].size()
        }
    }

    pub const fn cols(&self) -> usize {
        self.vectors.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.cols() == 0
    }

    pub fn is_square(&self) -> bool {
        self.cols() == self.rows()
    }

    pub fn push(&mut self, vector: Vector<K>) {
        assert!(self.is_valid_vector(&vector));
        self.vectors.push(vector);
    }

    fn is_valid_vector(&self, vector: &Vector<K>) -> bool {
        if self.vectors.is_empty() {
            return true;
        }

        self.vectors[0].size() == vector.size()
    }

    pub const fn vectors(&self) -> &Vec<Vector<K>> {
        &self.vectors
    }

    #[expect(dead_code)]
    pub fn row(&self, index: usize) -> impl Iterator<Item = &K> {
        self.vectors.iter().map(move |vec| &vec[index])
    }

    #[expect(dead_code)]
    pub fn row_mut(&mut self, index: usize) -> impl Iterator<Item = &mut K> {
        self.vectors.iter_mut().map(move |vec| &mut vec[index])
    }

    pub fn as_rows(&self) -> impl Iterator<Item = Vec<&K>> {
        use crate::rows::IntoRows;
        self.vectors.iter().into_rows()
    }

    pub fn as_cols(&self) -> impl Iterator<Item = &Vector<K>> {
        self.vectors.iter()
    }
}

impl Matrix<f32> {
    pub fn identity(length: usize) -> Self {
        let mut matrix = Self::new(Vec::with_capacity(length));

        for i in 0..length {
            let mut vec = vec![0.; length];
            vec[i] = 1.;
            matrix.push(Vector::new(vec));
        }

        matrix
    }
}

// -----------------------------------------------------------------------------
// TRAITS IMPLEMENTATION
// -----------------------------------------------------------------------------
impl<K: Scalar, I: SliceIndex<[Vector<K>]>> Index<I> for Matrix<K> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.vectors[index]
    }
}

impl<K: Scalar, I: SliceIndex<[Vector<K>]>> IndexMut<I> for Matrix<K> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.vectors[index]
    }
}

impl<K: Scalar> std::fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	for row in self.as_rows() {
        	let line = row
     			.iter()
          		.map(ToString::to_string)
               	.collect::<Vec<_>>()
               	.join(", ");

        	writeln!(f, "{line}")?;
     	}

      	Ok(())
    }
}

impl<K: Scalar> std::default::Default for Matrix<K> {
    fn default() -> Self {
        Self { vectors: vec![] }
    }
}

impl<K: Scalar> FromIterator<Vector<K>> for Matrix<K> {
    fn from_iter<I: IntoIterator<Item = Vector<K>>>(iter: I) -> Self {
        Self {
            vectors: Vec::from_iter(iter),
        }
    }
}

impl<T, K: Scalar> From<T> for Matrix<K>
where
    T: IntoIterator<Item = Vector<K>>,
{
    fn from(iter: T) -> Self {
        Self::from_iter(iter)
    }
}

impl<K: Scalar> Neg for Matrix<K> {
    type Output = Matrix<<K as Neg>::Output>;

    fn neg(self) -> Self::Output {
    	use Neg;

        let vec: Vec<Vector<<K as Neg>::Output>> = self.vectors
         	.into_iter()
          	.map(Neg::neg)
          	.collect();

        Matrix::new(vec)
    }
}
