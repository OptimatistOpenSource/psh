pub mod config;
pub mod gauges;

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
use psh_system::interrupt::InterruptHandle;
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
    let _ = gauges::memory::start(meter.clone(), interval)?;
    let _ = gauges::network::start(meter.clone(), interval)?;
    let _ = gauges::disk::start(meter.clone(), interval)?;
    let _ = otlp_interrupt(meter.clone(), interval)?;
    loop {
        tokio::time::sleep(interval).await;
    }
}
