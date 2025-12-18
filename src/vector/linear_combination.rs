use crate::vector::Vector;

/// sum of multiple vectors, each multiplied by a scalar weight (e.g., av+bw)
pub fn linear_combination<K>(vectors: &[Vector<K>], coefs: &[K]) -> Vector<K> {
	assert_eq!(vectors.len(), coefs.len());

	

	todo!()
}