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
use psh_system::memory::MemoryHandle;

pub fn start(
    token: String,
    meter: Meter,
    interval: Duration,
) -> anyhow::Result<ObservableGauge<u64>> {
    let memory = MemoryHandle::new();

    let gauge = meter
        .u64_observable_gauge("MemoryStat")
        .with_description("System profile memory statistics.")
        // .with_unit(Unit::new("KiB"))
        .with_callback(move |gauge| {
            let Ok(mem) = memory.stat(Some(interval)) else {
                return;
            };
            macro_rules! gauges_mem {
                ($($stat:ident), *,) => {
                    [
                        $(
                        (mem.$stat, KeyValue::new("stat", stringify!($stat))),
                        )*
                    ]
                };
            }
            let gauges = gauges_mem!(
                mem_total,
                mem_free,
                mem_available,
                buffers,
                cached,
                swap_cached,
                active,
                inactive,
                active_anon,
                inactive_anon,
                active_file,
                inactive_file,
                unevictable,
                mlocked,
                swap_total,
                swap_free,
                dirty,
                writeback,
                anon_pages,
                mapped,
                shmem,
                kreclaimable,
                slab,
                sreclaimable,
                sunreclaim,
                kernel_stack,
                page_tables,
                nfs_unstable,
                bounce,
                writeback_tmp,
                commit_limit,
                committed_as,
                vmalloc_total,
                vmalloc_used,
                vmalloc_chunk,
                percpu,
                huge_pages_total,
                huge_pages_free,
                huge_pages_rsvd,
                huge_pages_surp,
                huge_page_size,
                huge_tlb,
            );

            gauges.into_iter().for_each(|(m, kv)| {
                gauge.observe(m, &[KeyValue::new("token", token.clone()), kv]);
            });

            macro_rules! gauges_mem_ {
                ($($stat:ident), *,) => {
                    [
                        $(
                        (mem.$stat.unwrap_or(0), KeyValue::new("stat", stringify!($stat))),
                        )*
                    ]
                };
            }
            let gauges = gauges_mem_!(
                cma_total,
                cma_free,
                hardware_corrupted,
                anon_huge_pages,
                shmem_huge_pages,
                shmem_pmd_mapped,
                file_huge_pages,
                file_pmd_mapped,
                direct_map4k,
                direct_map2_m,
                direct_map1_g,
            );

            gauges.into_iter().for_each(|(m, kv)| {
                gauge.observe(m, &[KeyValue::new("token", token.clone()), kv]);
            })
        })
        .build();
    Ok(gauge)
}
