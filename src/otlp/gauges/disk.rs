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

use opentelemetry::{KeyValue, metrics::ObservableGauge};
use psh_system::disk::DiskHandle;

impl super::super::Otlp {
    pub fn disk_gagues(&self) -> ObservableGauge<u64> {
        let host = self.host.clone();
        let interval = self.interval;
        let disk = DiskHandle::new();

        let gauge = self
            .meter
            .u64_observable_gauge("DiskStat")
            .with_description("System profile disk statistics.")
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
                        let a = &[KeyValue::new("host", host.clone()), kv1, kv2];
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
                        let a = &[KeyValue::new("host", host.clone()), kv1, kv2];
                        gauge.observe(m, a);
                    });
                }
            })
            .build();
        gauge
    }
}
