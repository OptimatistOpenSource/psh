use std::time::Duration;

use opentelemetry::{
    metrics::{Meter, ObservableGauge},
    KeyValue,
};
use psh_system::memory::MemoryHandle;

pub fn start(meter: Meter, interval: Duration) -> anyhow::Result<ObservableGauge<u64>> {
    let memory = MemoryHandle::new();
    let gauge = meter
        .u64_observable_gauge("MemoryStat")
        .with_description("System profile memory statistics.")
        // .with_unit(Unit::new("KiB"))
        .with_callback(move |gauge| {
            let Ok(mem) = memory.stat(Some(interval)) else {
                return;
            };
            gauge.observe(mem.mem_total, &[KeyValue::new("stat", "mem_total")]);
            gauge.observe(mem.mem_free, &[KeyValue::new("stat", "mem_free")]);
            gauge.observe(mem.mem_available, &[KeyValue::new("stat", "mem_available")]);
            gauge.observe(mem.buffers, &[KeyValue::new("stat", "buffers")]);
            gauge.observe(mem.cached, &[KeyValue::new("stat", "cached")]);
            gauge.observe(mem.swap_cached, &[KeyValue::new("stat", "swap_cached")]);
            gauge.observe(mem.active, &[KeyValue::new("stat", "active")]);
            gauge.observe(mem.inactive, &[KeyValue::new("stat", "inactive")]);
            gauge.observe(mem.active_anon, &[KeyValue::new("stat", "active_anon")]);
            gauge.observe(mem.inactive_anon, &[KeyValue::new("stat", "inactive_anon")]);
            gauge.observe(mem.active_file, &[KeyValue::new("stat", "active_file")]);
            gauge.observe(mem.inactive_file, &[KeyValue::new("stat", "inactive_file")]);
            gauge.observe(mem.unevictable, &[KeyValue::new("stat", "unevictable")]);
            gauge.observe(mem.mlocked, &[KeyValue::new("stat", "mlocked")]);
            gauge.observe(mem.swap_total, &[KeyValue::new("stat", "swap_total")]);
            gauge.observe(mem.swap_free, &[KeyValue::new("stat", "swap_free")]);
            gauge.observe(mem.dirty, &[KeyValue::new("stat", "dirty")]);
            gauge.observe(mem.writeback, &[KeyValue::new("stat", "writeback")]);
            gauge.observe(mem.anon_pages, &[KeyValue::new("stat", "anon_pages")]);
            gauge.observe(mem.mapped, &[KeyValue::new("stat", "mapped")]);
            gauge.observe(mem.shmem, &[KeyValue::new("stat", "shmem")]);
            gauge.observe(mem.kreclaimable, &[KeyValue::new("stat", "kreclaimable")]);
            gauge.observe(mem.slab, &[KeyValue::new("stat", "slab")]);
            gauge.observe(mem.sreclaimable, &[KeyValue::new("stat", "sreclaimable")]);
            gauge.observe(mem.sunreclaim, &[KeyValue::new("stat", "sunreclaim")]);
            gauge.observe(mem.kernel_stack, &[KeyValue::new("stat", "kernel_stack")]);
            gauge.observe(mem.page_tables, &[KeyValue::new("stat", "page_tables")]);
            gauge.observe(mem.nfs_unstable, &[KeyValue::new("stat", "nfs_unstable")]);
            gauge.observe(mem.bounce, &[KeyValue::new("stat", "bounce")]);
            gauge.observe(mem.writeback_tmp, &[KeyValue::new("stat", "writeback_tmp")]);
            gauge.observe(mem.commit_limit, &[KeyValue::new("stat", "commit_limit")]);
            gauge.observe(mem.committed_as, &[KeyValue::new("stat", "committed_as")]);
            gauge.observe(mem.vmalloc_total, &[KeyValue::new("stat", "vmalloc_total")]);
            gauge.observe(mem.vmalloc_used, &[KeyValue::new("stat", "vmalloc_used")]);
            gauge.observe(mem.vmalloc_chunk, &[KeyValue::new("stat", "vmalloc_chunk")]);
            gauge.observe(mem.percpu, &[KeyValue::new("stat", "percpu")]);
            gauge.observe(
                mem.cma_total.unwrap_or(0),
                &[KeyValue::new("stat", "cma_total")],
            );
            gauge.observe(
                mem.cma_free.unwrap_or(0),
                &[KeyValue::new("stat", "cma_free")],
            );
            gauge.observe(
                mem.hardware_corrupted.unwrap_or(0),
                &[KeyValue::new("stat", "hardware_corrupted")],
            );
            gauge.observe(
                mem.anon_huge_pages.unwrap_or(0),
                &[KeyValue::new("stat", "anon_huge_pages")],
            );
            gauge.observe(
                mem.shmem_huge_pages.unwrap_or(0),
                &[KeyValue::new("stat", "shmem_huge_pages")],
            );
            gauge.observe(
                mem.shmem_pmd_mapped.unwrap_or(0),
                &[KeyValue::new("stat", "shmem_pmd_mapped")],
            );
            gauge.observe(
                mem.file_huge_pages.unwrap_or(0),
                &[KeyValue::new("stat", "file_huge_pages")],
            );
            gauge.observe(
                mem.file_pmd_mapped.unwrap_or(0),
                &[KeyValue::new("stat", "file_pmd_mapped")],
            );
            gauge.observe(
                mem.huge_pages_total,
                &[KeyValue::new("stat", "huge_pages_total")],
            );
            gauge.observe(
                mem.huge_pages_free,
                &[KeyValue::new("stat", "huge_pages_free")],
            );
            gauge.observe(
                mem.huge_pages_rsvd,
                &[KeyValue::new("stat", "huge_pages_rsvd")],
            );
            gauge.observe(
                mem.huge_pages_surp,
                &[KeyValue::new("stat", "huge_pages_surp")],
            );
            gauge.observe(
                mem.huge_page_size,
                &[KeyValue::new("stat", "huge_page_size")],
            );
            gauge.observe(mem.huge_tlb, &[KeyValue::new("stat", "huge_tlb")]);
            gauge.observe(
                mem.direct_map4k.unwrap_or(0),
                &[KeyValue::new("stat", "direct_map4k")],
            );
            gauge.observe(
                mem.direct_map2_m.unwrap_or(0),
                &[KeyValue::new("stat", "direct_map2_m")],
            );
            gauge.observe(
                mem.direct_map1_g.unwrap_or(0),
                &[KeyValue::new("stat", "direct_map1_g")],
            );
        })
        .try_init()?;
    Ok(gauge)
}
