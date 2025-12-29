use std::ops::{Mul, Sub};

use crate::Vector;

pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    K: Copy + Sub<Output = K> + Mul<Output = K>,
{
    assert_eq!(u.dimension(), 3);
    assert_eq!(v.dimension(), 3);

    let ux = u[0];
    let uy = u[1];
    let uz = u[2];

    let vx = v[0];
    let vy = v[1];
    let vz = v[2];

    Vector::new(vec![
        (uy * vz) - (uz * vy),
        (uz * vx) - (ux * vz),
        (ux * vy) - (uy * vx),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================
    // Subject Test Cases
    // ==========================================

    #[test]
    fn test_subject_cross_product_axes() {
        // Case 1: Z-axis cross X-axis should give Y-axis
        let u = Vector::from(vec![0., 0., 1.]);
        let v = Vector::from(vec![1., 0., 0.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.scalars, vec![0., 1., 0.]);
    }

    #[test]
    fn test_subject_cross_product_standard() {
        // Case 2: [1, 2, 3] x [4, 5, 6]
        let u = Vector::from(vec![1., 2., 3.]);
        let v = Vector::from(vec![4., 5., 6.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.scalars, vec![-3., 6., -3.]);
    }

    #[test]
    fn test_subject_cross_product_complex() {
        // Case 3: [4, 2, -3] x [-2, -5, 16]
        let u = Vector::from(vec![4., 2., -3.]);
        let v = Vector::from(vec![-2., -5., 16.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.scalars, vec![17., -58., -16.]);
    }

    // ==========================================
    // Additional Unit Tests
    // ==========================================

    #[test]
    fn test_cross_product_zero_vector() {
        // Crossing any vector with a zero vector must result in a zero vector
        let u = Vector::from(vec![1.0, 2.0, 3.0]);
        let z = Vector::from(vec![0.0, 0.0, 0.0]);
        let res = cross_product(&u, &z);

        assert_eq!(res.scalars, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_cross_product_self() {
        // Crossing a vector with itself is always zero (angle is 0 degrees)
        let u = Vector::from(vec![42.0, -12.0, 7.0]);
        let res = cross_product(&u, &u);

        assert_eq!(res.scalars, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_cross_product_anticommutativity() {
        // u x v == -(v x u)
        let u = Vector::from(vec![1.0, 2.0, 3.0]);
        let v = Vector::from(vec![4.0, 5.0, 6.0]);

        let uv = cross_product(&u, &v);
        let vu = cross_product(&v, &u);

        assert_eq!(uv.scalars[0], -vu.scalars[0]);
        assert_eq!(uv.scalars[1], -vu.scalars[1]);
        assert_eq!(uv.scalars[2], -vu.scalars[2]);
    }

    #[test]
    fn test_cross_product_integers() {
        // Verifying it works with integer types
        let u = Vector::from(vec![1, 0, 0]);
        let v = Vector::from(vec![0, 1, 0]);
        let res = cross_product(&u, &v);

        assert_eq!(res.scalars, vec![0, 0, 1]);
    }

    #[test]
    #[should_panic]
    fn test_cross_product_invalid_dim() {
        // Should panic if vectors are not 3D
        let u = Vector::from(vec![1.0, 2.0]);
        let v = Vector::from(vec![3.0, 4.0]);
        let _ = cross_product(&u, &v);
    }
}
