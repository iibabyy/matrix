use std::ops::Neg;

use crate::vector::Vector;

#[expect(unused)]
impl<K> Vector<K>
where
    K: Copy + Neg + Into<f32>,
{
    fn norm_1(&self) -> f32
    {
        self.scalars
            .iter()
            .map(|&x| x.into().abs()) // Convert to float, then absolute value
            .sum()
    }

    fn norm(&self) -> f32
    {
        self.scalars
            .iter()
            .map(|&x| {
                let val: f32 = x.into();
                val * val // Multiplication is usually slightly faster/cleaner than powf(2.0)
            })
            .sum::<f32>()
            .sqrt()
    }

    fn norm_inf(&self) -> f32
    {
        self.scalars
            .iter()
            .map(|&x| x.into().abs())
            .fold(0.0, |max, x| max.max(x))
    }
}
