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

pub mod gauges;

use std::{sync::LazyLock, time::Duration};

use anyhow::Result;
use opentelemetry::{
    metrics::{Meter, MeterProvider},
    KeyValue,
};
use opentelemetry_otlp::{ExportConfig, MetricExporter, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::{
    metrics::{PeriodicReader, SdkMeterProvider},
    runtime, Resource,
};
use tinyufo::TinyUfo;
use tonic::{metadata::MetadataMap, transport::ClientTlsConfig};

// TODO: Make size configurable
static NET_DEV_SPEED: LazyLock<TinyUfo<String, Option<u32>>> =
    LazyLock::new(|| TinyUfo::new_compact(15, 15));

#[derive(Debug, Clone)]
pub struct Otlp {
    token: String,
    interval: Duration,
    meter: Meter,
}

impl Otlp {
    pub fn new(token: String, interval: Duration, export_config: ExportConfig) -> Result<Self> {
        let meter = meter_provider(export_config, token.clone())?.meter("SystemProfile");
        Ok(Self {
            token,
            interval,
            meter,
        })
    }

    pub fn net_dev_speed(name: &String) -> Option<u32> {
        if let Some(speed) = NET_DEV_SPEED.get(name) {
            return speed;
        }

        let speed = psh_system::network::dev_speed(name);
        NET_DEV_SPEED.put(name.clone(), speed, 1);

        speed
    }

    pub async fn otlp_tasks(&self) -> anyhow::Result<()> {
        let interval = self.interval;

        if let Err(e) = self.mem_gauges() {
            tracing::error!("Otlp memory: {e}")
        }
        if let Err(e) = self.net_gauges() {
            tracing::error!("Otlp network: {e}")
        }
        if let Err(e) = self.disk_gagues() {
            tracing::error!("Otlp disk: {e}")
        }
        if let Err(e) = self.irq_gauges() {
            tracing::error!("Otlp interrupt: {e}")
        }
        if let Err(e) = self.cpu_gauges() {
            tracing::error!("Otlp cpu: {e}")
        }
        if let Err(e) = self.rps_gauges() {
            tracing::error!("Otlp rps: {e}")
        }

        loop {
            tokio::time::sleep(interval).await;
        }
    }
}

fn meter_provider(export_config: ExportConfig, token: String) -> Result<SdkMeterProvider> {
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
