mod norm;
mod dot_product;

mod linear_combination;
pub use linear_combination::linear_combination;

mod linear_interpolation;
pub use linear_interpolation::{lerp, lerp_generic};

mod cosine;
pub use cosine::angle_cos;
