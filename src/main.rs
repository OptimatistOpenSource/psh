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
mod resources;
mod runtime;
mod security;
mod services;
mod utils;

use args::Args;
use clap::Parser;
use wasmtime_wasi::preview2::WasiCtxBuilder;

use host_op_perf::PerfCtx;
use host_op_system::SysCtx;

fn main() {
    let args = Args::parse();
    let component_envs: Vec<(String, String)> = std::env::vars().collect();
    let mut component_args: Vec<String> = vec![args.psh_wasm_component.clone()];
    component_args.extend(args.extra_args);

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .envs(&component_envs)
        .args(&component_args)
        .build();
    let sys_ctx = SysCtx {};
    let perf_ctx = PerfCtx::new();

    let mut wasi_builder = runtime::PshWasiConfigBuilder::new(wasi_ctx);
    wasi_builder
        .set_component_path(&args.psh_wasm_component)
        .enable_perf_ops(perf_ctx)
        .enable_system_ops(sys_ctx);
    let wasi_config = wasi_builder.build();
    runtime::run_wasmtime_engine(wasi_config).unwrap();
}
