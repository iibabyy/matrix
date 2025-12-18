#![allow(dead_code)]

mod macros;

#[derive(Debug)]
pub struct Vector<T = f32> {
    scalars: Vec<T>,
}

impl<T: std::fmt::Display> std::fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.scalars.iter().try_for_each(|c| writeln!(f, "[{c}]"))
    }
}

/* Utils functions */
impl<T> Vector<T> {
    pub fn new() -> Self {
        Self { scalars: vec![] }
    }

    fn from_elem(elem: T, n: usize) -> Self
    where
        T: Clone,
    {
        Self {
            scalars: vec![elem; n],
        }
    }

    pub const fn len(&self) -> usize {
        self.scalars.len()
    }
}

/* Exercise 00 */
impl<T> Vector<T> {
    fn add<K>(&mut self, other: &Vector<K>)
    where
        T: std::ops::AddAssign<K>,
        K: Copy,
    {
        assert_eq!(self.len(), other.len());

        for i in 0..self.scalars.len() {
            self.scalars[i] += other.scalars[i];
        }
    }

    fn sub<K>(&mut self, other: &Vector<K>)
    where
        T: std::ops::SubAssign<K>,
        K: Copy,
    {
        assert_eq!(self.len(), other.len());

        for i in 0..self.scalars.len() {
            self.scalars[i] -= other.scalars[i];
        }
    }

    fn scl<K>(&mut self, scale: K)
    where
        T: std::ops::MulAssign<K>,
        K: Copy,
    {
        for i in 0..self.scalars.len() {
            self.scalars[i] *= scale;
        }
    }
}

/* Trait implementations */

impl<T> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            scalars: Vec::from_iter(iter),
        }
    }
}

impl<T, I> From<I> for Vector<T>
where
    I: IntoIterator<Item = T>,
{
    fn from(iter: I) -> Self {
        Self::from_iter(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::{Vector, macros::vector};

    mod add {
        use super::*;

        #[test]
        fn test_add_basic() {
            let mut v1 = vector!(1.0, 2.0);
            let v2 = vector![3.0, 4.0];
            v1.add(&v2);
            assert_eq!(v1.scalars, [4.0, 6.0]);
        }

        #[test]
        #[should_panic]
        fn test_add_mismatched_size() {
            let mut v1 = vector![1, 2];
            let v2 = vector![1, 2, 3];
            v1.add(&v2);
        }
    }

    mod sub {
        use super::*;

        #[test]
        fn test_sub_basic() {
            let mut v1 = vector![10, 20];
            let v2 = vector![2, 5];
            v1.sub(&v2);
            assert_eq!(v1.scalars, [8, 15]);
        }

        #[test]
        #[should_panic]
        fn test_sub_mismatched_size() {
            let mut v1 = vector![1];
            let v2: Vector<i32> = vector![];
            v1.sub(&v2);
        }
    }

    mod scl {
        use super::*;

        #[test]
        fn test_scl_basic() {
            let mut v = vector![1.0, -2.0, 3.5];
            v.scl(2.0);
            assert_eq!(v.scalars, [2.0, -4.0, 7.0]);
        }

        #[test]
        fn test_scl_zero() {
            let mut v = vector![10, 20];
            v.scl(0);
            assert_eq!(v.scalars, [0, 0]);
        }
    }
}
