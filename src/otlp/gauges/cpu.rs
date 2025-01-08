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
use psh_system::cpu::CpuHandle;

pub fn start(
    token: String,
    meter: &Meter,
    interval: Duration,
) -> anyhow::Result<ObservableGauge<u64>> {
    let cpu = CpuHandle::new();
    let gauge = meter
        .u64_observable_gauge("CpuStat")
        .with_description("System profile cpu statistics.")
        .with_callback(move |gauge| {
            let Ok(cpus) = cpu.stat(Some(interval)) else {
                return;
            };
            let desc = "The amount of time, measured in ticks, the CPU has been in specific states";
            for (cpu, cpu_time) in cpus.per_cpu.into_iter().enumerate() {
                macro_rules! gauges {
                    ($($item:ident,)+) => {
                        [
                            $((
                                cpu_time.$item,
                                [
                                    KeyValue::new("cpu", cpu as i64),
                                    KeyValue::new("stat", stringify!($item)),
                                    KeyValue::new("desc", desc),
                                ],
                            ),)*
                        ]
                    };
                }
                let gauges = gauges![system, idle, user, nice,];
                gauges.into_iter().for_each(|(m, [kv1, kv2, kv3])| {
                    let a = &[KeyValue::new("token", token.clone()), kv1, kv2, kv3];
                    gauge.observe(m, a);
                });

                macro_rules! gauges {
                    ($($item:ident,)+) => {
                        [
                            $((
                                cpu_time.$item(),
                                [
                                    KeyValue::new("cpu", cpu as i64),
                                    KeyValue::new("stat", stringify!($item)),
                                    KeyValue::new("desc", desc),
                                ],
                            ),)*
                        ]
                    };
                }
                let gauges = gauges![user_ms, nice_ms, system_ms, idle_ms,];
                gauges.into_iter().for_each(|(m, [kv1, kv2, kv3])| {
                    let a = &[KeyValue::new("token", token.clone()), kv1, kv2, kv3];
                    gauge.observe(m, a);
                });

                macro_rules! gauges {
                    ($($item_o:ident,)+) => {
                        [
                            $((
                                cpu_time.$item_o.unwrap_or(0),
                                [
                                    KeyValue::new("cpu", cpu as i64),
                                    KeyValue::new("stat", stringify!($item_o)),
                                    KeyValue::new("desc", desc),
                                ],
                            ),)*
                        ]
                    };
                }
                let gauges = gauges![iowait, irq, softirq, steal, guest, guest_nice,];
                gauges.into_iter().for_each(|(m, [kv1, kv2, kv3])| {
                    let a = &[KeyValue::new("token", token.clone()), kv1, kv2, kv3];
                    gauge.observe(m, a);
                });

                macro_rules! gauges {
                    ($($item_o:ident,)+) => {
                        [
                            $((
                                cpu_time.$item_o().unwrap_or(0),
                                [
                                    KeyValue::new("cpu", cpu as i64),
                                    KeyValue::new("stat", stringify!($item_o)),
                                    KeyValue::new("desc", desc),
                                ],
                            ),)*
                        ]
                    };
                }
                let gauges = gauges![
                    iowait_ms,
                    irq_ms,
                    softirq_ms,
                    steal_ms,
                    guest_ms,
                    guest_nice_ms,
                ];
                gauges.into_iter().for_each(|(m, [kv1, kv2, kv3])| {
                    let a = &[KeyValue::new("token", token.clone()), kv1, kv2, kv3];
                    gauge.observe(m, a);
                });
            }
        })
        .build();
    Ok(gauge)
}
