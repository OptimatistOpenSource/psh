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
mod raw;

use crate::convert::Wrap;
use crate::profiling::perf::counter::*;
use crate::PerfCtx;
use perf_event_rs::config::{Cpu as RawCpu, Process as RawProcess};
use perf_event_rs::counting::{Config as RawConfig, Counter};
use wasmtime::component::Resource;

impl HostCounter for PerfCtx {
    fn new(
        &mut self,
        process: Process,
        cpu: Cpu,
        cfg: Config,
    ) -> wasmtime::Result<Result<Resource<Counter>, String>> {
        let create_counter = || -> Result<_, String> {
            let process = Wrap::<RawProcess>::from(&process).into_inner();
            let cpu = Wrap::<RawCpu>::from(&cpu).into_inner();
            let mut cfg = Wrap::<RawConfig>::try_from(&cfg)
                .map_err(|err| err.to_string())?
                .into_inner();
            raw::counter_new(&process, &cpu, &mut cfg).map_err(|err| err.to_string())
        };
        Ok(match create_counter() {
            Ok(counter) => Ok(self.table.push(counter)?),
            Err(err) => Err(err),
        })
    }

    fn enable(&mut self, self_: Resource<Counter>) -> wasmtime::Result<Result<(), String>> {
        let counter: &Counter = self.table.get(&self_)?;
        Ok(raw::counter_enable(counter).map_err(|err| err.to_string()))
    }

    fn disable(&mut self, self_: Resource<Counter>) -> wasmtime::Result<Result<(), String>> {
        let counter: &Counter = self.table.get(&self_)?;
        Ok(raw::counter_disable(counter).map_err(|err| err.to_string()))
    }

    fn reset(&mut self, self_: Resource<Counter>) -> wasmtime::Result<Result<(), String>> {
        let counter: &Counter = self.table.get(&self_)?;
        Ok(raw::counter_reset(counter).map_err(|err| err.to_string()))
    }

    fn stat(&mut self, self_: Resource<Counter>) -> wasmtime::Result<Result<CounterStat, String>> {
        let counter: &mut Counter = self.table.get_mut(&self_)?;
        let stat = || -> Result<_, String> {
            let stat = raw::counter_stat(counter).map_err(|err| err.to_string())?;
            Ok(Wrap::<CounterStat>::from(&stat).into_inner())
        }();
        Ok(stat)
    }

    fn drop(&mut self, rep: Resource<Counter>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
