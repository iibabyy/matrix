use std::ops::{AddAssign, Mul, Neg};

use crate::vector::Vector;

impl<K> Vector<K>
where
    K: Copy + Neg + AddAssign + Mul<Output = K>,
{
    pub fn dot(&self, mut v: Vector<K>) -> K {
        assert_eq!(self.len(), v.len());
        debug_assert!(!self.is_empty());

        v *= self;

        let mut res = v[0];

        for scalar in &v.scalars[1..] {
            res += *scalar;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector;

    // -------------------------------------------------------------------------
    // SUBJECT TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_subject_case_1() {
        // u=[0, 0], v=[1, 1] -> 0*1 + 0*1 = 0
        let u = vector![0., 0.];
        let v = vector![1., 1.];
        assert_eq!(0.0, u.dot(v));
    }

    #[test]
    fn test_subject_case_2() {
        // u=[1, 1], v=[1, 1] -> 1*1 + 1*1 = 2
        let u = vector![1., 1.];
        let v = vector![1., 1.];
        assert_eq!(2.0, u.dot(v));
    }

    #[test]
    fn test_subject_case_3() {
        // u=[-1, 6], v=[3, 2] -> (-1*3) + (6*2) = -3 + 12 = 9
        let u = vector![-1., 6.];
        let v = vector![3., 2.];
        assert_eq!(9.0, u.dot(v));
    }

    // -------------------------------------------------------------------------
    // LOGIC & ALGEBRAIC PROPERTIES
    // -------------------------------------------------------------------------

    #[test]
    fn test_orthogonality() {
        // Perpendicular vectors should result in 0
        let u = vector![1., 0.];
        let v = vector![0., 1.];
        assert_eq!(0.0, u.dot(v));
    }

    #[test]
    fn test_collinear_opposite() {
        // Vectors pointing in exact opposite directions
        let u = vector![2., 0.];
        let v = vector![-2., 0.];
        assert_eq!(-4.0, u.dot(v));
    }

    #[test]
    fn test_commutativity() {
        // u.dot(v) == v.dot(u)
        let u = vector![2., 3.];
        let v = vector![4., 5.];

        // Note: We clone because dot() consumes 'v'
        let res1 = u.dot(v.clone());
        let res2 = v.dot(u.clone());

        assert_eq!(res1, res2);
        assert_eq!(res1, 23.0); // 2*4 + 3*5 = 8 + 15 = 23
    }

    #[test]
    fn test_larger_dimension() {
        // 3D vector test
        let u = vector![1., 2., 3.];
        let v = vector![4., -5., 6.];
        // 1*4 + 2*(-5) + 3*6 = 4 - 10 + 18 = 12
        assert_eq!(12.0, u.dot(v));
    }

    #[test]
    fn test_identity_magnitude_squared() {
        // Dot product with self is magnitude squared
        let u = vector![3., 4.]; // Magnitude 5
        assert_eq!(25.0, u.dot(u.clone()));
    }

    // -------------------------------------------------------------------------
    // PANIC / ERROR HANDLING
    // -------------------------------------------------------------------------

    #[test]
    #[should_panic]
    fn test_panic_dim_mismatch() {
        let u = vector![1., 1.];
        let v = vector![1., 1., 1.];
        // Should panic due to assert_eq!(self.len(), v.len())
        u.dot(v);
    }

    #[test]
    #[should_panic]
    fn test_panic_empty() {
        // Empty vectors, so this must panic
        let u: Vector<f32> = Vector { scalars: vec![] };
        let v: Vector<f32> = Vector { scalars: vec![] };
        u.dot(v);
    }
}
