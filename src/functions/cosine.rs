use std::ops::{AddAssign, Mul};

use crate::Vector;

pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> f32
where
    K: Copy + AddAssign + Mul<Output = K> + Into<f32>,
{
    assert!(!u.is_empty());
    assert_eq!(u.size(), v.size());

    let dot_product = u.dot(v.clone()).into();

    let u_norm = u.norm();
    let v_norm = v.norm();

    assert!(u_norm > 0.);
    assert!(v_norm > 0.);

    dot_product / (u_norm * v_norm)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper for floating point comparison
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
    fn test_subject_cosine_identical() {
        let u = Vector::from(vec![1., 0.]);
        let v = Vector::from(vec![1., 0.]);
        assert_approx_eq(angle_cos(&u, &v), 1.0);
    }

    #[test]
    fn test_subject_cosine_orthogonal() {
        let u = Vector::from(vec![1., 0.]);
        let v = Vector::from(vec![0., 1.]);
        assert_approx_eq(angle_cos(&u, &v), 0.0);
    }

    #[test]
    fn test_subject_cosine_opposite() {
        let u = Vector::from(vec![-1., 1.]);
        let v = Vector::from(vec![1., -1.]);
        assert_approx_eq(angle_cos(&u, &v), -1.0);
    }

    #[test]
    fn test_subject_cosine_collinear() {
        let u = Vector::from(vec![2., 1.]);
        let v = Vector::from(vec![4., 2.]);
        assert_approx_eq(angle_cos(&u, &v), 1.0);
    }

    #[test]
    fn test_subject_cosine_3d() {
        let u = Vector::from(vec![1., 2., 3.]);
        let v = Vector::from(vec![4., 5., 6.]);
        assert_approx_eq(angle_cos(&u, &v), 0.974631846);
    }

    // ==========================================
    // Additional Unit Tests
    // ==========================================

    #[test]
    fn test_cosine_negative_scaling() {
        // One vector is a negative multiple of the other
        let u = Vector::from(vec![1.0, 2.0, 3.0]);
        let v = Vector::from(vec![-2.0, -4.0, -6.0]);
        assert_approx_eq(angle_cos(&u, &v), -1.0);
    }

    #[test]
    fn test_cosine_high_size() {
        // orthogonal 5D vectors
        let u = Vector::from(vec![1.0, 0.0, 0.0, 0.0, 0.0]);
        let v = Vector::from(vec![0.0, 0.0, 0.0, 0.0, 1.0]);
        assert_approx_eq(angle_cos(&u, &v), 0.0);
    }

    #[test]
    fn test_cosine_same_direction_different_lengths() {
        let u = Vector::from(vec![10.0, 10.0]);
        let v = Vector::from(vec![0.1, 0.1]);
        assert_approx_eq(angle_cos(&u, &v), 1.0);
    }

    #[test]
    #[should_panic]
    fn test_cosine_zero_vector() {
        let u = Vector::from(vec![0.0, 0.0]);
        let v = Vector::from(vec![1.0, 2.0]);
        angle_cos(&u, &v);
    }
}
