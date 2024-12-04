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

use std::{borrow::Cow, time::Duration};

use opentelemetry::{
    metrics::{Meter, ObservableGauge},
    KeyValue,
};
use psh_system::disk::DiskHandle;

pub fn start(
    token: String,
    meter: Meter,
    interval: Duration,
) -> anyhow::Result<ObservableGauge<u64>> {
    let disk = DiskHandle::new();
    let gauge = meter
        .u64_observable_gauge("DiskStat")
        .with_description("System profile disk statistics.")
        // .with_unit(Unit::new("KiB"))
        .with_callback(move |gauge| {
            let Ok(disks) = disk.stat(Some(interval)) else {
                return;
            };
            for stat in disks {
                // TODO
                let name = Cow::from(stat.name);

                let gauges = [
                    (
                        stat.reads,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "reads"),
                        ],
                    ),
                    (
                        stat.merged,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "merged"),
                        ],
                    ),
                    (
                        stat.sectors_read,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "sectors_read"),
                        ],
                    ),
                    (
                        stat.time_reading,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "time_reading"),
                        ],
                    ),
                    (
                        stat.writes,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "writes"),
                        ],
                    ),
                    (
                        stat.writes_merged,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "writes_merged"),
                        ],
                    ),
                    (
                        stat.sectors_written,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "sectors_written"),
                        ],
                    ),
                    (
                        stat.time_writing,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "time_writing"),
                        ],
                    ),
                    (
                        stat.in_progress,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "in_progress"),
                        ],
                    ),
                    (
                        stat.time_in_progress,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "time_in_progress"),
                        ],
                    ),
                    (
                        stat.weighted_time_in_progress,
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "weighted_time_in_progress"),
                        ],
                    ),
                    (
                        stat.discards.unwrap_or(0),
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "discards"),
                        ],
                    ),
                    (
                        stat.discards_merged.unwrap_or(0),
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "discards_merged"),
                        ],
                    ),
                    (
                        stat.sectors_discarded.unwrap_or(0),
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "sectors_discarded"),
                        ],
                    ),
                    (
                        stat.time_discarding.unwrap_or(0),
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "time_discarding"),
                        ],
                    ),
                    (
                        stat.flushes.unwrap_or(0),
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "flushes"),
                        ],
                    ),
                    (
                        stat.time_flushing.unwrap_or(0),
                        [
                            KeyValue::new("disk", name.clone()),
                            KeyValue::new("stat", "time_flushing"),
                        ],
                    ),
                ];

                gauges.into_iter().for_each(|(m, [kv1, kv2])| {
                    let a = &[KeyValue::new("token", token.clone()), kv1, kv2];
                    gauge.observe(m, a);
                });
            }
        })
        .build();
    Ok(gauge)
}
