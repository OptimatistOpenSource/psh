pub trait WrapResult
where
    Self: Sized,
{
    #[inline]
    fn wrap_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
    #[inline]
    fn wrap_err<O>(self) -> Result<O, Self> {
        Err(self)
    }
}

impl<T> WrapResult for T {}

#[cfg(test)]
mod tests {
    use crate::infra::result::WrapResult;

    #[test]
    fn test_wrap_ok() {
        assert_eq!(().wrap_ok::<()>(), Ok(()));
    }

    #[test]
    fn test_wrap_err() {
        assert_eq!(().wrap_err::<()>(), Err(()));
    }
}
