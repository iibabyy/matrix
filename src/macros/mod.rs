pub(crate) mod arithmetics;
pub(crate) use arithmetics::*;

#[macro_export]
macro_rules! vector {
    ($elem:expr; $n:expr) => {
        $crate::vector::Vector::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        $crate::vector::Vector::from([$($x),+])
    };
}

#[macro_export]
macro_rules! matrix {
    ($elem:expr; $n:expr) => {
        $crate::matrix::Matrix::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        $crate::matrix::Matrix::from([$($crate::vector::Vector::from($x)),+])
    };
}
