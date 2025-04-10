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
    KeyValue,
    metrics::{Meter, MeterProvider},
};
use opentelemetry_otlp::{ExportConfig, MetricExporter, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::{
    Resource,
    metrics::{PeriodicReader, SdkMeterProvider},
};
use tinyufo::TinyUfo;
use tonic::{metadata::MetadataMap, transport::ClientTlsConfig};

// TODO: Make size configurable
static NET_DEV_SPEED: LazyLock<TinyUfo<String, Option<u32>>> =
    LazyLock::new(|| TinyUfo::new_compact(15, 15));

#[derive(Debug, Clone)]
pub struct Otlp {
    host: String,
    interval: Duration,
    meter: Meter,
    // NOTE: the field avoid provider early drop see: <https://github.com/open-telemetry/opentelemetry-rust/issues/1661>
    _provider: SdkMeterProvider,
}

impl Otlp {
    pub fn new(token: String, interval: Duration, export_config: ExportConfig) -> Result<Self> {
        let provider = Self::meter_provider(export_config, &token, interval)?;
        let host = nix::unistd::gethostname()
            .ok()
            .map(|v| v.to_string_lossy().to_string())
            .unwrap_or_else(|| token.clone());
        let meter = provider.meter("SystemProfile");
        Ok(Self {
            host,
            interval,
            meter,
            _provider: provider,
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

    pub async fn otlp_tasks(&self) {
        let interval = self.interval;

        self.mem_gauges();
        self.net_gauges();
        self.disk_gagues();
        self.irq_gauges();
        self.cpu_gauges();
        self.rps_gauges();
        self.vmstat_gauges();
        self.gpu_gauges();

        loop {
            tokio::time::sleep(interval).await;
        }
    }

    fn meter_provider(
        export_config: ExportConfig,
        token: &str,
        interval: Duration,
    ) -> Result<SdkMeterProvider> {
        let mut meta = MetadataMap::new();
        meta.insert("authorization", format!("Bearer {}", token).parse()?);
        let otlp_exporter = MetricExporter::builder()
            .with_tonic()
            .with_tls_config(ClientTlsConfig::new().with_native_roots())
            .with_metadata(meta)
            .with_timeout(Duration::from_secs(10))
            .with_export_config(export_config)
            .build()?;
        let reader = PeriodicReader::builder(otlp_exporter)
            .with_interval(interval)
            .build();
        let resource = Resource::builder()
            .with_attribute(KeyValue::new("service.name", "PSH"))
            .build();
        let a = SdkMeterProvider::builder()
            .with_reader(reader)
            .with_resource(resource)
            .build();

        Ok(a)
    }
}
