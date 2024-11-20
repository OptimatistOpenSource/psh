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

use std::{thread, time::Duration};

use anyhow::Context;
use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};
use wasmtime_wasi::bindings::sync::Command;

use super::PshState;

pub struct PshEngine {
    pub(crate) engine: Engine,
    pub(crate) store: Store<PshState>,
    pub(crate) linker: Linker<PshState>,
}

impl PshEngine {
    pub fn run(mut self, binary: &[u8], time_slice: u64) -> anyhow::Result<()> {
        let component =
            Component::from_binary(&self.engine, binary).context("Failed to load component!")?;
        let (cmd, _inst) = Command::instantiate(&mut self.store, &component, &self.linker)
            .context("Failed to instantiate Wasi Command!")?;
        self.store.set_epoch_deadline(1);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(time_slice as _));
            self.engine.increment_epoch();
        });
        let _ = cmd
            .wasi_cli_run()
            .call_run(&mut self.store)
            .context("Failed to run component")?;
        Ok(())
    }
}
