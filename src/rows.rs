use crate::Vector;

pub(crate) trait AsRows<'a, K>: Iterator<Item = &'a Vector<K>>
where
    K: 'a + Copy,
    Self: 'a + Sized,
{
    fn as_rows(self) -> impl Iterator<Item = Vec<&'a K>> {
        let columns: Vec<Vec<_>> = self.map(|col| col.scalars.iter().collect()).collect();

        let mut row_index = 0;
        let until_index = if columns.is_empty() {
            0
        } else {
            columns[0].len()
        };

        std::iter::from_fn(move || {
            if row_index >= until_index {
                None
            } else {
                row_index += 1;
                Some(columns.iter().map(|col| col[row_index - 1]).collect())
            }
        })
    }
}

impl<'a, T, K> AsRows<'a, K> for T
where
    T: Iterator<Item = &'a Vector<K>> + 'a,
    K: 'a + Copy,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector;

    #[test]
    fn test_as_rows_empty_vectors() {
        let vectors: Vec<Vector<i32>> = vec![];
        let result: Vec<_> = vectors.iter().as_rows().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_as_rows_single_vector() {
        let v1 = Vector::from(vec![1, 2, 3]);
        let vectors = vec![v1];
        let result: Vec<Vec<&i32>> = vectors.iter().as_rows().collect();

        assert_eq!(result.len(), 3); // 3 rows since the vector has 3 elements
        assert_eq!(result[0], vec![&1]);
        assert_eq!(result[1], vec![&2]);
        assert_eq!(result[2], vec![&3]);
    }

    #[test]
    fn test_as_rows_multiple_vectors() {
        let v1 = Vector::from(vec![1, 2]);
        let v2 = Vector::from(vec![3, 4]);
        let vectors = vec![v1, v2];
        let result: Vec<Vec<&i32>> = vectors.iter().as_rows().collect();

        assert_eq!(result.len(), 2); // 2 rows since vectors have 2 elements each
        assert_eq!(result[0], vec![&1, &3]); // First row: [v1[0], v2[0]]
        assert_eq!(result[1], vec![&2, &4]); // Second row: [v1[1], v2[1]]
    }

    #[test]
    fn test_as_rows_different_values() {
        let v1 = Vector::from(vec![10, 20, 30]);
        let v2 = Vector::from(vec![40, 50, 60]);
        let v3 = Vector::from(vec![70, 80, 90]);
        let vectors = vec![v1, v2, v3];
        let result: Vec<Vec<&i32>> = vectors.iter().as_rows().collect();

        assert_eq!(result.len(), 3); // 3 rows since vectors have 3 elements each
        assert_eq!(result[0], vec![&10, &40, &70]); // First row: [v1[0], v2[0], v3[0]]
        assert_eq!(result[1], vec![&20, &50, &80]); // Second row: [v1[1], v2[1], v3[1]]
        assert_eq!(result[2], vec![&30, &60, &90]); // Third row: [v1[2], v2[2], v3[2]]
    }
}

