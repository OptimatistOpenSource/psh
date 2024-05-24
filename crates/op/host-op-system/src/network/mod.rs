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

use std::time::Duration;

use crate::{
    profiling::system::network::{self, NetworkStat as GuestNetworkStat},
    SysCtx,
};

use psh_system::network::DeviceStatus;

impl From<&DeviceStatus> for GuestNetworkStat {
    fn from(value: &DeviceStatus) -> Self {
        Self {
            name: value.name.clone(),
            recv_bytes: value.recv_bytes,
            recv_packets: value.recv_packets,
            recv_errors: value.recv_errs,
            recv_drop: value.recv_drop,
            recv_fifo: value.recv_fifo,
            recv_frame: value.recv_frame,
            recv_compressed: value.recv_compressed,
            recv_multicast: value.recv_multicast,
            sent_bytes: value.sent_bytes,
            sent_packets: value.sent_packets,
            sent_errors: value.sent_errs,
            sent_drop: value.sent_drop,
            sent_fifo: value.sent_fifo,
            sent_collisions: value.sent_colls,
            sent_carrier: value.sent_carrier,
            sent_compressed: value.sent_compressed,
        }
    }
}

impl From<DeviceStatus> for GuestNetworkStat {
    fn from(value: DeviceStatus) -> Self {
        Self {
            name: value.name,
            recv_bytes: value.recv_bytes,
            recv_packets: value.recv_packets,
            recv_errors: value.recv_errs,
            recv_drop: value.recv_drop,
            recv_fifo: value.recv_fifo,
            recv_frame: value.recv_frame,
            recv_compressed: value.recv_compressed,
            recv_multicast: value.recv_multicast,
            sent_bytes: value.sent_bytes,
            sent_packets: value.sent_packets,
            sent_errors: value.sent_errs,
            sent_drop: value.sent_drop,
            sent_fifo: value.sent_fifo,
            sent_collisions: value.sent_colls,
            sent_carrier: value.sent_carrier,
            sent_compressed: value.sent_compressed,
        }
    }
}

impl network::Host for SysCtx {
    fn stat(
        &mut self,
        interval_ms: u64,
    ) -> wasmtime::Result<Result<Vec<GuestNetworkStat>, String>> {
        let networks = self
            .network
            .stat(Some(Duration::from_millis(interval_ms)))
            .map(|nets| nets.into_values().map(Into::into).collect())
            .map_err(|err| err.to_string());

        Ok(networks)
    }
}
