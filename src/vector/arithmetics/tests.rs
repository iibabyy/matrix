use crate::vector::macros::vector;

// -------------------------------------------------------------------------
// TEST: ADDITION
// -------------------------------------------------------------------------
mod addition {
    use super::*;

    #[test]
    fn test_owned_owned() {
        let v1 = vector![1, 2, 3];
        let v2 = vector![4, 5, 6];
        let result = v1 + v2;
        assert_eq!(result.scalars, vec![5, 7, 9]);
    }

    #[test]
    fn test_owned_ref() {
        let v1 = vector![10, 20];
        let v2 = vector![1, 2];
        let result = v1 + &v2;
        assert_eq!(result.scalars, vec![11, 22]);
    }

    #[test]
    fn test_ref_ref() {
        let v1 = vector![1, 1];
        let v2 = vector![2, 2];
        let result = &v1 + &v2;
        assert_eq!(result.scalars, vec![3, 3]);
        // Ensure originals are still valid
        assert_eq!(v1.scalars, vec![1, 1]);
        assert_eq!(v2.scalars, vec![2, 2]);
    }

    #[test]
    fn test_assign_owned() {
        let mut v1 = vector![0, 10];
        v1 += vector![1, 1];
        assert_eq!(v1.scalars, vec![1, 11]);
    }

    #[test]
    fn test_assign_ref() {
        let mut v1 = vector![5, 5];
        let v2 = vector![1, 2];
        v1 += &v2;
        assert_eq!(v1.scalars, vec![6, 7]);
    }

    #[test]
    #[should_panic]
    fn test_panic_dim_mismatch() {
        let v1 = vector![1, 2];
        let v2 = vector![1, 2, 3];
        let _ = v1 + v2; // Should panic
    }
}

// -------------------------------------------------------------------------
// TEST: SUBTRACTION
// -------------------------------------------------------------------------
mod subtraction {
    use super::*;

    #[test]
    fn test_owned_owned() {
        let v1 = vector![10, 20];
        let v2 = vector![1, 2];
        let result = v1 - v2;
        assert_eq!(result.scalars, vec![9, 18]);
    }

    #[test]
    fn test_ref_ref() {
        let v1 = vector![5, 5, 5];
        let v2 = vector![1, 1, 1];
        let result = &v1 - &v2;
        assert_eq!(result.scalars, vec![4, 4, 4]);
    }

    #[test]
    fn test_assign() {
        let mut v1 = vector![10, 10];
        let v2 = vector![3, 4];
        v1 -= v2;
        assert_eq!(v1.scalars, vec![7, 6]);
    }

    #[test]
    fn test_negative_result() {
        let v1 = vector![0, 0];
        let v2 = vector![1, 1];
        let result = v1 - v2;
        assert_eq!(result.scalars, vec![-1, -1]);
    }

    #[test]
    #[should_panic]
    fn test_panic_dim_mismatch() {
        let v1 = vector![1];
        let v2 = vector![1, 2];
        let _ = &v1 - &v2; // Should panic
    }
}

// -------------------------------------------------------------------------
// TEST: SCALAR MULTIPLICATION (SCL)
// -------------------------------------------------------------------------
mod multiplication {
    use super::*;

    #[test]
    fn test_owned_scalar() {
        let v1 = vector![1, -2, 3];
        let result = v1 * 2;
        assert_eq!(result.scalars, vec![2, -4, 6]);
    }

    #[test]
    fn test_ref_scalar() {
        let v1 = vector![10, 20];
        let result = &v1 * 3;
        assert_eq!(result.scalars, vec![30, 60]);
        // Ensure original is preserved
        assert_eq!(v1.scalars, vec![10, 20]);
    }

    #[test]
    fn test_assign() {
        let mut v1 = vector![2, 4];
        v1 *= 2;
        assert_eq!(v1.scalars, vec![4, 8]);
    }

    #[test]
    fn test_zero() {
        let v1 = vector![1, 2, 3];
        let result = v1 * 0;
        assert_eq!(result.scalars, vec![0, 0, 0]);
    }
}
