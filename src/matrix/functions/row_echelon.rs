use std::ops::{Add, Div, Mul, Neg};

use crate::Matrix;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum RowEchelonOperation<K> {
    // usize: row index
    // K: scalar

    // row_a = row_b & row_b = row_a
    Swap(usize, usize),

    // row = row * k
    Multipication(usize, K),

    // row = row / k
    Division(usize, K),

    // row_a = row_a + row_b * k
    RowAddition(usize, usize, K),
}

impl<K: Copy> Matrix<K> {
    pub(crate) fn swap(&mut self, row_a: usize, row_b: usize) -> RowEchelonOperation<K> {
        let mut temp;

        for i in 0..self.cols() {
            temp = self[i][row_a];
            self[i][row_a] = self[i][row_b];
            self[i][row_b] = temp;
        }

        RowEchelonOperation::Swap(row_a, row_b)
    }

    pub(crate) fn multiply(&mut self, row: usize, scalar: K) -> RowEchelonOperation<K>
    where
        K: Mul<Output = K>,
    {
        for col in 0..self.cols() {
            self[col][row] = self[col][row] * scalar;
        }

        RowEchelonOperation::Multipication(row, scalar)
    }

    pub(crate) fn divide(&mut self, row: usize, scalar: K) -> RowEchelonOperation<K>
    where
        K: Div<Output = K>,
    {
        for col in 0..self.cols() {
            self[col][row] = self[col][row] / scalar;
        }

        RowEchelonOperation::Division(row, scalar)
    }

    pub(crate) fn row_add(
        &mut self,
        row_to_modify: usize,
        row_to_add: usize,
        scalar: K,
    ) -> RowEchelonOperation<K>
    where
        K: Mul<Output = K> + Add<Output = K>,
    {
        for col in 0..self.cols() {
            self[col][row_to_modify] = self[col][row_to_modify] + self[col][row_to_add] * scalar;
        }

        RowEchelonOperation::RowAddition(row_to_modify, row_to_add, scalar)
    }

    pub(crate) fn apply(&mut self, op: RowEchelonOperation<K>)
    where
        K: Mul<Output = K> + Div<Output = K> + Add<Output = K>,
    {
        match op {
            RowEchelonOperation::Swap(row_a, row_b) => self.swap(row_a, row_b),
            RowEchelonOperation::Multipication(row, scalar) => self.multiply(row, scalar),
            RowEchelonOperation::Division(row, scalar) => self.divide(row, scalar),
            RowEchelonOperation::RowAddition(row_to_modify, row_to_add, scalar) => {
                self.row_add(row_to_modify, row_to_add, scalar)
            }
        };
    }

    pub(crate) fn apply_multiple(&mut self, ops: Vec<RowEchelonOperation<K>>)
    where
        K: Mul<Output = K> + Div<Output = K> + Add<Output = K>,
    {
        ops.into_iter().for_each(|op| self.apply(op));
    }
}

#[derive(Debug, Default)]
pub struct RowEchelonDetails<K> {
    pub tracked_pivots: Vec<K>,
    pub operations: Vec<RowEchelonOperation<K>>,
}

impl<K> Matrix<K>
where
    K: Copy + PartialOrd + Default + Neg<Output = K>,
    K: Div<Output = K> + Mul<Output = K> + Add<Output = K>,
{
    /// Converts the matrix to her reduced row echelon form
    pub fn row_echelon(&self) -> Matrix<K> {
        self.reduced_row_echelon_form(None)
    }

    pub fn row_echelon_with_details(&self) -> (Matrix<K>, RowEchelonDetails<K>) {
        let mut details = RowEchelonDetails::default();
        let matrix = self.reduced_row_echelon_form(Some(&mut details));
        (matrix, details)
    }

    /// Converts the matrix to her reduced row echelon form while tracking pivot values and row swaps
    fn reduced_row_echelon_form(&self, mut details: Option<&mut RowEchelonDetails<K>>) -> Matrix<K> {
        macro_rules! details {
            ($($arg:tt)*) => {
                if let Some(details) = &mut details {
                    details.$($arg)*
                }
            };
        }

        if self.is_empty() {
            return self.clone();
        }

        let mut matrix = self.clone();
        let max_iterations = matrix.rows().min(matrix.cols());

        for row_index in 0..max_iterations {
            let next_pivot = matrix.next_pivot(row_index);
            if next_pivot.is_none() {
                break;
            }

            let (pivot_col, mut pivot_row) = next_pivot.unwrap();

            if pivot_row != row_index {
                let op = matrix.swap(pivot_row, row_index);
                details!(operations.push(op));
                pivot_row = row_index;
            }

            // track the pivot if needed (useful for 'Matrix::determinant()')
            details!(tracked_pivots.push(matrix[pivot_col][pivot_row]));

            // using elementary row operations, we transform the pivot to 1
            let op = matrix.scale_pivot_row(pivot_col, pivot_row);
            details!(operations.push(op));

            if pivot_row < matrix.rows() - 1 {
                // using elementary row operations, we put a 0 in values below the pivot
                let ops = matrix.nullify_rows_below_pivot(pivot_col, pivot_row);
                details!(operations.extend(ops));
            }
        }

        matrix
    }

    /// Calculates the rank of the matrix (true dimension of the matrix / number of linearly independent rows)
    pub fn rank(&self) -> usize {
        self.row_echelon()
            .as_rows()
            .position(|row| row.into_iter().all(|value| *value == K::default()))
            .unwrap_or(self.rows())
    }

    /// Checks if the matrix is in row echelon form
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

    /// Uses elementary row operations to put zeros below the pivot element
    #[doc(hidden)]
    fn nullify_rows_below_pivot(
        &mut self,
        pivot_col: usize,
        pivot_row: usize,
    ) -> Vec<RowEchelonOperation<K>> {
        // we assume that pivot == 1

        let mut operations = vec![];

        for row in pivot_row + 1..self.rows() {
            let factor = self[pivot_col][row];
            if factor == K::default() {
                continue;
            }

            operations.push(self.row_add(row, pivot_row, -factor));
        }

        operations
    }

    /// Uses elementary row operations to make the pivot equals to 1
    #[doc(hidden)]
    fn scale_pivot_row(&mut self, pivot_col: usize, pivot_row: usize) -> RowEchelonOperation<K> {
        let pivot = self[pivot_col][pivot_row];

        assert!(pivot != K::default());

        self.divide(pivot_row, pivot)
    }

    #[doc(hidden)]
    fn next_pivot(&self, min_row_index: usize) -> Option<(usize, usize)> {
        let zero = K::default();

        for col in 0..self.cols() {
            let mut saved_pivot: Option<(usize, usize)> = None;
            for row in min_row_index..self.rows() {
                let current = self[col][row];
                if self[col][row] == zero {
                    continue;
                }

                /*  Partial Pivoting
                    a technique used to ensure numerical stability:

                    Before you clear a column, you look at the current pivot position
                    and all the numbers below it in the same column. You find the number
                    with the largest absolute value and swap its row with your current row.
                */
                if let Some((pivot_col, pivot_row)) = saved_pivot {
                    let pivot = self[pivot_col][pivot_row];
                    if current > pivot {
                        saved_pivot = Some((col, row))
                    }
                } else {
                    saved_pivot = Some((col, row))
                }
            }

            if let Some(pivot) = saved_pivot {
                return Some(pivot);
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
