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
use runtime::PshEngineBuilder;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let component_envs: Vec<(String, String)> = std::env::vars().collect();
    let mut component_args: Vec<String> = vec![args.psh_wasm_component.clone()];
    component_args.extend(args.extra_args);

    let th = std::thread::spawn(|| tokio::runtime::Runtime::new()?.block_on(otlp::otlp_tasks()));

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
