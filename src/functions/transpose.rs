use crate::{Matrix, Vector};

impl<K> Matrix<K>
where
    K: Copy,
{
    pub fn transpose(&self) -> Matrix<K> {
        if self.is_empty() {
            // empty Matrix
            return Matrix::default();
        }

        let rows = self.cols();
        let cols = self[0].dimension();

        // New matrix will have 'cols' rows and 'rows' columns
        let mut transposed_vectors = Vec::with_capacity(cols);

        for j in 0..cols {
            let mut new_row = Vec::with_capacity(rows);
            for i in 0..rows {
                new_row.push(self[i][j]);
            }
            transposed_vectors.push(Vector::new(new_row));
        }

        Matrix::new(transposed_vectors)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn test_transpose_square() {
        // [1, 2]  ->  [1, 3]
        // [3, 4]      [2, 4]
        let m = matrix![[1, 2], [3, 4]];
        let t = m.transpose();

        assert_eq!(t.vectors[0].scalars, vec![1, 3]);
        assert_eq!(t.vectors[1].scalars, vec![2, 4]);
    }

    #[test]
    fn test_transpose_rectangular_wide() {
        // (2x3) Matrix
        // [1, 2, 3]  ->  [1, 4]
        // [4, 5, 6]      [2, 5]
        //                [3, 6]
        let m = matrix![[1, 2, 3], [4, 5, 6]];
        let t = m.transpose();

        assert_eq!(t.vectors.len(), 3); // Now 3 rows
        assert_eq!(t.vectors[0].scalars, vec![1, 4]);
        assert_eq!(t.vectors[1].scalars, vec![2, 5]);
        assert_eq!(t.vectors[2].scalars, vec![3, 6]);
    }

    #[test]
    fn test_transpose_rectangular_tall() {
        // (3x1) Matrix (Column Vector)
        // [1]  ->  [1, 2, 3] (Row Vector)
        // [2]
        // [3]
        let m = matrix![[1], [2], [3]];
        let t = m.transpose();

        assert_eq!(t.vectors.len(), 1);
        assert_eq!(t.vectors[0].scalars, vec![1, 2, 3]);
    }

    #[test]
    fn test_transpose_identity() {
        // The transpose of an identity matrix is itself
        let m = matrix![[1., 0.], [0., 1.]];
        let t = m.transpose();
        assert_eq!(t.vectors[0].scalars, vec![1., 0.]);
        assert_eq!(t.vectors[1].scalars, vec![0., 1.]);
    }

    #[test]
    fn test_transpose_twice() {
        // (A^T)^T = A
        let m = matrix![[1, 5, 9], [2, 6, 10]];
        let t_twice = m.transpose().transpose();

        assert_eq!(m.vectors[0].scalars, t_twice.vectors[0].scalars);
        assert_eq!(m.vectors[1].scalars, t_twice.vectors[1].scalars);
    }
}
