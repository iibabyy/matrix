use super::*;

// Helper to easily create vectors for testing
fn v(inputs: Vec<f32>) -> Vector<f32> {
    Vector::from(inputs)
}

// Helper for integer vectors
fn v_i32(inputs: Vec<i32>) -> Vector<i32> {
    Vector::from(inputs)
}

// -------------------------------------------------------------------------
// TEST: ADDITION
// -------------------------------------------------------------------------

#[test]
fn test_add_trait() {
    // Test + operator (consumes self)
    let v1 = v_i32(vec![1, 2]);
    let v2 = v_i32(vec![3, 4]);

    // Equation: [1, 2] + [3, 4] = [4, 6]
    let result = v1 + &v2;
    assert_eq!(result.scalars, vec![4, 6]);
}

#[test]
fn test_add_assign_trait() {
    // Test += operator
    let mut v1 = v_i32(vec![10, 20]);
    let v2 = v_i32(vec![1, 2]);

    v1 += &v2;
    assert_eq!(v1.scalars, vec![11, 22]);
}

#[test]
fn test_add_explicit_fn() {
    // Test the private helper fn add()
    let mut v1 = v(vec![1.5, 2.5]);
    let v2 = v(vec![0.5, 0.5]);

    Vector::add(&mut v1, &v2);
    assert_eq!(v1.scalars, vec![2.0, 3.0]);
}

#[test]
#[should_panic]
fn test_add_panic_on_size_mismatch() {
    let v1 = v_i32(vec![1, 2]); // Size 2
    let v2 = v_i32(vec![1, 2, 3]); // Size 3

    // This should trigger the assert_eq! inside add_assign
    let _ = v1 + &v2;
}

// -------------------------------------------------------------------------
// TEST: SUBTRACTION
// -------------------------------------------------------------------------

#[test]
fn test_sub_trait() {
    // Test - operator
    let v1 = v_i32(vec![10, 20]);
    let v2 = v_i32(vec![1, 2]);

    // Equation: [10, 20] - [1, 2] = [9, 18]
    let result = v1 - &v2;
    assert_eq!(result.scalars, vec![9, 18]);
}

#[test]
fn test_sub_assign_trait() {
    // Test -= operator
    let mut v1 = v(vec![5.5, 6.5]);
    let v2 = v(vec![0.5, 1.5]);

    v1 -= &v2;
    assert_eq!(v1.scalars, vec![5.0, 5.0]);
}

#[test]
fn test_sub_explicit_fn() {
    // Test the private helper fn sub()
    let mut v1 = v_i32(vec![5, 5]);
    let v2 = v_i32(vec![5, 5]);

    Vector::sub(&mut v1, &v2);
    assert_eq!(v1.scalars, vec![0, 0]);
}

#[test]
#[should_panic]
fn test_sub_panic_on_size_mismatch() {
    let v1 = v_i32(vec![1]);
    let v2 = v_i32(vec![1, 2]);
    let _ = v1 - &v2;
}

// -------------------------------------------------------------------------
// TEST: SCALAR MULTIPLICATION (SCL)
// -------------------------------------------------------------------------

#[test]
fn test_mul_trait() {
    // Test * operator
    let v1 = v_i32(vec![1, -2, 3]);
    let scale = 3;

    // Equation: [1, -2, 3] * 3 = [3, -6, 9]
    let result = v1 * scale;
    assert_eq!(result.scalars, vec![3, -6, 9]);
}

#[test]
fn test_mul_assign_trait() {
    // Test *= operator
    let mut v1 = v(vec![2.0, 4.0]);

    v1 *= 0.5;
    assert_eq!(v1.scalars, vec![1.0, 2.0]);
}

#[test]
fn test_scl_explicit_fn() {
    // Test the private helper fn scl()
    let mut v1 = v_i32(vec![10, 20]);

    v1.scl(2);
    assert_eq!(v1.scalars, vec![20, 40]);
}
