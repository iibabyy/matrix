use crate::Vector;

#[expect(unused)]
pub(crate) trait IntoRows<'a, K>: Iterator<Item = &'a Vector<K>>
where
    K: 'a + Copy,
    Self: 'a + Sized,
{
    fn into_rows(self) -> impl Iterator<Item = Vec<&'a K>> {
        let mut col_iter: Vec<_> = self.map(|col| col.scalars.iter()).collect();

        std::iter::from_fn(move || {
            let column: Option<Vec<&K>> = col_iter.iter_mut().map(|iter| iter.next()).collect();

            column
        })
    }
}

#[expect(unused)]
pub(crate) trait IntoRowsMut<'a, K>: Iterator<Item = &'a mut Vector<K>>
where
    K: 'a + Copy,
    Self: 'a + Sized,
{
    fn into_rows_mut(self) -> impl Iterator<Item = Vec<&'a mut K>> {
        let mut col_iter: Vec<_> = self.map(|col| col.scalars.iter_mut()).collect();

        std::iter::from_fn(move || {
            let column: Option<Vec<&mut K>> = col_iter.iter_mut().map(|iter| iter.next()).collect();

            column
        })
    }
}

impl<'a, T, K> IntoRows<'a, K> for T
where
    T: Iterator<Item = &'a Vector<K>> + 'a,
    K: 'a + Copy,
{
}

impl<'a, T, K> IntoRowsMut<'a, K> for T
where
    T: Iterator<Item = &'a mut Vector<K>> + 'a,
    K: 'a + Copy,
{
}
