mod raw;

use crate::convert::Wrap;
use crate::profiling::perf::counter_group::*;
use crate::{FixedCounterGroup, PerfView};
use perf_event_rs::counting::{CounterGroup, CounterGuard};
use wasmtime::component::Resource;

impl<T> HostCounterGroup for T
where
    T: PerfView,
{
    fn new(
        &mut self,
        process: Process,
        cpu: Cpu,
    ) -> wasmtime::Result<Result<Resource<CounterGroup>, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter_group = raw::counter_group_new(&process, &cpu)?;
            let handle = PerfView::table_mut(self).push(counter_group)?;
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
            let counter_group: &mut CounterGroup = PerfView::table_mut(self).get_mut(&self_)?;
            let guard = raw::counter_group_add_member(counter_group, &cfg)?;
            let handle = PerfView::table_mut(self).push(guard)?;
            Ok(handle)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn enable(
        &mut self,
        counter_group: Resource<CounterGroup>,
    ) -> wasmtime::Result<Result<Resource<FixedCounterGroup>, String>> {
        let f = || -> anyhow::Result<_> {
            let counter_group: CounterGroup = PerfView::table_mut(self).delete(counter_group)?;
            let fixed_counter_group = raw::counter_group_enable(counter_group)?;
            let handle = PerfView::table_mut(self).push(fixed_counter_group)?;
            Ok(handle)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn stat(
        &mut self,
        self_: Resource<CounterGroup>,
    ) -> wasmtime::Result<Result<CounterGroupStat, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter_group: &mut CounterGroup = PerfView::table_mut(self).get_mut(&self_)?;
            let stat = raw::counter_group_stat(counter_group)?;
            let stat = Wrap::<CounterGroupStat>::from(&stat).into_inner();
            Ok(stat)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn drop(&mut self, rep: Resource<CounterGroup>) -> wasmtime::Result<()> {
        PerfView::table_mut(self).delete(rep)?;
        Ok(())
    }
}

impl<T> HostFixedCounterGroup for T
where
    T: PerfView,
{
    fn enable(
        &mut self,
        self_: Resource<FixedCounterGroup>,
    ) -> wasmtime::Result<Result<(), String>> {
        let f = || -> anyhow::Result<_> {
            let fixed_counter_group: &FixedCounterGroup = PerfView::table(self).get(&self_)?;
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
            let fixed_counter_group: &FixedCounterGroup = PerfView::table(self).get(&self_)?;
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
            let fixed_counter_group: &FixedCounterGroup = PerfView::table(self).get(&self_)?;
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
            let fixed_counter_group: &mut FixedCounterGroup =
                PerfView::table_mut(self).get_mut(&self_)?;
            let stat = raw::fixed_counter_group_stat(fixed_counter_group)?;
            let stat = Wrap::<CounterGroupStat>::from(&stat).into_inner();
            Ok(stat)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn drop(&mut self, rep: Resource<FixedCounterGroup>) -> wasmtime::Result<()> {
        PerfView::table_mut(self).delete(rep)?;
        Ok(())
    }
}

impl<T> HostCounterGuard for T
where
    T: PerfView,
{
    fn event_id(&mut self, self_: Resource<CounterGuard>) -> wasmtime::Result<u64> {
        let counter_guard: &CounterGuard = PerfView::table(self).get(&self_)?;
        let event_id = raw::counter_guard_event_id(counter_guard);
        Ok(event_id)
    }
    fn stat(
        &mut self,
        self_: Resource<CounterGuard>,
    ) -> wasmtime::Result<Result<CounterStat, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter_guard: &mut CounterGuard = PerfView::table_mut(self).get_mut(&self_)?;
            let stat = raw::counter_guard_stat(counter_guard)?;
            let stat = Wrap::<CounterStat>::from(&stat).into_inner();
            Ok(stat)
        };
        Ok(f().map_err(|e| e.to_string()))
    }
    fn drop(&mut self, rep: Resource<CounterGuard>) -> wasmtime::Result<()> {
        PerfView::table_mut(self).delete(rep)?;
        Ok(())
    }
}
