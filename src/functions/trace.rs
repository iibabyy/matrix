use std::ops::Add;

use crate::Matrix;

impl<K> Matrix<K>
where
    K: Copy + Add<Output = K> + Default,
{
    /// Calculates the trace of the matrix (sum of diagonal elements)
    pub fn trace(&self) -> K {
        if self.is_empty() {
            return K::default();
        }

        assert_eq!(self.cols(), self[0].size());

        let mut sum = self[0][0];

        for i in 1..self.cols() {
            sum = sum + self[i][i];
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    // ==========================================
    // Subject Test Cases
    // ==========================================

    #[test]
    fn test_subject_trace_identity() {
        // Identity matrix 2x2: 1 + 1 = 2
        let u = matrix![[1., 0.], [0., 1.]];
        assert_eq!(u.trace(), 2.0);
    }

    #[test]
    fn test_subject_trace_3x3_positive() {
        // 3x3 Matrix: 2 + 3 + 4 = 9
        let u = matrix![[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]];
        assert_eq!(u.trace(), 9.0);
    }

    #[test]
    fn test_subject_trace_3x3_negative() {
        // 3x3 Matrix: -2 + -23 + 4 = -21
        let u = matrix![[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]];
        assert_eq!(u.trace(), -21.0);
    }

    // ==========================================
    // Additional Unit Tests
    // ==========================================

    #[test]
    fn test_trace_1x1() {
        // Trace of a 1x1 matrix is just the single element
        let u = matrix![[42.5]];
        assert_eq!(u.trace(), 42.5);
    }

    #[test]
    fn test_trace_integers() {
        // Ensure it works with integer types
        let u = matrix![[10, 2], [3, 5]];
        assert_eq!(u.trace(), 15);
    }

    #[test]
    fn test_trace_zero_matrix() {
        let u = matrix![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]];
        assert_eq!(u.trace(), 0.0);
    }

    #[test]
    fn test_trace_empty_matrix() {
        use crate::Matrix;
        let u: Matrix = matrix![];
        assert_eq!(u.trace(), 0.0);
    }

    #[test]
    #[should_panic]
    fn test_trace_non_square_panic() {
        // Attempting to calculate trace on a 2x3 matrix
        let u = matrix![[1., 2., 3.], [4., 5., 6.]];
        let _ = u.trace();
    }
}
