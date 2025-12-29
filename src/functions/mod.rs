mod dot_product;
mod norm;
mod trace;
mod transpose;
mod row_echelon;

mod linear_combination;
pub use linear_combination::linear_combination;

mod linear_interpolation;
pub use linear_interpolation::{lerp, lerp_generic};

mod cosine;
pub use cosine::angle_cos;

mod cross_product;
pub use cross_product::cross_product;
