pub trait WrapOption
where
    Self: Sized,
{
    #[inline]
    fn wrap_some(self) -> Option<Self> {
        Some(self)
    }
}

impl<T> WrapOption for T {}

#[cfg(test)]
mod tests {
    use crate::infra::option::WrapOption;

    #[test]
    fn test_wrap_some() {
        assert_eq!(().wrap_some(), Some(()));
    }
}
