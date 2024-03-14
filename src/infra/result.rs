// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of PSH.
//
// PSH is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// PSH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

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
