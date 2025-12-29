use std::ops::{Div, Mul, Sub};

use crate::Matrix;

impl<K> Matrix<K>
where
    K: Copy + PartialOrd + Default,
    K: Div<Output = K> + Mul<Output = K> + Sub<Output = K>,
{
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut matrix = self.clone();
        let rows = matrix.rows();
        let cols = matrix.cols();

        let mut pivot_row = 0;
        for col in 0..cols {
            // stop if we have run out of pivot rows
            if pivot_row >= rows {
                break;
            }

            // find the pivot
            let selected_row = matrix.max_value_in_col(col, pivot_row);

            // stop if no pivot at this row
            if matrix[col][selected_row] == K::default() {
                continue;
            }

            // swap the current pivot row with the selected best row
            matrix.swap_rows(pivot_row, selected_row);

            // make everything below the pivot zero
            for row in pivot_row + 1..cols {
                if matrix[row][col] == K::default() {
                    continue;
                }

                let factor = matrix[col][row] / matrix[col][pivot_row];
                for i in 0..cols {
                    matrix[i][row] = matrix[i][row] - (matrix[i][pivot_row] * factor)
                }
            }

            pivot_row += 1;
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix; // Assuming you have a macro, otherwise use Matrix::from

    // Helper to compare two matrices with a small tolerance for floating point errors
    fn assert_matrix_approx_eq(m1: &Matrix<f32>, m2: &Matrix<f32>) {
        let epsilon = 1e-5;

        assert_eq!(m1.rows(), m2.rows(), "Row count mismatch");
        assert_eq!(m1.cols(), m2.cols(), "Col count mismatch");

        for i in 0..m1.rows() {
            for j in 0..m1.cols() {
                let val1 = m1[i][j];
                let val2 = m2[i][j];
                assert!(
                    (val1 - val2).abs() < epsilon,
                    "Mismatch at [{i}][{j}]: {} != {} (diff: {})",
                    val1,
                    val2,
                    (val1 - val2).abs()
                );
            }
        }
    }

    // ==========================================
    // Subject Test Cases
    // ==========================================

    #[test]
    fn test_subject_rref_identity() {
        let u = matrix!(vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.],);
        // Identity is already in RREF
        let res = u.row_echelon();
        assert_matrix_approx_eq(&res, &u);
    }

    #[test]
    fn test_subject_rref_2x2_invertible() {
        // [1, 2]       [1, 0]
        // [3, 4]  -->  [0, 1]
        let u = matrix!(vec![1., 2.], vec![3., 4.],);
        let expected = matrix!(vec![1., 0.], vec![0., 1.],);

        let res = u.row_echelon();
        assert_matrix_approx_eq(&res, &expected);
    }

    #[test]
    fn test_subject_rref_2x2_singular() {
        // [1, 2]       [1, 2]
        // [2, 4]  -->  [0, 0]
        let u = matrix!(vec![1., 2.], vec![2., 4.],);
        let expected = matrix!(vec![1., 2.], vec![0., 0.],);

        let res = u.row_echelon();
        assert_matrix_approx_eq(&res, &expected);
    }

    #[test]
    fn test_subject_rref_complex() {
        let u = matrix!(
            vec![8., 5., -2., 4., 28.],
            vec![4., 2.5, 20., 4., -4.],
            vec![8., 5., 1., 4., 17.],
        );

        // Expected values from the subject text
        let expected = matrix!(
            vec![1.0, 0.625, 0.0, 0.0, -12.1666667],
            vec![0.0, 0.0, 1.0, 0.0, -3.6666667],
            vec![0.0, 0.0, 0.0, 1.0, 29.5],
        );

        let res = u.row_echelon();
        assert_matrix_approx_eq(&res, &expected);
    }

    // ==========================================
    // Additional Logic Tests
    // ==========================================

    #[test]
    fn test_rref_zero_matrix() {
        // [0, 0]  -->  [0, 0]
        // [0, 0]       [0, 0]
        let u = matrix!(vec![0., 0.], vec![0., 0.],);
        let res = u.row_echelon();
        assert_matrix_approx_eq(&res, &u);
    }

    #[test]
    fn test_rref_tall_matrix() {
        // More rows than columns
        // [1, 2]
        // [3, 4]  -->  Identity on top, zero row bottom
        // [5, 6]
        let u = matrix!(vec![1., 2.], vec![3., 4.], vec![5., 6.],);
        let expected = matrix!(vec![1., 0.], vec![0., 1.], vec![0., 0.],);

        let res = u.row_echelon();
        assert_matrix_approx_eq(&res, &expected);
    }

    #[test]
    fn test_rref_pivot_moving() {
        // Pivot shouldn't be in the first column for the second row
        // [0, 1, 2]
        // [0, 3, 6] -> [0, 1, 2] and zero row
        let u = matrix!(vec![0., 1., 2.], vec![0., 3., 6.],);
        let expected = matrix!(vec![0., 1., 2.], vec![0., 0., 0.],);

        let res = u.row_echelon();
        assert_matrix_approx_eq(&res, &expected);
    }
}
