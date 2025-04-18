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

use std::time::Duration;

use psh_system::disk::DiskStat as HostDiskStat;

use crate::{
    SysCtx,
    profiling::system::disk::{
        self, DiskOperationStat as GuestDiskOperationStat, DiskStat as GuestDiskStat,
    },
};

impl From<&HostDiskStat> for GuestDiskStat {
    fn from(value: &HostDiskStat) -> Self {
        let read = GuestDiskOperationStat {
            operations: value.reads,
            sectors: value.sectors_read,
            merged: value.merged,
            time: value.time_reading,
        };
        let write = GuestDiskOperationStat {
            operations: value.writes,
            sectors: value.sectors_written,
            merged: value.writes_merged,
            time: value.time_writing,
        };
        let discard = value
            .discards
            .zip(value.sectors_discarded)
            .zip(value.discards_merged)
            .zip(value.time_discarding)
            .map(
                |(((operations, sectors), merged), time)| GuestDiskOperationStat {
                    operations,
                    sectors,
                    merged,
                    time,
                },
            );
        Self {
            name: value.name.clone(),
            major: value.major,
            minor: value.minor,
            read,
            write,
            discard,
            in_progress: value.in_progress,
            time_in_progress: value.time_in_progress,
            weighted_time_in_progress: value.weighted_time_in_progress,
            flushes: value.flushes,
            time_flushing: value.time_flushing,
        }
    }
}

impl From<HostDiskStat> for GuestDiskStat {
    fn from(value: HostDiskStat) -> Self {
        let read = GuestDiskOperationStat {
            operations: value.reads,
            sectors: value.sectors_read,
            merged: value.merged,
            time: value.time_reading,
        };
        let write = GuestDiskOperationStat {
            operations: value.writes,
            sectors: value.sectors_written,
            merged: value.writes_merged,
            time: value.time_writing,
        };
        let discard = value
            .discards
            .zip(value.sectors_discarded)
            .zip(value.discards_merged)
            .zip(value.time_discarding)
            .map(
                |(((operations, sectors), merged), time)| GuestDiskOperationStat {
                    operations,
                    sectors,
                    merged,
                    time,
                },
            );
        Self {
            name: value.name,
            major: value.major,
            minor: value.minor,
            read,
            write,
            discard,
            in_progress: value.in_progress,
            time_in_progress: value.time_in_progress,
            weighted_time_in_progress: value.weighted_time_in_progress,
            flushes: value.flushes,
            time_flushing: value.time_flushing,
        }
    }
}

impl disk::Host for SysCtx {
    fn stat(&mut self, interval_ms: u64) -> Result<Vec<GuestDiskStat>, String> {
        self.disk
            .stat(Some(Duration::from_millis(interval_ms)))
            .map(|disks| disks.into_iter().map(Into::into).collect())
            .map_err(|err| err.to_string())
    }
}
