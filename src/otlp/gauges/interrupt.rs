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

use opentelemetry::{KeyValue, metrics::ObservableGauge};
use psh_system::interrupt::InterruptHandle;

impl super::super::Otlp {
    pub fn irq_gauges(&self) -> ObservableGauge<u64> {
        let host = self.host.clone();
        let interval = self.interval;
        let interrupt = InterruptHandle::new();

        let gauge = self
            .meter
            .u64_observable_gauge("InterruptStat")
            .with_description("System profile interrupt statistics.")
            .with_callback(move |gauge| {
                let Ok(irqs) = interrupt.stat(Some(interval)) else {
                    return;
                };

                for int in irqs {
                    let desc = Cow::from(int.description);
                    for (cpu, &cnt) in int.cpu_counts.iter().enumerate() {
                        let a = [
                            KeyValue::new("host", host.clone()),
                            KeyValue::new("desc", desc.clone()),
                            KeyValue::new("cpu", cpu as i64),
                            KeyValue::new("type", int.interrupt_type.to_string()),
                        ];
                        gauge.observe(cnt, &a)
                    }
                }
            })
            .build();
        gauge
    }
}
