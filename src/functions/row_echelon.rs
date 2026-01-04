use std::ops::{Div, Mul, Sub};

use crate::Matrix;

impl<K> Matrix<K>
where
    K: Copy + PartialOrd + Default,
    K: Div<Output = K> + Mul<Output = K> + Sub<Output = K>,
{
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut matrix = self.clone();

        if self.is_empty() {
            return matrix;
        }

        let max_row = matrix.rows().min(matrix.cols());

        for row_index in 0..max_row {
            let next_pivot = matrix.next_pivot(row_index);
            if next_pivot.is_none() {
                continue;
            }

            let (pivot_col, mut pivot_row) = next_pivot.unwrap();
            if pivot_row != row_index {
                matrix.swap_rows(pivot_row, row_index);
                pivot_row = row_index;
            }

            // using elementary row operations, we transform the pivot to 1
            matrix.scale_pivot_row(pivot_col, pivot_row);

            if pivot_row < max_row {
                // using elementary row operations, we put a 0 in values below the pivot
                matrix.nullify_rows_below_pivot(pivot_col, pivot_row);
            }
        }

        matrix
    }

    pub fn is_row_echelon_form(&self) -> bool {
        if self.rows() == 0 {
            return true;
        }

        let mut row_iter = self.as_rows();

        // closure to get the first non-zero index in a row
        let get_first_non_zero_index = |row: Vec<&K>| -> Option<usize> {
            row.into_iter().position(|value| *value != K::default())
        };

        // initialize saved_non_zero_index using the first row
        let mut saved_non_zero_index =
            if let Some(index) = get_first_non_zero_index(row_iter.next().unwrap()) {
                index
            } else {
                return row_iter.all(|row| row.into_iter().all(|value| *value == K::default()));
            };

        while let Some(row) = row_iter.next() {
            let first_non_zero_index = if let Some(index) = get_first_non_zero_index(row) {
                index
            } else {
                return row_iter.all(|row| row.into_iter().all(|value| *value == K::default()));
            };

            if first_non_zero_index <= saved_non_zero_index {
                return false;
            }

            saved_non_zero_index = first_non_zero_index;
        }

        true
    }

    #[doc(hidden)]
    fn nullify_rows_below_pivot(&mut self, pivot_col: usize, pivot_row: usize) {
        // we assume that pivot == 1

        for row in pivot_row + 1..self.rows() {
            let factor = self[pivot_col][row];
            if factor == K::default() {
                continue;
            }

            for col in pivot_col..self.cols() {
                self[col][row] = self[col][row] - self[col][pivot_row] * factor;
            }
        }
    }

    /// Divide the pivot row by the pivot value to make the pivot equals to 1
    #[doc(hidden)]
    fn scale_pivot_row(&mut self, pivot_col: usize, pivot_row: usize) {
        let pivot = self[pivot_col][pivot_row];

        assert!(pivot != K::default());

        for col in pivot_col..self.cols() {
            self[col][pivot_row] = self[col][pivot_row] / pivot;
        }
    }

    #[doc(hidden)]
    fn next_pivot(&self, min_row_index: usize) -> Option<(usize, usize)> {
        let zero = K::default();

        for col in 0..self.cols() {
            for row in min_row_index..self.rows() {
                if self[col][row] != zero {
                    return Some((col, row));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    // ==========================================
    // Subject Test Cases
    // ==========================================

    #[test]
    fn test_subject_rref_identity() {
        let u = matrix!(vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.],);
        // Identity is already in RREF
        let res = u.row_echelon();
        assert!(res.is_row_echelon_form());
    }

    #[test]
    fn test_subject_rref_2x2_invertible() {
        // [1, 2]       [1, 0]
        // [3, 4]  -->  [0, 1]
        let u = matrix!(vec![1., 2.], vec![3., 4.],);

        let res = u.row_echelon();
        assert!(res.is_row_echelon_form());
    }

    #[test]
    fn test_subject_rref_2x2_singular() {
        // [1, 2]       [1, 2]
        // [2, 4]  -->  [0, 0]
        let u = matrix!(vec![1., 2.], vec![2., 4.],);

        let res = u.row_echelon();
        assert!(res.is_row_echelon_form());
    }

    #[test]
    fn test_subject_rref_complex() {
        let u = matrix!(
            vec![8., 5., -2., 4., 28.],
            vec![4., 2.5, 20., 4., -4.],
            vec![8., 5., 1., 4., 17.],
        );

        let res = u.row_echelon();
        assert!(res.is_row_echelon_form());
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
        assert!(res.is_row_echelon_form());
    }

    #[test]
    fn test_rref_tall_matrix() {
        // More rows than columns
        // [1, 2]
        // [3, 4]  -->  Identity on top, zero row bottom
        // [5, 6]
        let u = matrix!(vec![1., 2.], vec![3., 4.], vec![5., 6.],);

        let res = u.row_echelon();
        assert!(res.is_row_echelon_form());
    }

    #[test]
    fn test_rref_pivot_moving() {
        // Pivot shouldn't be in the first column for the second row
        // [0, 1, 2]
        // [0, 3, 6] -> [0, 1, 2] and zero row
        let u = matrix!(vec![0., 1., 2.], vec![0., 3., 6.],);

        let res = u.row_echelon();
        assert!(res.is_row_echelon_form());
    }

    mod is_row_echelon_form {
        use super::*;

        // --- Basic Validity Tests ---

        #[test]
        fn test_simple_valid_case() {
            // Matrix: | 1 1 |
            // Pivot at (0,0). Valid.
            let matrix = matrix!([1.], [1.]);
            assert_eq!(matrix.is_row_echelon_form(), true);
        }

        #[test]
        fn test_valid_case() {
            // Matrix (Identity/Upper Triangular):
            // 1 2 3
            // 0 1 2
            // 0 0 1
            let matrix = matrix!([1., 0., 0.], [2., 1., 0.], [3., 2., 1.]);
            assert_eq!(matrix.is_row_echelon_form(), true);
        }

        #[test]
        fn test_zero_matrix() {
            // A matrix of all zeros is always in REF.
            let matrix = matrix!([0., 0.], [0., 0.]);
            assert_eq!(matrix.is_row_echelon_form(), true);
        }

        #[test]
        fn test_identity_matrix() {
            // 1 0
            // 0 1
            let matrix = matrix!([1., 0.], [0., 1.]);
            assert_eq!(matrix.is_row_echelon_form(), true);
        }

        // --- Violation Tests (Should be False) ---

        #[test]
        fn test_invalid_case_bad_stairs() {
            // Matrix:
            // 1 2 3
            // 0 1 2
            // 0 1 1  <-- Pivot is directly below previous pivot (not strictly right)
            let matrix = matrix!([1., 0., 0.], [2., 1., 1.], [3., 2., 1.]);
            assert_eq!(matrix.is_row_echelon_form(), false);
        }

        #[test]
        fn test_invalid_pivot_order() {
            // Matrix:
            // 0 1
            // 1 0
            // Row 0 starts with 0, Row 1 starts with 1.
            // The pivot of Row 1 is to the LEFT of Row 0. Invalid.
            let matrix = matrix!([0., 1.], [1., 0.]);
            assert_eq!(matrix.is_row_echelon_form(), false);
        }

        #[test]
        fn test_invalid_zero_row_gap() {
            // Matrix:
            // 1 1
            // 0 0  <-- Zero row
            // 0 1  <-- Non-zero row below a zero row
            // Zero rows must be at the bottom.
            let matrix = matrix!([1., 0., 0.], [1., 0., 1.]);
            assert_eq!(matrix.is_row_echelon_form(), false);
        }

        // --- Complex Shapes (Wide & Tall) ---

        #[test]
        fn test_wide_matrix_with_large_step() {
            // Matrix:
            // 1 2 3 4
            // 0 0 1 2
            // Valid: Pivot at (0,0), next Pivot at (1,2).
            let matrix = matrix!([1., 0.], [2., 0.], [3., 1.], [4., 2.]);
            assert_eq!(matrix.is_row_echelon_form(), true);
        }

        #[test]
        fn test_tall_matrix_valid() {
            // Matrix:
            // 1 2
            // 0 1
            // 0 0
            // Valid REF.
            let matrix = matrix!([1., 0., 0.], [2., 1., 0.]);
            assert_eq!(matrix.is_row_echelon_form(), true);
        }

        #[test]
        fn test_tall_matrix_invalid() {
            // Matrix:
            // 1 2
            // 0 0
            // 0 1
            // Invalid because zero row is not at the bottom.
            let matrix = matrix!([1., 0., 0.], [2., 0., 1.]);
            assert_eq!(matrix.is_row_echelon_form(), false);
        }
    }
}
