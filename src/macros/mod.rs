pub(crate) mod arithmetics;
pub(crate) use arithmetics::*;

#[macro_export]
macro_rules! vector {
    ($elem:expr; $n:expr) => {
        $crate::vector::Vector::new(vec![$elem; $n])
    };
    ($($x:expr),+ $(,)?) => {
        $crate::vector::Vector::from([$($x),+])
    };
}

#[macro_export]
macro_rules! matrix {
    () => {
        $crate::matrix::Matrix::new(vec![])
    };
    ($elem:expr; $n:expr) => {
        $crate::matrix::Matrix::new(vec![$elem; $n])
    };
    ($($x:expr),+ $(,)?) => {
        $crate::matrix::Matrix::from([$($crate::vector::Vector::from($x)),+])
    };
}
