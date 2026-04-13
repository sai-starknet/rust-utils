/// Extension trait for element-wise conversion of a [`Vec`](alloc::vec::Vec) via [`Into`].
///
/// For a more general form that works with any [`IntoIterator`] and target
/// collection, see [`ItemsInto`].
pub trait VecInto<T> {
    /// Convert each element of the vector into a new type.
    fn vec_into<RT>(self) -> alloc::vec::Vec<RT>
    where
        T: Into<RT>;
}

impl<T> VecInto<T> for alloc::vec::Vec<T> {
    fn vec_into<RT>(self) -> alloc::vec::Vec<RT>
    where
        T: Into<RT>,
    {
        self.into_iter().map(Into::into).collect()
    }
}

/// Extension trait for converting each element of an iterator into a new type.
///
/// This is the general form of [`VecInto`] — it works with any [`IntoIterator`]
/// and can collect into any type that implements [`FromIterator`].
pub trait ItemsInto {
    /// The type of items in the iterator.
    type Item;
    /// Convert each element of an iterator into a new type.
    fn items_into<I, C>(self) -> C
    where
        Self::Item: Into<I>,
        C: FromIterator<I>;
}

impl<IT> ItemsInto for IT
where
    IT: IntoIterator,
{
    type Item = IT::Item;

    fn items_into<II, CC>(self) -> CC
    where
        Self::Item: Into<II>,
        CC: FromIterator<II>,
    {
        self.into_iter().map(Into::into).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn vec_into_converts_elements() {
        let v: alloc::vec::Vec<u8> = vec![1, 2, 3];
        let converted: alloc::vec::Vec<u16> = v.vec_into();
        assert_eq!(converted, vec![1u16, 2u16, 3u16]);
    }

    #[test]
    fn vec_into_empty() {
        let v: alloc::vec::Vec<u8> = vec![];
        let converted: alloc::vec::Vec<u16> = v.vec_into();
        assert!(converted.is_empty());
    }

    #[test]
    fn items_into_vec() {
        let v: alloc::vec::Vec<u8> = vec![1, 2, 3];
        let converted: alloc::vec::Vec<u16> = v.items_into();
        assert_eq!(converted, vec![1u16, 2u16, 3u16]);
    }

    #[test]
    fn items_into_collects_to_btreeset() {
        use alloc::collections::BTreeSet;
        let v: alloc::vec::Vec<u8> = vec![3, 1, 2, 1];
        let converted: BTreeSet<u16> = v.items_into();
        assert_eq!(converted.len(), 3);
        assert!(converted.contains(&1u16));
        assert!(converted.contains(&2u16));
        assert!(converted.contains(&3u16));
    }

    #[test]
    fn items_into_empty() {
        let v: alloc::vec::Vec<u8> = vec![];
        let converted: alloc::vec::Vec<u16> = v.items_into();
        assert!(converted.is_empty());
    }
}
