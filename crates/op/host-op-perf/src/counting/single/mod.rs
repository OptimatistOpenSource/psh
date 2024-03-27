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
        let mut f = || -> anyhow::Result<_> {
            let process = Wrap::<RawProcess>::from(&process).into_inner();
            let cpu = Wrap::<RawCpu>::from(&cpu).into_inner();
            let mut cfg = Wrap::<RawConfig>::try_from(&cfg)?.into_inner();
            let counter = raw::counter_new(&process, &cpu, &mut cfg)?;
            let handle = self.table.push(counter)?;
            Ok(handle)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn enable(&mut self, self_: Resource<Counter>) -> wasmtime::Result<Result<(), String>> {
        let f = || -> anyhow::Result<_> {
            let counter: &Counter = self.table.get(&self_)?;
            raw::counter_enable(counter)?;
            Ok(())
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn disable(&mut self, self_: Resource<Counter>) -> wasmtime::Result<Result<(), String>> {
        let f = || -> anyhow::Result<_> {
            let counter: &Counter = self.table.get(&self_)?;
            raw::counter_disable(counter)?;
            Ok(())
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn reset(&mut self, self_: Resource<Counter>) -> wasmtime::Result<Result<(), String>> {
        let f = || -> anyhow::Result<_> {
            let counter: &Counter = self.table.get(&self_)?;
            raw::counter_reset(counter)?;
            Ok(())
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn stat(&mut self, self_: Resource<Counter>) -> wasmtime::Result<Result<CounterStat, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter: &mut Counter = self.table.get_mut(&self_)?;
            let stat = raw::counter_stat(counter)?;
            let stat = Wrap::<CounterStat>::from(&stat).into_inner();
            Ok(stat)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn drop(&mut self, rep: Resource<Counter>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
