use std::ops::Neg;

use crate::vector::Vector;

impl<K> Vector<K>
where
    K: Copy + Neg + Into<f32>,
{
    pub fn norm_1(&self) -> f32 {
        self.scalars.iter().map(|&x| x.into().abs()).sum()
    }

    pub fn norm(&self) -> f32 {
        self.scalars
            .iter()
            .map(|&x| {
                let val: f32 = x.into();
                val * val
            })
            .sum::<f32>()
            .powf(0.5)
    }

    pub fn norm_inf(&self) -> f32 {
        self.scalars
            .iter()
            .map(|&x| x.into().abs())
            .fold(0.0, |max, x| max.max(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to compare floats with tolerance
    fn assert_approx_eq(a: f32, b: f32) {
        let epsilon = 1e-6;
        assert!(
            (a - b).abs() < epsilon,
            "Assertion failed: {} is not approximately equal to {}",
            a,
            b
        );
    }

    // ==========================================
    // Subject Test Cases
    // ==========================================

    #[test]
    fn test_subject_case_1() {
        let u = Vector::from(vec![0., 0., 0.]);

        assert_eq!(u.norm_1(), 0.0);
        assert_eq!(u.norm(), 0.0);
        assert_eq!(u.norm_inf(), 0.0);
    }

    #[test]
    fn test_subject_case_2() {
        let u = Vector::from(vec![1., 2., 3.]);

        assert_eq!(u.norm_1(), 6.0);
        assert_approx_eq(u.norm(), 3.74165738);
        assert_eq!(u.norm_inf(), 3.0);
    }

    #[test]
    fn test_subject_case_3() {
        let u = Vector::from(vec![-1., -2.]);

        assert_eq!(u.norm_1(), 3.0);
        assert_approx_eq(u.norm(), 2.236067977);
        assert_eq!(u.norm_inf(), 2.0);
    }

    // ==========================================
    // Additional Unit Tests
    // ==========================================

    #[test]
    fn test_single_element() {
        // Test a vector with a single negative component
        let u = Vector::from(vec![-5.0]);

        assert_eq!(u.norm_1(), 5.0);
        assert_eq!(u.norm(), 5.0); // sqrt(25) = 5
        assert_eq!(u.norm_inf(), 5.0);
    }

    #[test]
    fn test_empty_vector() {
        let u: Vector<f32> = Vector::from(vec![]);

        assert_eq!(u.norm_1(), 0.0);
        assert_eq!(u.norm(), 0.0);
        assert_eq!(u.norm_inf(), 0.0);
    }

    #[test]
    fn test_large_numbers() {
        // Test to ensure no overflow issues with reasonably large inputs
        let u = Vector::from(vec![1000.0, 0.0]);

        assert_eq!(u.norm_1(), 1000.0);
        assert_eq!(u.norm(), 1000.0);
        assert_eq!(u.norm_inf(), 1000.0);
    }
}
