use std::ops::{Add, Mul, Neg};

use crate::vector::Vector;

/// sum of multiple vectors, each multiplied by a coefficient (e.g., av+bw)
pub fn linear_combination<K>(vectors: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Copy + Neg + Mul<Output = K> + Add<Output = K>,
{
    assert!(!vectors.is_empty());
    assert_eq!(vectors.len(), coefs.len());

    let mut vector = &vectors[0] * coefs[0];

    for i in 1..vectors.len() {
        vector += &vectors[i] * coefs[i];
    }

    vector
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create vectors easily for testing
    fn v(vals: Vec<i32>) -> Vector<i32> {
        Vector::from(vals)
    }

    #[test]
    fn test_simple() {
        // Test av + bw where a=2, b=3, v=[1, 0], w=[0, 1]
        let v1 = v(vec![1, 0]);
        let v2 = v(vec![0, 1]);
        let vectors = vec![v1, v2];
        let coefficients = vec![2, 3];

        let result = linear_combination(&vectors, &coefficients);
        assert_eq!(result.scalars, vec![2, 3]);
    }

    #[test]
    fn test_three_vectors() {
        // Test av + bw + cu where a=1, b=2, c=3, v=[1, 0, 0], w=[0, 1, 0], u=[0, 0, 1]
        let v1 = v(vec![1, 0, 0]);
        let v2 = v(vec![0, 1, 0]);
        let v3 = v(vec![0, 0, 1]);
        let vectors = vec![v1, v2, v3];
        let coefficients = vec![1, 2, 3];

        let result = linear_combination(&vectors, &coefficients);
        assert_eq!(result.scalars, vec![1, 2, 3]);
    }

    #[test]
    fn test_with_negatives() {
        // Test av + bw where a=-1, b=2, v=[1, 2], w=[3, 4]
        let v1 = v(vec![1, 2]);
        let v2 = v(vec![3, 4]);
        let vectors = vec![v1, v2];
        let coefficients = vec![-1, 2];

        let result = linear_combination(&vectors, &coefficients);
        assert_eq!(result.scalars, vec![5, 6]); // -1*[1,2] + 2*[3,4] = [-1,-2] + [6,8] = [5,6]
    }

    #[test]
    fn test_with_zeros() {
        // Test av + bw where a=0, b=5, v=[1, 2], w=[3, 4]
        let v1 = v(vec![1, 2]);
        let v2 = v(vec![3, 4]);
        let vectors = vec![v1, v2];
        let coefficients = vec![0, 5];

        let result = linear_combination(&vectors, &coefficients);
        assert_eq!(result.scalars, vec![15, 20]); // 0*[1,2] + 5*[3,4] = [0,0] + [15,20] = [15,20]
    }

    #[test]
    fn test_single_vector() {
        // Test av where a=5, v=[2, 3, 4]
        let v1 = v(vec![2, 3, 4]);
        let vectors = vec![v1];
        let coefficients = vec![5];

        let result = linear_combination(&vectors, &coefficients);
        assert_eq!(result.scalars, vec![10, 15, 20]);
    }

    #[test]
    #[should_panic(expected = "assertion failed: !vectors.is_empty()")]
    fn test_empty_vectors_should_panic() {
        let vectors: Vec<Vector<i32>> = vec![];
        let coefficients: Vec<i32> = vec![];

        let _ = linear_combination(&vectors, &coefficients);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed\n  left: 2\n right: 1")]
    fn test_mismatched_lengths_should_panic() {
        let v1 = v(vec![1, 2]);
        let v2 = v(vec![3, 4]);
        let vectors = vec![v1, v2];
        let coefficients = vec![1]; // Mismatched length

        let _ = linear_combination(&vectors, &coefficients);
    }

    #[test]
    fn test_floating_point() {
        // Test with floating point values
        let v1 = Vector::from(vec![1.0, 0.0]);
        let v2 = Vector::from(vec![0.0, 1.0]);
        let vectors = vec![v1, v2];
        let coefficients = vec![2.5, -1.5];

        let result = linear_combination(&vectors, &coefficients);
        assert_eq!(result.scalars, vec![2.5, -1.5]);
    }
}
