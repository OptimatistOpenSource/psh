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
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.

use std::{sync::LazyLock, time::Duration};

use procfs::CurrentSI;

use super::{raw::parse_cpuinfo, CpuInfo, CpuStats};
use crate::{error::Result, utils::Handle};

static INFO_GLOBAL: LazyLock<Handle<CpuInfo>> =
    LazyLock::new(|| Handle::new(|| parse_cpuinfo!().map_err(Into::into)));

static STAT_GLOBAL: LazyLock<Handle<CpuStats>> = LazyLock::new(|| {
    Handle::new(|| {
        procfs::KernelStats::current()
            .map(Into::into)
            .map_err(Into::into)
    })
});

#[derive(Debug, Clone)]
pub struct CpuHandle {
    info: Handle<CpuInfo>,
    stat: Handle<CpuStats>,
}

impl Default for CpuHandle {
    fn default() -> Self {
        Self {
            info: INFO_GLOBAL.clone(),
            stat: STAT_GLOBAL.clone(),
        }
    }
}

impl CpuHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn info(&self) -> Result<CpuInfo> {
        self.info.get(None)
    }

    pub fn stat(&self, interval: Option<Duration>) -> Result<CpuStats> {
        self.stat.get(interval)
    }
}
