pub mod config;

use std::{borrow::Cow, time::Duration};

use anyhow::Result;
use opentelemetry::{
    metrics::{Meter, MeterProvider, ObservableGauge},
    KeyValue,
};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::{
    metrics::{
        reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
        SdkMeterProvider,
    },
    runtime, Resource,
};
use psh_system::{
    disk::DiskHandle, interrupt::InterruptHandle, memory::MemoryHandle, network::NetworkHandle,
};
use tonic::{metadata::MetadataMap, transport::ClientTlsConfig};

pub fn meter_provider(export_config: ExportConfig, token: String) -> Result<SdkMeterProvider> {
    let mut meta = MetadataMap::new();
    meta.insert("authorization", format!("Bearer {}", token).parse()?);
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_tls_config(ClientTlsConfig::new().with_native_roots())
        .with_metadata(meta)
        .with_export_config(export_config);

    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(otlp_exporter)
        .with_resource(Resource::new(vec![KeyValue::new("service.name", "PSH")]))
        .with_period(Duration::from_secs(1))
        .with_timeout(Duration::from_secs(10))
        .with_aggregation_selector(DefaultAggregationSelector::new())
        .with_temporality_selector(DefaultTemporalitySelector::new())
        .build()
        .map_err(Into::into)
}

pub fn otlp_memories(meter: Meter, interval: Duration) -> anyhow::Result<ObservableGauge<u64>> {
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

pub fn otlp_networks(meter: Meter, interval: Duration) -> anyhow::Result<ObservableGauge<u64>> {
    let network = NetworkHandle::new();
    let gauge = meter
        .u64_observable_gauge("NetworkStat")
        .with_description("System profile network statistics.")
        // .with_unit(Unit::new("KiB"))
        .with_callback(move |gauge| {
            let Ok(stat) = network.stat(Some(interval)) else {
                return;
            };
            for (dev, status) in stat {
                // TODO
                let dev = Cow::from(dev);
                gauge.observe(
                    status.recv_bytes,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_bytes"),
                    ],
                );
                gauge.observe(
                    status.recv_packets,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_packets"),
                    ],
                );
                gauge.observe(
                    status.recv_errs,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_errs"),
                    ],
                );
                gauge.observe(
                    status.recv_drop,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_drop"),
                    ],
                );
                gauge.observe(
                    status.recv_fifo,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_fifo"),
                    ],
                );
                gauge.observe(
                    status.recv_frame,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_frame"),
                    ],
                );
                gauge.observe(
                    status.recv_compressed,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_compressed"),
                    ],
                );
                gauge.observe(
                    status.recv_multicast,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_multicast"),
                    ],
                );
                gauge.observe(
                    status.sent_bytes,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_bytes"),
                    ],
                );
                gauge.observe(
                    status.sent_packets,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_packets"),
                    ],
                );
                gauge.observe(
                    status.sent_errs,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_errs"),
                    ],
                );
                gauge.observe(
                    status.sent_drop,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_drop"),
                    ],
                );
                gauge.observe(
                    status.sent_fifo,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_fifo"),
                    ],
                );
                gauge.observe(
                    status.sent_colls,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_colls"),
                    ],
                );
                gauge.observe(
                    status.sent_carrier,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_carrier"),
                    ],
                );
                gauge.observe(
                    status.sent_compressed,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_compressed"),
                    ],
                );
            }
        })
        .try_init()?;
    Ok(gauge)
}

pub fn otlp_disks(meter: Meter, interval: Duration) -> anyhow::Result<ObservableGauge<u64>> {
    let disk = DiskHandle::new();
    let gauge = meter
        .u64_observable_gauge("DiskStat")
        .with_description("System profile disk statistics.")
        // .with_unit(Unit::new("KiB"))
        .with_callback(move |gauge| {
            let Ok(disks) = disk.stat(Some(interval)) else {
                return;
            };
            for stat in disks {
                // TODO
                let name = Cow::from(stat.name);
                gauge.observe(
                    stat.reads,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "reads"),
                    ],
                );
                gauge.observe(
                    stat.merged,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "merged"),
                    ],
                );
                gauge.observe(
                    stat.sectors_read,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "sectors_read"),
                    ],
                );
                gauge.observe(
                    stat.time_reading,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "time_reading"),
                    ],
                );
                gauge.observe(
                    stat.writes,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "writes"),
                    ],
                );
                gauge.observe(
                    stat.writes_merged,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "writes_merged"),
                    ],
                );
                gauge.observe(
                    stat.sectors_written,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "sectors_written"),
                    ],
                );
                gauge.observe(
                    stat.time_writing,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "time_writing"),
                    ],
                );
                gauge.observe(
                    stat.in_progress,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "in_progress"),
                    ],
                );
                gauge.observe(
                    stat.time_in_progress,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "time_in_progress"),
                    ],
                );
                gauge.observe(
                    stat.weighted_time_in_progress,
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "weighted_time_in_progress"),
                    ],
                );
                gauge.observe(
                    stat.discards.unwrap_or(0),
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "discards"),
                    ],
                );
                gauge.observe(
                    stat.discards_merged.unwrap_or(0),
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "discards_merged"),
                    ],
                );
                gauge.observe(
                    stat.sectors_discarded.unwrap_or(0),
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "sectors_discarded"),
                    ],
                );
                gauge.observe(
                    stat.time_discarding.unwrap_or(0),
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "time_discarding"),
                    ],
                );
                gauge.observe(
                    stat.flushes.unwrap_or(0),
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "flushes"),
                    ],
                );
                gauge.observe(
                    stat.time_flushing.unwrap_or(0),
                    &[
                        KeyValue::new("disk", name.clone()),
                        KeyValue::new("stat", "time_flushing"),
                    ],
                );
            }
        })
        .try_init()?;
    Ok(gauge)
}

pub fn otlp_interrupt(meter: Meter, interval: Duration) -> anyhow::Result<ObservableGauge<u64>> {
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
                // TODO
                let desc = Cow::from(int.description);
                for (cpu, &cnt) in int.cpu_counts.iter().enumerate() {
                    gauge.observe(
                        cnt,
                        &[
                            KeyValue::new("desc", desc.clone()),
                            KeyValue::new("cpu", cpu as i64),
                        ],
                    )
                }
            }
        })
        .try_init()?;
    Ok(gauge)
}

pub async fn otlp_tasks(export_config: ExportConfig, token: String) -> anyhow::Result<()> {
    let provider = meter_provider(export_config, token)?;
    let meter = provider.meter("SystemProfile");
    let interval = Duration::from_secs(1);
    let _ = otlp_memories(meter.clone(), interval)?;
    let _ = otlp_networks(meter.clone(), interval)?;
    let _ = otlp_disks(meter.clone(), interval)?;
    let _ = otlp_interrupt(meter.clone(), interval)?;
    loop {
        tokio::time::sleep(interval).await;
    }
}
