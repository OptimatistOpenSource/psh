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
use crate::profiling::perf::counter_group::*;
use crate::{FixedCounterGroup, PerfCtx};
use perf_event_rs::config::{Cpu as RawCpu, Process as RawProcess};
use perf_event_rs::counting::{Config as RawConfig, CounterGroup, CounterGuard};
use wasmtime::component::Resource;

impl HostCounterGroup for PerfCtx {
    fn new(
        &mut self,
        process: Process,
        cpu: Cpu,
    ) -> wasmtime::Result<Result<Resource<CounterGroup>, String>> {
        let mut f = || -> anyhow::Result<_> {
            let process = Wrap::<RawProcess>::from(&process).into_inner();
            let cpu = Wrap::<RawCpu>::from(&cpu).into_inner();
            let counter_group = raw::counter_group_new(&process, &cpu)?;
            let handle = self.table.push(counter_group)?;
            Ok(handle)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn add_member(
        &mut self,
        self_: Resource<CounterGroup>,
        cfg: Config,
    ) -> wasmtime::Result<Result<Resource<CounterGuard>, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter_group: &mut CounterGroup = self.table.get_mut(&self_)?;
            let mut cfg = Wrap::<RawConfig>::try_from(&cfg)?.into_inner();
            let guard = raw::counter_group_add_member(counter_group, &mut cfg)?;
            let handle = self.table.push(guard)?;
            Ok(handle)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn enable(
        &mut self,
        counter_group: Resource<CounterGroup>,
    ) -> wasmtime::Result<Result<Resource<FixedCounterGroup>, String>> {
        let f = || -> anyhow::Result<_> {
            let counter_group: CounterGroup = self.table.delete(counter_group)?;
            let fixed_counter_group = raw::counter_group_enable(counter_group)?;
            let handle = self.table.push(fixed_counter_group)?;
            Ok(handle)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn stat(
        &mut self,
        self_: Resource<CounterGroup>,
    ) -> wasmtime::Result<Result<CounterGroupStat, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter_group: &mut CounterGroup = self.table.get_mut(&self_)?;
            let stat = raw::counter_group_stat(counter_group)?;
            let stat = Wrap::<CounterGroupStat>::from(&stat).into_inner();
            Ok(stat)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn into_fixed(
        &mut self,
        counter_group: Resource<CounterGroup>,
    ) -> wasmtime::Result<Result<Resource<FixedCounterGroup>, String>> {
        let f = || -> anyhow::Result<_> {
            let counter_group: CounterGroup = self.table.delete(counter_group)?;
            let fixed_counter_group = raw::counter_group_into_fixed(counter_group)?;
            let handle = self.table.push(fixed_counter_group)?;
            Ok(handle)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn drop(&mut self, rep: Resource<CounterGroup>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl HostFixedCounterGroup for PerfCtx {
    fn enable(
        &mut self,
        self_: Resource<FixedCounterGroup>,
    ) -> wasmtime::Result<Result<(), String>> {
        let f = || -> anyhow::Result<_> {
            let fixed_counter_group: &FixedCounterGroup = self.table.get(&self_)?;
            raw::fixed_counter_group_enable(fixed_counter_group)?;
            Ok(())
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn disable(
        &mut self,
        self_: Resource<FixedCounterGroup>,
    ) -> wasmtime::Result<Result<(), String>> {
        let f = || -> anyhow::Result<_> {
            let fixed_counter_group: &FixedCounterGroup = self.table.get(&self_)?;
            raw::fixed_counter_group_disable(fixed_counter_group)?;
            Ok(())
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn reset(
        &mut self,
        self_: Resource<FixedCounterGroup>,
    ) -> wasmtime::Result<Result<(), String>> {
        let f = || -> anyhow::Result<_> {
            let fixed_counter_group: &FixedCounterGroup = self.table.get(&self_)?;
            raw::fixed_counter_group_reset(fixed_counter_group)?;
            Ok(())
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn stat(
        &mut self,
        self_: Resource<FixedCounterGroup>,
    ) -> wasmtime::Result<Result<CounterGroupStat, String>> {
        let mut f = || -> anyhow::Result<_> {
            let fixed_counter_group: &mut FixedCounterGroup = self.table.get_mut(&self_)?;
            let stat = raw::fixed_counter_group_stat(fixed_counter_group)?;
            let stat = Wrap::<CounterGroupStat>::from(&stat).into_inner();
            Ok(stat)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn drop(&mut self, rep: Resource<FixedCounterGroup>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl HostCounterGuard for PerfCtx {
    fn event_id(&mut self, self_: Resource<CounterGuard>) -> wasmtime::Result<u64> {
        let counter_guard: &CounterGuard = self.table.get(&self_)?;
        let event_id = raw::counter_guard_event_id(counter_guard);
        Ok(event_id)
    }
    fn stat(
        &mut self,
        self_: Resource<CounterGuard>,
    ) -> wasmtime::Result<Result<CounterStat, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter_guard: &mut CounterGuard = self.table.get_mut(&self_)?;
            let stat = raw::counter_guard_stat(counter_guard)?;
            let stat = Wrap::<CounterStat>::from(&stat).into_inner();
            Ok(stat)
        };
        Ok(f().map_err(|e| e.to_string()))
    }
    fn drop(&mut self, rep: Resource<CounterGuard>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
