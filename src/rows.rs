use std::ops::Neg;
use crate::Vector;

#[expect(unused)]
pub(crate) trait AsRows<'a, K>: Iterator<Item = &'a Vector<K>>
where
	K: Copy + std::ops::Neg,
	K: 'a,
	Self: 'a + Sized
{
    fn rows(self) -> impl Iterator<Item = Vec<&'a K>> {
        let mut col_iter: Vec<_> = self
            .map(|col| col.scalars.iter())
            .collect();

        let rows = std::iter::from_fn(move || {
            let column: Option<Vec<&K>> = col_iter
                .iter_mut()
                .map(|iter| iter.next())
                .collect();

            column
        });

        rows
    }
}

#[expect(unused)]
pub(crate) trait AsRowsMut<'a, K>: Iterator<Item = &'a mut Vector<K>>
where
	K: Copy + std::ops::Neg,
	K: 'a,
	Self: 'a + Sized
{
    fn rows(self) -> impl Iterator<Item = Vec<&'a mut K>> {
        let mut col_iter: Vec<_> = self
            .map(|col| col.scalars.iter_mut())
            .collect();

        let rows = std::iter::from_fn(move || {
            let column: Option<Vec<&mut K>> = col_iter
                .iter_mut()
                .map(|iter| iter.next())
                .collect();

            column
        });

        rows
    }
}

impl<'a, T, K> AsRows<'a, K> for T
where
	T: Iterator<Item = &'a Vector<K>> + 'a,
	K: Copy + Neg + 'a
{}

impl<'a, T, K> AsRowsMut<'a, K> for T
where
	T: Iterator<Item = &'a mut Vector<K>> + 'a,
	K: Copy + Neg + 'a
{}
