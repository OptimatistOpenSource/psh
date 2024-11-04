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

use anyhow::Context;
use wasmtime::{
    component::{Linker, ResourceTable},
    Config, Engine, Store,
};
use wasmtime_wasi::{StdinStream, StdoutStream, WasiCtxBuilder};

use host_op_perf::PerfCtx;
use host_op_system::SysCtx;

use super::{PshEngine, PshState};

#[allow(dead_code)]
pub struct PshEngineBuilder {
    wasi_ctx_builder: WasiCtxBuilder,
    engine_config: Config,
    use_perf_op: bool,
    use_system_op: bool,
}

#[allow(dead_code)]
impl PshEngineBuilder {
    pub fn new() -> Self {
        let mut engine_config = Config::new();
        engine_config.wasm_component_model(true);
        Self {
            wasi_ctx_builder: WasiCtxBuilder::new(),
            engine_config,
            use_perf_op: false,
            use_system_op: false,
        }
    }

    pub fn build(mut self) -> anyhow::Result<PshEngine> {
        let wasi_ctx = self.wasi_ctx_builder.build();
        let engine = Engine::new(&self.engine_config).context("Failed to create Wasi Engine.")?;
        let state = PshState {
            name: "PSH Wasi Runtime".to_owned(),
            table: ResourceTable::new(),
            wasi_ctx,
            perf_ctx: PerfCtx::new(),
            sys_ctx: SysCtx::default(),
        };
        let store = Store::new(&engine, state);
        let mut linker: Linker<PshState> = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker)
            .context("Failed to link wasi sync module")?;
        if self.use_perf_op {
            host_op_perf::add_to_linker(&mut linker, |state| &mut state.perf_ctx)
                .context("Failed to link perf module")?;
        }
        if self.use_system_op {
            host_op_system::add_to_linker(&mut linker, |state| &mut state.sys_ctx)
                .context("Failed to link system module")?;
        }
        Ok(PshEngine {
            engine,
            store,
            linker,
        })
    }

    pub fn wasi_stdin(mut self, stdin: impl StdinStream + 'static) -> Self {
        self.wasi_ctx_builder.stdin(stdin);
        self
    }

    pub fn wasi_stdout(mut self, stdout: impl StdoutStream + 'static) -> Self {
        self.wasi_ctx_builder.stdout(stdout);
        self
    }

    pub fn wasi_stderr(mut self, stderr: impl StdoutStream + 'static) -> Self {
        self.wasi_ctx_builder.stderr(stderr);
        self
    }

    pub fn wasi_inherit_stdin(mut self) -> Self {
        self.wasi_ctx_builder.inherit_stdin();
        self
    }

    pub fn wasi_inherit_stdout(mut self) -> Self {
        self.wasi_ctx_builder.inherit_stdout();
        self
    }

    pub fn wasi_inherit_stderr(mut self) -> Self {
        self.wasi_ctx_builder.inherit_stderr();
        self
    }

    pub fn wasi_inherit_stdio(mut self) -> Self {
        self.wasi_ctx_builder.inherit_stdio();
        self
    }

    pub fn wasi_envs(mut self, env: &[(impl AsRef<str>, impl AsRef<str>)]) -> Self {
        self.wasi_ctx_builder.envs(env);
        self
    }

    pub fn wasi_env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.wasi_ctx_builder.env(k, v);
        self
    }
    pub fn wasi_args(mut self, args: &[impl AsRef<str>]) -> Self {
        self.wasi_ctx_builder.args(args);
        self
    }

    pub fn wasi_arg(mut self, arg: impl AsRef<str>) -> Self {
        self.wasi_ctx_builder.arg(arg);
        self
    }

    pub fn wasi_inherit_network(mut self) -> Self {
        self.wasi_ctx_builder.inherit_network();
        self
    }

    pub fn wasi_allow_ip_name_lookup(mut self, enable: bool) -> Self {
        self.wasi_ctx_builder.allow_ip_name_lookup(enable);
        self
    }

    pub fn wasi_allow_udp(mut self, enable: bool) -> Self {
        self.wasi_ctx_builder.allow_udp(enable);
        self
    }

    pub fn wasi_allow_tcp(mut self, enable: bool) -> Self {
        self.wasi_ctx_builder.allow_tcp(enable);
        self
    }

    pub fn allow_perf_op(mut self, enable: bool) -> Self {
        self.use_perf_op = enable;
        self
    }

    pub fn allow_system_op(mut self, enable: bool) -> Self {
        self.use_system_op = enable;
        self
    }
}
