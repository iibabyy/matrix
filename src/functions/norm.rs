use std::ops::Neg;

use crate::vector::Vector;

#[expect(unused)]
impl<K> Vector<K>
where
    K: Copy + Neg,
{
    fn norm_1(&mut self) -> f32 {
        todo!()
    }

    fn norm(&mut self) -> f32 {
        todo!()
    }

    fn norm_inf(&mut self) -> f32 {
        todo!()
    }
}
