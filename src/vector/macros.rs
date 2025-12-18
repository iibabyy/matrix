#![allow(unused)]

macro_rules! vector {
    () => (
        $crate::vector::Vector::new()
    );
    ($elem:expr; $n:expr) => (
        $crate::vector::Vector::from_elem($elem, $n)
    );
    ($($x:expr),+ $(,)?) => (
        $crate::vector::Vector::from([$($x),+])
    );
}

pub(crate) use vector;