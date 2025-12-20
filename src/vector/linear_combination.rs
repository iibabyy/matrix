// use std::ops::{Add, Mul, Neg};

// use crate::vector::Vector;

// /// sum of multiple vectors, each multiplied by a scalar weight (e.g., av+bw)
// pub fn linear_combination<K>(vectors: &[Vector<K>], coefs: &[K]) -> Vector<K>
// where
//     K: Copy + Neg + Mul<Output = K> + Add<Output = K>,
// {
// 	assert!(vectors.len() > 0);
//     assert_eq!(vectors.len(), coefs.len());

//     let mut vec = vectors[0] * coefs[0];

// 	for i in 1..vectors.len() {
// 		vec += vectors[i] * coefs[i];
// 	}

// 	todo!()
// }
