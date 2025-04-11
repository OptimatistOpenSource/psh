// Copyright (c) 2024-2025 Optimatist Technology Co., Ltd. All rights reserved.
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
use psh_system::gpu::NvidiaHandle;

impl super::super::Otlp {
    pub fn gpu_gauges(&self) -> ObservableGauge<u64> {
        let host = self.host.clone();
        let interval = self.interval;
        let nvgpu = NvidiaHandle::new();

        let gauge = self
            .meter
            .u64_observable_gauge("NvGpuStat")
            .with_description("System profile nvgpu statistics.")
            .with_callback(move |gauge| {
                let Ok(gpustats) = nvgpu.stat(Some(interval)) else {
                    return;
                };

                for stat in gpustats {
                    let vals = [
                        (stat.irq_num.into(), KeyValue::new("stat", "irq_num")),
                        (
                            stat.temperature.into(),
                            KeyValue::new("stat", "temperature"),
                        ),
                        (
                            stat.max_pcie_link_gen.into(),
                            KeyValue::new("stat", "max_pcie_link_gen"),
                        ),
                        (
                            stat.memory_info.total,
                            KeyValue::new("stat", "memory_total"),
                        ),
                        (stat.memory_info.used, KeyValue::new("stat", "memory_used")),
                        (
                            stat.utilization_rates.memory.into(),
                            KeyValue::new("stat", "utilization_rates_memory"),
                        ),
                        (
                            stat.utilization_rates.gpu.into(),
                            KeyValue::new("stat", "utilization_rates_gpu"),
                        ),
                    ];
                    for val in vals.into_iter() {
                        gauge.observe(
                            val.0,
                            &[
                                KeyValue::new("host", host.clone()),
                                KeyValue::new("uuid", stat.uuid.clone()),
                                KeyValue::new("name", stat.name.clone()),
                                val.1,
                            ],
                        );
                    }
                }
            })
            .build();
        gauge
    }
}
