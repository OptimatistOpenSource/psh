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
            let gauges = [
                (mem.mem_total, KeyValue::new("stat", "mem_total")),
                (mem.mem_free, KeyValue::new("stat", "mem_free")),
                (mem.mem_available, KeyValue::new("stat", "mem_available")),
                (mem.buffers, KeyValue::new("stat", "buffers")),
                (mem.cached, KeyValue::new("stat", "cached")),
                (mem.swap_cached, KeyValue::new("stat", "swap_cached")),
                (mem.active, KeyValue::new("stat", "active")),
                (mem.inactive, KeyValue::new("stat", "inactive")),
                (mem.active_anon, KeyValue::new("stat", "active_anon")),
                (mem.inactive_anon, KeyValue::new("stat", "inactive_anon")),
                (mem.active_file, KeyValue::new("stat", "active_file")),
                (mem.inactive_file, KeyValue::new("stat", "inactive_file")),
                (mem.unevictable, KeyValue::new("stat", "unevictable")),
                (mem.mlocked, KeyValue::new("stat", "mlocked")),
                (mem.swap_total, KeyValue::new("stat", "swap_total")),
                (mem.swap_free, KeyValue::new("stat", "swap_free")),
                (mem.dirty, KeyValue::new("stat", "dirty")),
                (mem.writeback, KeyValue::new("stat", "writeback")),
                (mem.anon_pages, KeyValue::new("stat", "anon_pages")),
                (mem.mapped, KeyValue::new("stat", "mapped")),
                (mem.shmem, KeyValue::new("stat", "shmem")),
                (mem.kreclaimable, KeyValue::new("stat", "kreclaimable")),
                (mem.slab, KeyValue::new("stat", "slab")),
                (mem.sreclaimable, KeyValue::new("stat", "sreclaimable")),
                (mem.sunreclaim, KeyValue::new("stat", "sunreclaim")),
                (mem.kernel_stack, KeyValue::new("stat", "kernel_stack")),
                (mem.page_tables, KeyValue::new("stat", "page_tables")),
                (mem.nfs_unstable, KeyValue::new("stat", "nfs_unstable")),
                (mem.bounce, KeyValue::new("stat", "bounce")),
                (mem.writeback_tmp, KeyValue::new("stat", "writeback_tmp")),
                (mem.commit_limit, KeyValue::new("stat", "commit_limit")),
                (mem.committed_as, KeyValue::new("stat", "committed_as")),
                (mem.vmalloc_total, KeyValue::new("stat", "vmalloc_total")),
                (mem.vmalloc_used, KeyValue::new("stat", "vmalloc_used")),
                (mem.vmalloc_chunk, KeyValue::new("stat", "vmalloc_chunk")),
                (mem.percpu, KeyValue::new("stat", "percpu")),
                (
                    mem.cma_total.unwrap_or(0),
                    KeyValue::new("stat", "cma_total"),
                ),
                (mem.cma_free.unwrap_or(0), KeyValue::new("stat", "cma_free")),
                (
                    mem.hardware_corrupted.unwrap_or(0),
                    KeyValue::new("stat", "hardware_corrupted"),
                ),
                (
                    mem.anon_huge_pages.unwrap_or(0),
                    KeyValue::new("stat", "anon_huge_pages"),
                ),
                (
                    mem.shmem_huge_pages.unwrap_or(0),
                    KeyValue::new("stat", "shmem_huge_pages"),
                ),
                (
                    mem.shmem_pmd_mapped.unwrap_or(0),
                    KeyValue::new("stat", "shmem_pmd_mapped"),
                ),
                (
                    mem.file_huge_pages.unwrap_or(0),
                    KeyValue::new("stat", "file_huge_pages"),
                ),
                (
                    mem.file_pmd_mapped.unwrap_or(0),
                    KeyValue::new("stat", "file_pmd_mapped"),
                ),
                (
                    mem.huge_pages_total,
                    KeyValue::new("stat", "huge_pages_total"),
                ),
                (
                    mem.huge_pages_free,
                    KeyValue::new("stat", "huge_pages_free"),
                ),
                (
                    mem.huge_pages_rsvd,
                    KeyValue::new("stat", "huge_pages_rsvd"),
                ),
                (
                    mem.huge_pages_surp,
                    KeyValue::new("stat", "huge_pages_surp"),
                ),
                (mem.huge_page_size, KeyValue::new("stat", "huge_page_size")),
                (mem.huge_tlb, KeyValue::new("stat", "huge_tlb")),
                (
                    mem.direct_map4k.unwrap_or(0),
                    KeyValue::new("stat", "direct_map4k"),
                ),
                (
                    mem.direct_map2_m.unwrap_or(0),
                    KeyValue::new("stat", "direct_map2_m"),
                ),
                (
                    mem.direct_map1_g.unwrap_or(0),
                    KeyValue::new("stat", "direct_map1_g"),
                ),
            ];

            gauges.into_iter().for_each(|(m, kv)| {
                gauge.observe(m, &[KeyValue::new("token", token.clone()), kv]);
            })
        })
        .build();
    Ok(gauge)
}
