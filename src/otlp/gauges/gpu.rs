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
use tracing::error;

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
                let gpustats = match nvgpu.stat(Some(interval)) {
                    Ok(stats) => stats,
                    Err(e) => {
                        error!("Failed to collect GPU stats: {}", e);
                        return;
                    }
                };

                for stat in gpustats {
                    // Report per device metrics
                    let device_vals = [
                        // Static fields
                        (stat.irq_num.into(), KeyValue::new("stat", "irq_num")),
                        (
                            stat.max_pcie_link_gen.into(),
                            KeyValue::new("stat", "max_pcie_link_gen"),
                        ),
                        (
                            stat.max_pcie_link_width.into(),
                            KeyValue::new("stat", "max_pcie_link_width"),
                        ),
                        // Temperature and cooling
                        (
                            stat.temperature.into(),
                            KeyValue::new("stat", "temperature"),
                        ),
                        // PCIe status
                        (
                            stat.current_pcie_link_gen.into(),
                            KeyValue::new("stat", "current_pcie_link_gen"),
                        ),
                        (
                            stat.current_pcie_link_width.into(),
                            KeyValue::new("stat", "current_pcie_link_width"),
                        ),
                        // Performance and utilization
                        (
                            stat.utilization_rates.gpu.into(),
                            KeyValue::new("stat", "utilization_rates_gpu"),
                        ),
                        (
                            stat.utilization_rates.memory.into(),
                            KeyValue::new("stat", "utilization_rates_memory"),
                        ),
                        (
                            stat.performance_state.into(),
                            KeyValue::new("stat", "performance_state"),
                        ),
                        (
                            stat.compute_mode.into(),
                            KeyValue::new("stat", "compute_mode"),
                        ),
                        // Memory
                        (
                            stat.memory_info.total,
                            KeyValue::new("stat", "memory_total"),
                        ),
                        (stat.memory_info.used, KeyValue::new("stat", "memory_used")),
                        (stat.memory_info.free, KeyValue::new("stat", "memory_free")),
                        // Power
                        (
                            stat.power_usage.into(),
                            KeyValue::new("stat", "power_usage"),
                        ),
                        (
                            stat.power_limit.into(),
                            KeyValue::new("stat", "power_limit"),
                        ),
                        (
                            stat.enforced_power_limit.into(),
                            KeyValue::new("stat", "enforced_power_limit"),
                        ),
                        // Clocks
                        (
                            stat.memory_clock.into(),
                            KeyValue::new("stat", "memory_clock"),
                        ),
                        (
                            stat.graphics_clock.into(),
                            KeyValue::new("stat", "graphics_clock"),
                        ),
                        (stat.sm_clock.into(), KeyValue::new("stat", "sm_clock")),
                        (
                            stat.video_clock.into(),
                            KeyValue::new("stat", "video_clock"),
                        ),
                        // Processes
                        (
                            stat.graphics_processes_count.into(),
                            KeyValue::new("stat", "graphics_processes_count"),
                        ),
                        (
                            stat.compute_processes_count.into(),
                            KeyValue::new("stat", "compute_processes_count"),
                        ),
                    ];

                    // Report all static metrics
                    for val in device_vals.into_iter() {
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

                    // Report fan speeds with fan index
                    for (idx, speed) in stat.fan_speeds.iter().enumerate() {
                        gauge.observe(
                            (*speed).into(),
                            &[
                                KeyValue::new("host", host.clone()),
                                KeyValue::new("uuid", stat.uuid.clone()),
                                KeyValue::new("name", stat.name.clone()),
                                KeyValue::new("stat", "fan_speed"),
                                KeyValue::new("fan_index", idx.to_string()),
                            ],
                        );
                    }
                }
            })
            .build();
        gauge
    }
}
