use std::{borrow::Cow, time::Duration};

use opentelemetry::{
    metrics::{Meter, ObservableGauge},
    KeyValue,
};
use psh_system::disk::DiskHandle;

pub fn start(meter: Meter, interval: Duration) -> anyhow::Result<ObservableGauge<u64>> {
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
