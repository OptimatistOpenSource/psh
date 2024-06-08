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

use std::process::Command;
use std::time::Duration;

use once_cell::sync::Lazy;

use super::raw::{parse_meminfo, parse_memory_module};
use super::{MemInfo, MemoryModule};
use crate::error::Result;
use crate::utils::Handle;

static STAT_GLOBAL: Lazy<Handle<MemInfo>> =
    Lazy::new(|| Handle::new(|| parse_meminfo!().map_err(Into::into)));

static INFO_GLOBAL: Lazy<Handle<Vec<MemoryModule>>> = Lazy::new(|| {
    Handle::new(|| {
        let dmidecode_exe = which::which("dmidecode")?;
        let output = Command::new(dmidecode_exe).arg("-t").arg("17").output()?;
        let content = std::str::from_utf8(&output.stdout)?;

        Ok(parse_memory_module(content)
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>())
    })
});

#[derive(Debug, Clone)]
pub struct MemoryHandle {
    info: Handle<Vec<MemoryModule>>,
    stat: Handle<MemInfo>,
}

impl Default for MemoryHandle {
    fn default() -> Self {
        Self {
            info: INFO_GLOBAL.clone(),
            stat: STAT_GLOBAL.clone(),
        }
    }
}

impl MemoryHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn info(&self) -> Result<Vec<MemoryModule>> {
        self.info.get(None)
    }

    pub fn stat(&self, interval: Option<Duration>) -> Result<MemInfo> {
        self.stat.get(interval)
    }
}
