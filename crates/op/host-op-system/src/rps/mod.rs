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

use crate::profiling::system::rps::{self, RpsInfo as GuestRpsInfo, RpsQueue as GuestRpsQueue};

use psh_system::rps::{RpsDetails as HostRpsInfo, RpsQueue as HostRpsQueue};

use crate::SysCtx;

impl From<&HostRpsQueue> for GuestRpsQueue {
    fn from(value: &HostRpsQueue) -> Self {
        Self {
            name: value.name.clone(),
            cpus: value.cpus.as_ref().map(Into::into),
            flow_cnt: value.flow_cnt,
        }
    }
}

impl From<HostRpsQueue> for GuestRpsQueue {
    fn from(value: HostRpsQueue) -> Self {
        Self {
            name: value.name,
            cpus: value.cpus.map(Into::into),
            flow_cnt: value.flow_cnt,
        }
    }
}

impl From<&HostRpsInfo> for GuestRpsInfo {
    fn from(value: &HostRpsInfo) -> Self {
        Self {
            device: value.dev.clone(),
            queues: value.queues.iter().map(Into::into).collect(),
        }
    }
}

impl From<HostRpsInfo> for GuestRpsInfo {
    fn from(value: HostRpsInfo) -> Self {
        Self {
            device: value.dev,
            queues: value.queues.into_iter().map(Into::into).collect(),
        }
    }
}

impl rps::Host for SysCtx {
    fn info(&mut self) -> wasmtime::Result<Vec<GuestRpsInfo>> {
        Ok(self
            .system
            .rps_info(std::time::Duration::from_secs(1))
            .map_or(vec![], |info| info.into_iter().map(Into::into).collect()))
    }
}
