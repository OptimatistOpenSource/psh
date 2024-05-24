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

use once_cell::sync::Lazy;

use super::{
    raw::{get_kernel_version, parse_distro_version},
    OsInfo,
};

use crate::error::{Error, Result};
use crate::utils::Handle;

static INFO_GLOBAL: Lazy<Handle<OsInfo>> = Lazy::new(|| {
    Handle::new(|| match (parse_distro_version!(), get_kernel_version()) {
        (Ok(distro), Ok(kernel)) => Ok(OsInfo { distro, kernel }),
        _ => Err(Error::EmptyValue),
    })
});

#[derive(Debug, Clone)]
pub struct OsHandle(Handle<OsInfo>);

impl Default for OsHandle {
    fn default() -> Self {
        Self(INFO_GLOBAL.clone())
    }
}

impl OsHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn info(&self) -> Result<OsInfo> {
        self.0.get(None)
    }
}
