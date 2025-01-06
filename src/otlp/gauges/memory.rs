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

            macro_rules! gauges {
                ($($stat:ident,)+) => {
                    [
                        $(
                        (mem.$stat, KeyValue::new("stat", stringify!($stat))),
                        )*
                    ]
                };
            }
            let gauges = gauges![
                mem_total,
                mem_free,
                buffers,
                cached,
                swap_cached,
                active,
                inactive,
                swap_total,
                swap_free,
                dirty,
                writeback,
                mapped,
                slab,
                committed_as,
                vmalloc_total,
                vmalloc_used,
                vmalloc_chunk,
            ];
            gauges.into_iter().for_each(|(m, kv)| {
                gauge.observe(m, &[KeyValue::new("token", token.clone()), kv]);
            });

            macro_rules! gauges {
                ($($stat:ident,)+) => {
                    [
                        $(
                        (mem.$stat.unwrap_or(0), KeyValue::new("stat", stringify!($stat))),
                        )*
                    ]
                };
            }
            let gauges = gauges![
                cma_total,
                cma_free,
                hugepages_total,
                hugepages_free,
                hugepages_rsvd,
                hugepages_surp,
                hugepagesize,
                hugetlb,
                per_cpu,
                hardware_corrupted,
                anon_hugepages,
                shmem_hugepages,
                shmem_pmd_mapped,
                file_huge_pages,
                file_pmd_mapped,
                direct_map_4k,
                direct_map_2M,
                direct_map_1G,
                k_reclaimable,
                commit_limit,
                writeback_tmp,
                mem_available,
                active_anon,
                inactive_anon,
                active_file,
                inactive_file,
                unevictable,
                mlocked,
                anon_pages,
                shmem,
                s_reclaimable,
                s_unreclaim,
                kernel_stack,
                page_tables,
                nfs_unstable,
                bounce,
                z_swap,
                z_swapped,
                secondary_page_tables,
            ];

            gauges.into_iter().for_each(|(m, kv)| {
                gauge.observe(m, &[KeyValue::new("token", token.clone()), kv]);
            })
        })
        .build();
    Ok(gauge)
}
