mod raw;

use crate::convert::Wrap;
use crate::profiling::perf::counter::*;
use crate::PerfCtx;
use perf_event_rs::counting::Counter;
use wasmtime::component::Resource;

impl HostCounter for PerfCtx {
    fn new(
        &mut self,
        process: Process,
        cpu: Cpu,
        cfg: Config,
    ) -> wasmtime::Result<Result<Resource<Counter>, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter = raw::counter_new(&process, &cpu, &cfg)?;
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
