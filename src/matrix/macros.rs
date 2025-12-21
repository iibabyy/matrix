#![allow(unused)]

macro_rules! matrix {
    () => (
        $crate::matrix::Matrix::new()
    );
    ($elem:expr; $n:expr) => (
        $crate::matrix::Matrix::from_elem($elem, $n)
    );
    ($($x:expr),+ $(,)?) => (
        $crate::matrix::Matrix::from([$($x),+])
    );
}

pub(crate) use matrix;
