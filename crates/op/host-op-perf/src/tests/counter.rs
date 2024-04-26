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

use crate::PerfCtx;
use std::path::Path;
use wasmtime::component::{Component, Instance, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{command, WasiCtx, WasiCtxBuilder, WasiView};

pub struct State {
    pub perf_ctx: PerfCtx,
    pub wasi_ctx: WasiCtx,
    pub table: ResourceTable,
}

impl WasiView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

#[test]
fn test_counter() {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(
        &engine,
        State {
            perf_ctx: PerfCtx::new(),
            wasi_ctx: WasiCtxBuilder::new().inherit_stdout().build(),
            table: ResourceTable::new(),
        },
    );
    let mut linker: Linker<State> = Linker::new(&engine);
    crate::add_to_linker(&mut linker, |t| &mut t.perf_ctx).unwrap();
    command::sync::add_to_linker(&mut linker).unwrap();

    let path = "../../../test_resources/profiling/test-perf-counter";
    let wasm_path = format!("{}/target/wasm32-wasi/debug/test-perf-counter.wasm", path);
    assert!(
        Path::new(&wasm_path).exists(),
        "wasm file doesn't exist: {}",
        wasm_path
    );
    let component = Component::from_file(&engine, wasm_path).unwrap();

    let (cmd, _): (_, Instance) =
        command::sync::Command::instantiate(&mut store, &component, &linker).unwrap();

    cmd.wasi_cli_run().call_run(&mut store).unwrap().unwrap();
}
