use std::ops::{Add, Div, Mul, Neg};

use crate::Matrix;

impl<K> Matrix<K>
where
    K: Copy + PartialOrd + Default + Neg<Output = K>,
    K: Div<Output = K> + Mul<Output = K> + Add<Output = K>,
{
    /// Calculates the rank of the matrix (true dimension of the matrix / number of linearly independent rows)
    pub fn rank(&self) -> usize {
        let (_, ref_details) = self.row_echelon_with_details();
        ref_details.tracked_pivots.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    // ==========================================
    // Subject Test Cases
    // ==========================================

    #[test]
    fn test_subject_rank_identity() {
        // Identity matrix is full rank (3x3 -> rank 3)
        let u = matrix![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
        assert_eq!(u.rank(), 3);
    }

    #[test]
    fn test_subject_rank_rectangular_dependent() {
        // [ 1., 2., 0., 0.]
        // [ 2., 4., 0., 0.]  <- Row 2 is Row 1 * 2 (Dependent)
        // [-1., 2., 1., 1.]
        // This should result in 2 pivots
        let u = matrix![[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]];
        assert_eq!(u.rank(), 2);
    }

    #[test]
    fn test_subject_rank_tall_matrix() {
        // 4x3 matrix. Max possible rank is 3.
        let u = matrix![[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]];
        assert_eq!(u.rank(), 3);
    }

    // ==========================================
    // Additional Unit Tests
    // ==========================================

    #[test]
    fn test_rank_zero_matrix() {
        // A matrix of all zeros has rank 0
        let u = matrix![[0., 0.], [0., 0.], [0., 0.]];
        assert_eq!(u.rank(), 0);
    }

    #[test]
    fn test_rank_single_element() {
        let u = matrix![[42.]];
        assert_eq!(u.rank(), 1);
    }

    #[test]
    fn test_rank_linear_dependency() {
        // Row 3 is exactly Row 1 + Row 2
        // [1, 1, 1]
        // [2, 2, 2]
        // [3, 3, 3]
        // All rows are multiples of [1, 1, 1], so rank is 1
        let u = matrix![[1., 1., 1.], [2., 2., 2.], [3., 3., 3.]];
        assert_eq!(u.rank(), 1);
    }

    #[test]
    fn test_rank_wide_matrix() {
        // 2x4 matrix. Max possible rank is 2.
        let u = matrix![[1., 0., 5., 9.], [0., 1., 2., 3.]];
        assert_eq!(u.rank(), 2);
    }
}
