use anyhow::Result;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::{
    metrics::{
        reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
        SdkMeterProvider,
    },
    runtime, Resource,
};

pub(crate) fn meter_provider() -> Result<SdkMeterProvider> {
    let export_config = ExportConfig {
        endpoint: "http://localhost:4317".to_string(),
        timeout: std::time::Duration::from_secs(3),
        protocol: opentelemetry_otlp::Protocol::Grpc,
    };

    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_export_config(export_config);

    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(otlp_exporter)
        .with_resource(Resource::new(vec![KeyValue::new("service.name", "PSH")]))
        .with_period(std::time::Duration::from_secs(1))
        .with_timeout(std::time::Duration::from_secs(10))
        .with_aggregation_selector(DefaultAggregationSelector::new())
        .with_temporality_selector(DefaultTemporalitySelector::new())
        .build()
        .map_err(Into::into)
}
