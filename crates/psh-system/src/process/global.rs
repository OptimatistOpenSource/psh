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

use once_cell::sync::Lazy;
use procfs::process::Process;

use crate::utils::Handle;

static STAT_SELF_GLOBAL: Lazy<Handle<Arc<Process>>> =
    Lazy::new(|| Handle::new(|| Process::myself().map(Arc::new).map_err(Into::into)));

pub fn stat_self_handle() -> Handle<Arc<Process>> {
    STAT_SELF_GLOBAL.clone()
}

static STAT_ALL_GLOBAL: Lazy<Handle<Vec<Arc<Process>>>> = Lazy::new(|| {
    Handle::new(|| {
        procfs::process::all_processes()
            .map_err(Into::into)
            .map(|iter| iter.filter_map(|proc| proc.ok().map(Arc::new)).collect())
    })
});

pub fn stat_all_handle() -> Handle<Vec<Arc<Process>>> {
    STAT_ALL_GLOBAL.clone()
}
