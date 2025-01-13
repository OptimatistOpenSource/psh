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

use opentelemetry::{metrics::ObservableGauge, Array, KeyValue, Value};
use psh_system::rps::RpsHandle;

impl super::super::Otlp {
    pub fn rps_gauges(&self) -> anyhow::Result<ObservableGauge<u64>> {
        let token = self.token.clone();
        let rps = RpsHandle::new();

        let gauge = self
            .meter
            .u64_observable_gauge("RpsStat")
            .with_description("System profile rps statistics.")
            .with_callback(move |gauge| {
                let Ok(rps_details) = rps.info() else {
                    return;
                };
                for detail in rps_details {
                    let dev = detail.dev;
                    for rps_queue in detail.queues {
                        let gauges = [(
                            rps_queue.flow_cnt.unwrap_or(0).into(),
                            [
                                KeyValue::new("dev", dev.clone()),
                                KeyValue::new("name", rps_queue.name),
                                KeyValue::new("cpu_mask", {
                                    let vec = rps_queue.cpus.map(|c| c.0).unwrap_or_default();
                                    Value::Array(Array::Bool(vec))
                                }),
                            ],
                        )];
                        gauges.into_iter().for_each(|(m, [kv1, kv2, kv3])| {
                            let a = &[KeyValue::new("token", token.clone()), kv1, kv2, kv3];
                            gauge.observe(m, a);
                        });
                    }
                }
            })
            .build();
        Ok(gauge)
    }
}
