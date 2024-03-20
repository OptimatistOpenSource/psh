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

use std::mem;

use __internal::anyhow::{self, Context};
use host_op_perf::PerfCtx;
use host_op_system::SysCtx;
use wasmtime::{component::*, Config, Engine, Store};
use wasmtime_wasi::preview2::{command, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!();

pub struct PshWasiConfigBuilder {
    wasi_ctx: WasiCtx,
    perf_ctx: Option<PerfCtx>,
    sys_ctx: Option<SysCtx>,

    component_path: String,
    built: bool,
}

#[allow(dead_code)]
pub struct PshWasiConfig {
    state: State,

    component_path: String,
}

#[allow(dead_code)]
impl PshWasiConfigBuilder {
    pub fn new(wasi_ctx: WasiCtx) -> Self {
        Self {
            wasi_ctx,
            perf_ctx: None,
            sys_ctx: None,
            component_path: "".to_string(),
            built: false,
        }
    }

    pub fn set_component_path(&mut self, path: &str) -> &mut Self {
        self.component_path = path.to_string();
        self
    }

    pub fn enable_system_ops(&mut self, ctx: SysCtx) -> &mut Self {
        self.sys_ctx = Some(ctx);
        self
    }

    pub fn enable_perf_ops(&mut self, ctx: PerfCtx) -> &mut Self {
        self.perf_ctx = Some(ctx);
        self
    }

    pub fn build(&mut self) -> PshWasiConfig {
        assert!(!self.built);

        let Self {
            wasi_ctx,
            perf_ctx,
            component_path,
            sys_ctx,
            built: _,
        } = mem::replace(self, Self::new(WasiCtxBuilder::new().build()));
        self.built = true;

        let state = State::new(wasi_ctx, perf_ctx, sys_ctx);

        PshWasiConfig {
            state,
            component_path,
        }
    }
}

pub struct State {
    wasi_table: ResourceTable,
    wasi_ctx: WasiCtx,
    perf_ctx: Option<PerfCtx>,
    sys_ctx: Option<SysCtx>,
    name: String,
}

impl State {
    fn new(wasi_ctx: WasiCtx, perf_ctx: Option<PerfCtx>, sys_ctx: Option<SysCtx>) -> Self {
        let wasi_table = ResourceTable::new();
        Self {
            wasi_table,
            wasi_ctx,
            perf_ctx,
            sys_ctx,
            name: "PSH Wasi Runtime".to_string(),
        }
    }
}

impl WasiView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.wasi_table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl BindingsImports for State {
    fn name(&mut self) -> wasmtime::Result<String> {
        Ok(self.name.clone())
    }
}

pub fn run_wasmtime_engine(psh_wasi_config: PshWasiConfig) -> wasmtime::Result<()> {
    let component_file = psh_wasi_config.component_path;

    // Configure an `Engine` and compile the `Component` that is being run for
    // the application.
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, component_file)?;

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated `add_to_linker`
    // method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the `HelloWorldImports`
    // trait. In this case the `T`, `MyState`, is stored directly in the
    // structure so no projection is necessary here.
    let mut linker = Linker::new(&engine);
    command::sync::add_to_linker(&mut linker).context("Failed to link command world")?;

    Bindings::add_root_to_linker(&mut linker, |state: &mut State| state)?;

    if psh_wasi_config.state.sys_ctx.is_some() {
        host_op_system::add_to_linker(&mut linker, |state: &mut State| {
            state.sys_ctx.as_mut().unwrap()
        })?;
    }

    if psh_wasi_config.state.perf_ctx.is_some() {
        host_op_perf::add_to_linker(&mut linker, |state: &mut State| {
            state.perf_ctx.as_mut().unwrap()
        })?;
    }

    // As with the core wasm API of Wasmtime instantiation occurs within a
    // `Store`. The bindings structure contains an `instantiate` method which
    // takes the store, component, and linker. This returns the `bindings`
    // structure which is an instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let mut store = Store::new(&engine, psh_wasi_config.state);
    let (cmd, _instance) = command::sync::Command::instantiate(&mut store, &component, &linker)?;

    let result = cmd.wasi_cli_run().call_run(&mut store)?;
    result.map_err(|()| anyhow::anyhow!("run returned an error"))
}
