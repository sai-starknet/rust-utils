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

/// Extension trait for converting the inner types of a [`Result`] via [`Into`].
///
/// These methods perform infallible type conversions — the `Result` structure
/// is preserved and only the inner types change.
#[allow(clippy::missing_errors_doc)]
pub trait ResultInto<T, E> {
    /// Convert both the `Ok` and `Err` variants into new types.
    fn result_into<RT, RE>(self) -> Result<RT, RE>
    where
        E: Into<RE>,
        T: Into<RT>;
    /// Convert the `Ok` variant into a new type, leaving `Err` unchanged.
    fn ok_into<RT>(self) -> Result<RT, E>
    where
        T: Into<RT>;
    /// Convert the `Err` variant into a new type, leaving `Ok` unchanged.
    fn err_into<RE>(self) -> Result<T, RE>
    where
        E: Into<RE>;
}

impl<T, E> ResultInto<T, E> for Result<T, E> {
    fn result_into<RT, RE>(self) -> Result<RT, RE>
    where
        E: Into<RE>,
        T: Into<RT>,
    {
        match self {
            Ok(t) => Ok(t.into()),
            Err(e) => Err(e.into()),
        }
    }
    fn ok_into<RT>(self) -> Result<RT, E>
    where
        T: Into<RT>,
    {
        self.map(Into::into)
    }
    fn err_into<RE>(self) -> Result<T, RE>
    where
        E: Into<RE>,
    {
        self.map_err(Into::into)
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

    #[test]
    fn result_into_converts_ok() {
        let r: Result<u8, u8> = Ok(1);
        let converted: Result<u16, u16> = r.result_into();
        assert_eq!(converted, Ok(1u16));
    }

    #[test]
    fn result_into_converts_err() {
        let r: Result<u8, u8> = Err(2);
        let converted: Result<u16, u16> = r.result_into();
        assert_eq!(converted, Err(2u16));
    }

    #[test]
    fn ok_into_converts_ok_only() {
        let r: Result<u8, u16> = Ok(5);
        let converted: Result<u16, u16> = r.ok_into();
        assert_eq!(converted, Ok(5u16));
    }

    #[test]
    fn ok_into_preserves_err() {
        let r: Result<u8, u16> = Err(10);
        let converted: Result<u16, u16> = r.ok_into();
        assert_eq!(converted, Err(10u16));
    }

    #[test]
    fn err_into_converts_err_only() {
        let r: Result<u16, u8> = Err(3);
        let converted: Result<u16, u16> = r.err_into();
        assert_eq!(converted, Err(3u16));
    }

    #[test]
    fn err_into_preserves_ok() {
        let r: Result<u16, u8> = Ok(7);
        let converted: Result<u16, u16> = r.err_into();
        assert_eq!(converted, Ok(7u16));
    }
}
