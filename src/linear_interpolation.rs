#![allow(unused)]

use std::ops::{Add, Mul, Sub};

pub fn lerp_generic<V, K>(u: V, v: V, t: K) -> V
where
    V: PartialOrd + Mul<K, Output = V> + Add<V, Output = V>,
    for<'a> V: Sub<&'a V, Output = V>,
    K: std::cmp::PartialOrd<f32>
{
    assert!(t >= 0.);
    assert!(t <= 1.);

    let diff = v - &u;
    u + (diff * t)
}

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: PartialOrd + Mul<f32, Output = V> + Add<V, Output = V>,
    for<'a> V: Sub<&'a V, Output = V>,
{
    lerp_generic(u, v, t)
}

mod tests {
    use super::*;
    use crate::macros::{matrix, vector};

    // -------------------------------------------------------------------------
    // SUBJECT TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_subject_case_1() {
        // lerp(0., 1., 0.) -> 0.0
        assert_eq!(0.0, lerp(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_subject_case_2() {
        // lerp(0., 1., 1.) -> 1.0
        assert_eq!(1.0, lerp(0.0, 1.0, 1.0));
    }

    #[test]
    fn test_subject_case_3() {
        // lerp(0., 1., 0.5) -> 0.5
        assert_eq!(0.5, lerp(0.0, 1.0, 0.5));
    }

    #[test]
    fn test_subject_case_4() {
        // lerp(21., 42., 0.3) -> 27.3
        assert_eq!(27.3, lerp(21.0, 42.0, 0.3));
    }

    #[test]
    fn test_subject_case_5() {
        // lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3)
        // Expected: [2.6, 1.3]
        let v1 = vector![2., 1.];
        let v2 = vector![4., 2.];
        let result = lerp(v1, v2, 0.3);

        assert_eq!(vector!(2.6, 1.3), result);
        // 2.0 + (4.0 - 2.0) * 0.3 = 2.6
        assert_eq!(2.6, result.scalars[0]);
        // 1.0 + (2.0 - 1.0) * 0.3 = 1.3
        assert_eq!(1.3, result.scalars[1]);
    }

    #[test]
    fn test_subject_case_6() {
        // lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20., 10.], [30., 40.]]), 0.5)
        // Expected: [[11., 5.5], [16.5, 22.]]
        let m1 = matrix![[2. as f32, 1.], [3., 4.]];
        let m2 = matrix![[20., 10.], [30., 40.]];
        let result = lerp(m1, m2, 0.5);

        assert_eq!(result.vectors[0].scalars, vec![11.0, 5.5]);
        assert_eq!(result.vectors[1].scalars, vec![16.5, 22.0]);
    }

    // -------------------------------------------------------------------------
    // ADDITIONAL TESTS (Directional & Edge Cases)
    // -------------------------------------------------------------------------

    #[test]
    fn test_reverse_direction() {
        // Validates that it does not swap arguments.
        // Start at 42, go towards 21.
        // Formula: 42 + (21 - 42) * 0.3
        //        = 42 - 6.3
        //        = 35.7
        let res = lerp(42.0, 21.0, 0.3);
        let diff = (res - 35.7).abs();

        assert!(
            diff < 0.0001,
            "Expected 35.7 (directional), but got {}",
            res
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_t_below_zero() {
        lerp(0.0, 10.0, -0.1);
    }

    #[test]
    #[should_panic]
    fn test_panic_t_above_one() {
        lerp(0.0, 10.0, 1.1);
    }
}
