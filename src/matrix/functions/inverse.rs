use crate::{Matrix, matrix::functions::row_echelon::RowEchelonOperation};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    SingularMatrix,
}

impl Matrix<f32> {
    pub fn inverse(&mut self) -> Result<Self, Error> {
        assert!(self.is_square());

        let (mut matrix, details) = self.row_echelon_with_details();

        if details.tracked_pivots.len() != matrix.rows() {
            return Err(Error::SingularMatrix);
        }

        let mut ops = details.operations;
        ops.extend(matrix.back_substitution());

        let mut identity_matrix = Matrix::identity(self.cols());
        identity_matrix.apply_multiple(ops);

        Ok(identity_matrix)
    }

    /// Uses elementary row operations to put zeros above the all the pivots
    #[doc(hidden)]
    pub fn back_substitution(&mut self) -> Vec<RowEchelonOperation<f32>> {
        assert!(self.is_row_echelon_form());

        let mut matrix = self.clone();
        let mut operations = vec![];

        for i in 1..matrix.cols() {
            let ops = matrix.nullify_rows_above_pivot(i, i);
            operations.extend(ops)
        }

        operations
    }

    /// Uses elementary row operations to put zeros above the pivot element
    #[doc(hidden)]
    fn nullify_rows_above_pivot(
        &mut self,
        pivot_col: usize,
        pivot_row: usize,
    ) -> Vec<RowEchelonOperation<f32>> {
        // we assume that pivot == 1

        let mut operations = vec![];

        for row in 0..pivot_row {
            let factor = self[pivot_col][row];
            if factor == 0. {
                continue;
            }

            let op = self.row_add(row, pivot_row, -factor);
            operations.push(op);
        }

        operations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix; // Assuming you have this macro

    // Helper to compare floating point matrices with tolerance
    fn assert_matrix_approx_eq(a: &Matrix<f32>, b: &Matrix<f32>) {
        let epsilon = 1e-6; // Slightly loose tolerance for complex inversions
        assert_eq!(a.cols(), b.cols(), "Column counts differ");
        if a.cols() > 0 {
            assert_eq!(a[0].scalars.len(), b[0].scalars.len(), "Row counts differ");
        }

        for i in 0..a.cols() {
            for j in 0..a[0].scalars.len() {
                let val_a = a[i][j];
                let val_b = b[i][j];
                assert!(
                    (val_a - val_b).abs() < epsilon,
                    "Mismatch at Col {}, Row {}: {} != {}",
                    i, j, val_a, val_b
                );
            }
        }
    }

    // ==========================================
    // Subject Test Cases
    // ==========================================

    #[test]
    fn test_subject_inverse_identity() {
        // Inverse of Identity is Identity
        let mut u = matrix![
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.]
        ];
        
        let result = u.inverse().expect("Matrix should be invertible");
        let expected = matrix![
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.]
        ];
        
        assert_matrix_approx_eq(&result, &expected);
    }

    #[test]
    fn test_subject_inverse_diagonal() {
        // Inverse of scaling matrix is 1/scale
        let mut u = matrix![
            [2., 0., 0.],
            [0., 2., 0.],
            [0., 0., 2.]
        ];

        let result = u.inverse().expect("Matrix should be invertible");
        let expected = matrix![
            [0.5, 0., 0.],
            [0., 0.5, 0.],
            [0., 0., 0.5]
        ];

        assert_matrix_approx_eq(&result, &expected);
    }

    #[test]
    fn test_subject_inverse_complex() {
        let mut u = matrix![
            [8., 5., -2.],
            [4., 7., 20.],
            [7., 6., 1.]
        ];

        let result = u.inverse().expect("Matrix should be invertible");
        
        // Exact values from subject
        let expected = matrix![
            [0.649425287, 0.097701149, -0.655172414],
            [-0.781609195, -0.126436782, 0.965517241],
            [0.143678161, 0.074712644, -0.206896552]
        ];

        assert_matrix_approx_eq(&result, &expected);
    }

    // ==========================================
    // Additional Unit Tests
    // ==========================================

    #[test]
    fn test_inverse_singular_matrix() {
        // A matrix with a zero row is singular (determinant is 0)
        let mut u = matrix![
            [1., 2., 3.],
            [0., 0., 0.], // Zero row -> Singular
            [7., 8., 9.]
        ];

        let result = u.inverse();
        
        // Assert that we got an Error
        assert!(result.is_err());
        // Optional: Check specific error variant if accessible
        // assert_eq!(result.unwrap_err(), Error::SingularMatrix);
    }

    #[test]
    fn test_inverse_property_verification() {
        // Fundamental Property: A * A_inverse = Identity
        let mut a = matrix![
            [4., 7.],
            [2., 6.]
        ];

        let a_inv = a.inverse().expect("Matrix should be invertible");

        let identity = &a * &a_inv;
        let expected_identity = matrix![
            [1., 0.],
            [0., 1.]
        ];

        assert_matrix_approx_eq(&identity, &expected_identity);
    }

    #[test]
    fn test_inverse_2x2_simple() {
        // [1, 2] -> Det = 4 - 6 = -2
        // [3, 4] -> Inv = (1/-2) * [4, -2]
        //                          [-3, 1]
        //               = [-2, 1]
        //                 [1.5, -0.5]
        let mut u = matrix![
            [1., 2.],
            [3., 4.]
        ];

        let result = u.inverse().expect("Invertible");
        let expected = matrix![
            [-2.0, 1.0],
            [1.5, -0.5]
        ];

        assert_matrix_approx_eq(&result, &expected);
    }
}
