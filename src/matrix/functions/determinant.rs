use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{Matrix, matrix::functions::row_echelon::RowEchelonOperation};

impl<K: Copy> Matrix<K>
where
    K: Mul<Output = K> + Sub<Output = K> + Add<Output = K> + Div<Output = K> + Neg<Output = K>,
    K: PartialOrd + Default,
{
    /// Calculates the determinant of the matrix (only for square matrices up to 4x4)
    pub fn determinant(&self) -> K {
        assert!(self.is_square());
        assert!(self.cols() <= 4);

        match self.cols() {
            0 => K::default(),

            1 => self[0][0],

            // ∣A∣ = ad - bc
            2 => self[0][0] * self[1][1] - self[1][0] * self[0][1],

            3 => {
                let (a, b, c) = (self[0][0], self[1][0], self[2][0]);
                let (d, e, f) = (self[0][1], self[1][1], self[2][1]);
                let (g, h, i) = (self[0][2], self[1][2], self[2][2]);

                // ∣A∣ = (aei + bfg + cdh) − (gec + hfa + idb)
                (a * e * i + b * f * g + c * d * h) - (g * e * c + h * f * a + i * d * b)
            }

            4 => self.determinant_for_dimension_4_and_more(),

            // Since we checked that self.cols() <= 4 before, the program should not go here
            _ => panic!("Program should not come go here"),
        }
    }

    fn determinant_for_dimension_4_and_more(&self) -> K {
        assert!(self.is_square());
        assert!(self.cols() >= 4);

        // ref: Row Echelon Form
        let (ref_matrix, details) = self.row_echelon_with_details();

        let ref_has_non_zero_row = ref_matrix
            .as_rows()
            .last()
            .is_some_and(|r| r.into_iter().all(|k| *k == K::default()));

        if ref_has_non_zero_row || details.tracked_pivots.is_empty() {
            return K::default();
        }

        let mut tracked_pivots = details.tracked_pivots.into_iter();
        let mut result = tracked_pivots.next().unwrap();
        for pivot in tracked_pivots {
            result = result * pivot;
        }

        let swap_count = details
            .operations
            .iter()
            .filter(|op| matches!(op, RowEchelonOperation::Swap(_, _)))
            .count();

        if swap_count % 2 != 0 {
            result = -result;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    // Helper to compare floats
    fn assert_approx_eq(a: f64, b: f64) {
        let epsilon = 1e-9;
        assert!((a - b).abs() < epsilon, "Expected {}, got {}", b, a);
    }

    // ==========================================
    // Subject Test Cases
    // ==========================================

    #[test]
    fn test_subject_case_1() {
        let u = matrix!([1., -1.], [-1., 1.],);

        assert_eq!(u.determinant(), 0.0);
    }

    #[test]
    fn test_subject_case_2() {
        let u = matrix!([2., 0., 0.], [0., 2., 0.], [0., 0., 2.],);

        assert_eq!(u.determinant(), 8.0);
    }

    #[test]
    fn test_subject_case_3() {
        let u = matrix!([8., 5., -2.], [4., 7., 20.], [7., 6., 1.],);

        assert_eq!(u.determinant(), -174.0);
    }

    #[test]
    fn test_subject_case_4() {
        let u = matrix!(
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        );

        assert_eq!(u.determinant(), 1032.0);
    }

    // ==========================================
    // Other Test Cases
    // ==========================================

    #[test]
    fn test_1x1() {
        let m = matrix!([10.0]);
        assert_approx_eq(m.determinant(), 10.0);
    }

    #[test]
    fn test_2x2() {
        // | 1 2 |
        // | 3 4 |
        // Det = 1*4 - 2*3 = 4 - 6 = -2
        let m = matrix!([1.0, 2.0], [3.0, 4.0]);
        assert_approx_eq(m.determinant(), -2.0);
    }

    #[test]
    fn test_3x3() {
        // | 6  1  1 |
        // | 4 -2  5 |
        // | 2  8  7 |
        // Rule of Sarrus calculation:
        // (6*-2*7) + (1*5*2) + (1*4*8) - (2*-2*1) - (8*5*6) - (7*4*1)
        // -84 + 10 + 32 - (-4) - 240 - 28
        // -42 - (-4) - 240 - 28 = -306
        let m = matrix!([6.0, 1.0, 1.0], [4.0, -2.0, 5.0], [2.0, 8.0, 7.0]);
        assert_approx_eq(m.determinant(), -306.0);
    }

    #[test]
    fn test_4x4_identity() {
        // Identity matrix determinant is always 1
        let m = matrix!(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        );
        assert_approx_eq(m.determinant(), 1.0);
    }

    #[test]
    fn test_4x4_singular() {
        // Two identical rows (Row 0 and Row 1) -> Det must be 0
        let m = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0], // Duplicate
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 0.0, 1.0, 2.0],
        );
        assert_approx_eq(m.determinant(), 0.0);
    }

    #[test]
    fn test_4x4_swap_required() {
        // | 0 1 0 0 |
        // | 1 0 0 0 |
        // | 0 0 1 0 |
        // | 0 0 0 1 |
        // One row swap needed (R1 <-> R2). Identity det is 1. Swap makes it -1.
        let m = matrix!(
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        );
        assert_approx_eq(m.determinant(), -1.0);
    }

    #[test]
    fn test_4x4_triangular() {
        // | 2 5 9 1 |
        // | 0 3 8 2 |
        // | 0 0 4 3 |
        // | 0 0 0 5 |
        // Det = Product of diagonal = 2 * 3 * 4 * 5 = 120
        let m = matrix!(
            [2.0, 5.0, 9.0, 1.0],
            [0.0, 3.0, 8.0, 2.0],
            [0.0, 0.0, 4.0, 3.0],
            [0.0, 0.0, 0.0, 5.0],
        );
        assert_approx_eq(m.determinant(), 120.0);
    }
}
