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
use psh_system::vmstat::VmstatHandle;

impl super::super::Otlp {
    pub fn vmstat_gauges(&self) -> ObservableGauge<u64> {
        let host = self.host.clone();
        let interval = self.interval;
        let vmstat = VmstatHandle::new();

        let gauge = self
            .meter
            .u64_observable_gauge("VmStat")
            .with_description("System profile vmstat statistics.")
            .with_callback(move |gauge| {
                let Ok(stat) = vmstat.stat(Some(interval)) else {
                    return;
                };

                for (k, v) in stat {
                    gauge.observe(
                        v as u64,
                        &[
                            KeyValue::new("stat", k),
                            KeyValue::new("host", host.clone()),
                        ],
                    )
                }
            })
            .build();
        gauge
    }
}
