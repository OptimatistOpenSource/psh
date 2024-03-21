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
mod op;
mod resources;
mod runtime;
mod security;
mod services;

use args::Args;
use clap::Parser;

use wasmtime_wasi::preview2::WasiCtxBuilder;

fn main() {
    let args = Args::parse();

    // detect if we were ran as netdata plugin
    let _netdata_plugin = std::env::var("NETDATA_HOST_PREFIX").is_ok();

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();

    let mut wasi_builder = runtime::PshWasiConfigBuilder::new(wasi_ctx);
    wasi_builder
        .set_component_path(&args.psh_wasm_component)
        .enable_memory_ops()
        .enable_system_ops()
        .enable_cpu_ops()
        .enable_interrupts_ops();
    let wasi_config = wasi_builder.build();
    runtime::run_wasmtime_engine(wasi_config).unwrap();
}
