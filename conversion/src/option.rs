/// Extension trait for converting the inner type of an [`Option`] via [`Into`].
pub trait OptionInto<T> {
    /// Convert the `Some` variant into a new type, leaving `None` unchanged.
    fn opt_into<RT>(self) -> Option<RT>
    where
        T: Into<RT>;
}

impl<T> OptionInto<T> for Option<T> {
    fn opt_into<RT>(self) -> Option<RT>
    where
        T: Into<RT>,
    {
        self.map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opt_into_converts_some() {
        let o: Option<u8> = Some(5);
        let converted: Option<u16> = o.opt_into();
        assert_eq!(converted, Some(5u16));
    }

    #[test]
    fn opt_into_preserves_none() {
        let o: Option<u8> = None;
        let converted: Option<u16> = o.opt_into();
        assert_eq!(converted, None);
    }
}
