pub mod config;
pub mod gauges;

use std::time::Duration;

use anyhow::Result;
use opentelemetry::{metrics::MeterProvider, KeyValue};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::{
    metrics::{
        reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
        SdkMeterProvider,
    },
    runtime, Resource,
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

pub async fn otlp_tasks(
    instance_id: Option<String>,
    export_config: ExportConfig,
    token: String,
) -> anyhow::Result<()> {
    let provider = meter_provider(export_config, token)?;
    let meter = provider.meter("SystemProfile");
    let interval = Duration::from_secs(1);
    let _ = gauges::memory::start(instance_id.clone(), meter.clone(), interval)?;
    let _ = gauges::network::start(meter.clone(), interval)?;
    let _ = gauges::disk::start(meter.clone(), interval)?;
    let _ = gauges::interrupt::start(meter.clone(), interval)?;
    loop {
        tokio::time::sleep(interval).await;
    }
}
