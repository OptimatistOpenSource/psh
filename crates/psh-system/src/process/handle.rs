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

use std::sync::Arc;
use std::time::Duration;

use once_cell::sync::Lazy;
use procfs::process::Process;

use crate::error::Result;
use crate::utils::Handle;

static INFO_SELF_GLOBAL: Lazy<Handle<Arc<Process>>> =
    Lazy::new(|| Handle::new(|| Process::myself().map(Arc::new).map_err(Into::into)));

static STAT_ALL_GLOBAL: Lazy<Handle<Vec<Arc<Process>>>> = Lazy::new(|| {
    Handle::new(|| {
        procfs::process::all_processes()
            .map_err(Into::into)
            .map(|iter| iter.filter_map(|proc| proc.ok().map(Arc::new)).collect())
    })
});

#[derive(Debug, Clone)]
pub struct ProcessHandle {
    myself: Handle<Arc<Process>>,
    all: Handle<Vec<Arc<Process>>>,
}

impl ProcessHandle {
    pub fn new() -> Self {
        Self {
            myself: INFO_SELF_GLOBAL.clone(),
            all: STAT_ALL_GLOBAL.clone(),
        }
    }

    pub fn myself(&self) -> Result<Arc<Process>> {
        self.myself.get(None)
    }

    pub fn all(&self, interval: Option<Duration>) -> Result<Vec<Arc<Process>>> {
        self.all.get(interval)
    }
}
