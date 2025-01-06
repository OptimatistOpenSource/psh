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

use opentelemetry::metrics::{Meter, ObservableGauge};
use opentelemetry::KeyValue;
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
                let name = stat.name;

                macro_rules! gauges {
                    ($($stat:ident,)+) => {
                        [$((
                            stat.$stat,
                            [
                                KeyValue::new("disk", name.clone()),
                                KeyValue::new("stat", stringify!($stat)),
                            ],
                        ),)*]
                    };
                }
                let gauges = gauges![
                    reads,
                    merged,
                    sectors_read,
                    time_reading,
                    writes,
                    writes_merged,
                    sectors_written,
                    time_writing,
                    in_progress,
                    time_in_progress,
                    weighted_time_in_progress,
                ];
                gauges.into_iter().for_each(|(m, [kv1, kv2])| {
                    let a = &[KeyValue::new("token", token.clone()), kv1, kv2];
                    gauge.observe(m, a);
                });

                macro_rules! gauges {
                    ($($stat:ident,)+) => {
                        [$((
                            stat.$stat.unwrap_or(0),
                            [
                                KeyValue::new("disk", name.clone()),
                                KeyValue::new("stat", stringify!($stat)),
                            ],
                        ),)*]
                    };
                }
                let gauges = gauges![
                    discards,
                    discards_merged,
                    sectors_discarded,
                    time_discarding,
                    flushes,
                    time_flushing,
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
