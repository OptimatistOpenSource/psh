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

pub mod config;
pub mod gauges;

use std::time::Duration;

use anyhow::Result;
use opentelemetry::{metrics::MeterProvider, KeyValue};
use opentelemetry_otlp::{ExportConfig, MetricExporter, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::{
    metrics::{PeriodicReader, SdkMeterProvider},
    runtime, Resource,
};
use tonic::{metadata::MetadataMap, transport::ClientTlsConfig};

pub fn meter_provider(export_config: ExportConfig, token: String) -> Result<SdkMeterProvider> {
    let mut meta = MetadataMap::new();
    meta.insert("authorization", format!("Bearer {}", token).parse()?);
    let otlp_exporter = MetricExporter::builder()
        .with_tonic()
        .with_tls_config(ClientTlsConfig::new().with_native_roots())
        .with_metadata(meta)
        .with_timeout(Duration::from_secs(10))
        .with_export_config(export_config)
        .build()?;
    let reader = PeriodicReader::builder(otlp_exporter, runtime::Tokio)
        .with_interval(Duration::from_secs(1))
        .build();

    let a = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(Resource::new(vec![KeyValue::new("service.name", "PSH")]))
        .build();

    Ok(a)
}

pub async fn otlp_tasks(export_config: ExportConfig, token: String) -> anyhow::Result<()> {
    let provider = meter_provider(export_config, token.clone())?;
    let meter = provider.meter("SystemProfile");
    let interval = Duration::from_secs(1);
    let _ = gauges::memory::start(token.clone(), meter.clone(), interval)?;
    let _ = gauges::network::start(token.clone(), meter.clone(), interval)?;
    let _ = gauges::disk::start(token.clone(), meter.clone(), interval)?;
    let _ = gauges::interrupt::start(token, meter.clone(), interval)?;
    loop {
        tokio::time::sleep(interval).await;
    }
}
