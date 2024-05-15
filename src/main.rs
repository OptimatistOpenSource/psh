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
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

mod args;
mod infra;
mod otlp;
mod resources;
mod runtime;
mod security;
mod services;
mod utils;

use anyhow::Context;
use clap::Parser;

use args::Args;
use opentelemetry::{metrics::MeterProvider, KeyValue};
use runtime::PshEngineBuilder;

async fn otlp() -> anyhow::Result<()> {
    let system = psh_system::System::default();

    let provider = otlp::meter_provider()?;
    let meter = provider.meter("SystemProfile");
    meter
        .u64_observable_gauge("SystemProfile")
        .with_description("System profile statistics.")
        .with_callback(move |gauge| {
            if let Ok(mem) = system.memory_stat_handle.get() {
                gauge.observe(mem.mem_free, &[KeyValue::new("system.mem.stat", "free")]);
                gauge.observe(
                    mem.mem_available,
                    &[KeyValue::new("system.mem.stat", "available")],
                );
                gauge.observe(mem.cached, &[KeyValue::new("system.mem.stat", "cached")]);
                gauge.observe(
                    mem.swap_free,
                    &[KeyValue::new("system.mem.stat", "swap_free")],
                );
                gauge.observe(mem.dirty, &[KeyValue::new("system.mem.stat", "dirty")]);
                gauge.observe(mem.mapped, &[KeyValue::new("system.mem.stat", "mapped")]);
                gauge.observe(
                    mem.huge_pages_free,
                    &[KeyValue::new("system.mem.stat", "huge_pages_free")],
                );
                gauge.observe(mem.bounce, &[KeyValue::new("system.mem.stat", "bounce")]);
            }
        })
        .try_init()?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let component_envs: Vec<(String, String)> = std::env::vars().collect();
    let mut component_args: Vec<String> = vec![args.psh_wasm_component.clone()];
    component_args.extend(args.extra_args);

    let th = std::thread::spawn(|| tokio::runtime::Runtime::new()?.block_on(otlp()));

    let mut engine = PshEngineBuilder::new()
        .wasi_inherit_stdio()
        .wasi_envs(&component_envs)
        .wasi_args(&component_args)
        .allow_perf_op(true)
        .allow_system_op(true)
        .build()
        .context("Failed to build PshEngine.")?;

    engine.run(&args.psh_wasm_component)?;

    let _ = th.join();

    Ok(())
}
