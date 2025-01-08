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

use std::borrow::Cow;
use std::time::Duration;

use opentelemetry::metrics::{Meter, ObservableGauge};
use opentelemetry::KeyValue;
use psh_system::interrupt::InterruptHandle;

pub fn start(
    token: String,
    meter: Meter,
    interval: Duration,
) -> anyhow::Result<ObservableGauge<u64>> {
    let interrupt = InterruptHandle::new();
    let gauge = meter
        .u64_observable_gauge("InterruptStat")
        .with_description("System profile interrupt statistics.")
        // .with_unit(Unit::new("KiB"))
        .with_callback(move |gauge| {
            let Ok(irqs) = interrupt.stat(Some(interval)) else {
                return;
            };

            for int in irqs {
                let desc = Cow::from(int.description);
                for (cpu, &cnt) in int.cpu_counts.iter().enumerate() {
                    let a = [
                        KeyValue::new("token", token.clone()),
                        KeyValue::new("desc", desc.clone()),
                        KeyValue::new("cpu", cpu as i64),
                        KeyValue::new("type", int.interrupt_type.to_string()),
                    ];
                    gauge.observe(cnt, &a)
                }
            }
        })
        .build();
    Ok(gauge)
}
