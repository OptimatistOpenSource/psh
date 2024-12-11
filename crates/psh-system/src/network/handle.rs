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

use std::collections::HashMap;
use std::time::Duration;

use once_cell::sync::Lazy;
use procfs::net::{self, DeviceStatus};

use crate::error::Result;
use crate::utils::Handle;

static STAT_GLOBAL: Lazy<Handle<HashMap<String, DeviceStatus>>> =
    Lazy::new(|| Handle::new(|| net::dev_status().map_err(Into::into)));

#[derive(Debug, Clone)]
pub struct NetworkHandle(Handle<HashMap<String, DeviceStatus>>);

impl Default for NetworkHandle {
    fn default() -> Self {
        Self(STAT_GLOBAL.clone())
    }
}

impl NetworkHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stat(&self, interval: Option<Duration>) -> Result<HashMap<String, DeviceStatus>> {
        self.0.get(interval)
    }
}
