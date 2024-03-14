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
use wasmtime::{component::*, Config, Engine, Store};
use wasmtime_wasi::preview2::{command, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!();

pub struct PshWasiConfigBuilder {
    wasi_ctx: WasiCtx,

    component_path: String,
    memory_ops: bool,
    system_ops: bool,
    cpu_ops: bool,
    interrupts_ops: bool,
    built: bool,
}

#[allow(dead_code)]
pub struct PshWasiConfig {
    state: State,

    component_path: String,
    memory_ops: bool,
    system_ops: bool,
    cpu_ops: bool,
    interrupts_ops: bool,
}

#[allow(dead_code)]
impl PshWasiConfigBuilder {
    pub fn new(wasi_ctx: WasiCtx) -> Self {
        Self {
            wasi_ctx,
            component_path: "".to_string(),
            memory_ops: false,
            system_ops: false,
            cpu_ops: false,
            interrupts_ops: false,
            built: false,
        }
    }

    pub fn set_component_path(&mut self, path: &str) -> &mut Self {
        self.component_path = path.to_string();
        self
    }

    pub fn enable_memory_ops(&mut self) -> &mut Self {
        self.memory_ops = true;
        self
    }

    pub fn enable_system_ops(&mut self) -> &mut Self {
        self.system_ops = true;
        self
    }

    pub fn enable_cpu_ops(&mut self) -> &mut Self {
        self.cpu_ops = true;
        self
    }

    pub fn enable_interrupts_ops(&mut self) -> &mut Self {
        self.interrupts_ops = true;
        self
    }

    pub fn build(&mut self) -> PshWasiConfig {
        assert!(!self.built);

        let Self {
            wasi_ctx,
            component_path: componenet_path,
            memory_ops,
            system_ops,
            cpu_ops,
            interrupts_ops,
            built: _,
        } = mem::replace(self, Self::new(WasiCtxBuilder::new().build()));
        self.built = true;

        let state = State::new(wasi_ctx);

        PshWasiConfig {
            state,
            component_path: componenet_path,
            memory_ops,
            system_ops,
            cpu_ops,
            interrupts_ops,
        }
    }
}

pub struct State {
    wasi_table: ResourceTable,
    wasi_ctx: WasiCtx,
    name: String,
}

impl State {
    fn new(wasi_ctx: WasiCtx) -> Self {
        let wasi_table = ResourceTable::new();
        Self {
            wasi_table,
            wasi_ctx,
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

    if psh_wasi_config.memory_ops {
        psh::profiling::memory::add_to_linker(&mut linker, |state: &mut State| state)?;
    }

    if psh_wasi_config.system_ops {
        psh::profiling::system::add_to_linker(&mut linker, |state: &mut State| state)?;
    }

    if psh_wasi_config.cpu_ops {
        psh::profiling::cpu::add_to_linker(&mut linker, |state: &mut ServerWasiView| state)?;
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
