#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! vector {
    ($elem:expr; $n:expr) => {
        $crate::vector::Vector::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        $crate::vector::Vector::from([$($x),+])
    };
}

macro_rules! matrix {
    ($elem:expr; $n:expr) => {
        $crate::matrix::Matrix::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        $crate::matrix::Matrix::from([$($x),+])
    };
}

pub(crate) use matrix;
pub(crate) use vector;
