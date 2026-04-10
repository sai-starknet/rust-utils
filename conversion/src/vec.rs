extern crate alloc;

/// Extension trait for element-wise conversion of a [`Vec`](alloc::vec::Vec) via [`Into`].
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
}
