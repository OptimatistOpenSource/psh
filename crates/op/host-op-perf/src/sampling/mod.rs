use perf_event_rs::{
    config::{Cpu as RawCpu, Process as RawProcess},
    sampling::Sampler,
};
use wasmtime::component::Resource;

use crate::{PerfCtx, convert::Wrap, profiling::perf::sampler::*};

impl HostSampler for PerfCtx {
    fn new(
        &mut self,
        process: Process,
        cpu: Cpu,
        cfg: SamplingConfig,
    ) -> wasmtime::Result<Result<Resource<Sampler>, String>> {
        let process = Wrap::<RawProcess>::from(&process).into_inner();
        let cpu = Wrap::<RawCpu>::from(&cpu).into_inner();
        let sampler = perf_event_rs::sampling::Sampler::new(&process, &cpu, 512, &cfg.try_into()?)?;
        Ok(self.table.push(sampler).map_err(|v| v.to_string()))
    }

    fn enable(&mut self, self_: Resource<Sampler>) -> wasmtime::Result<Result<(), String>> {
        let sampler = self.table.get(&self_)?;
        Ok(sampler.enable().map_err(|e| e.to_string()))
    }

    fn disable(&mut self, self_: Resource<Sampler>) -> wasmtime::Result<Result<(), String>> {
        let sampler = self.table.get(&self_)?;
        Ok(sampler.disable().map_err(|e| e.to_string()))
    }

    fn drop(&mut self, rep: Resource<Sampler>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }

    fn reset(&mut self, self_: Resource<Sampler>) -> wasmtime::Result<Result<(), String>> {
        let sampler = self.table.get(&self_)?;
        Ok(sampler.reset().map_err(|e| e.to_string()))
    }

    fn pause(&mut self, self_: Resource<Sampler>) -> wasmtime::Result<Result<(), String>> {
        let sampler = self.table.get(&self_)?;
        Ok(sampler.pause().map_err(|e| e.to_string()))
    }

    fn resume(&mut self, self_: Resource<Sampler>) -> wasmtime::Result<Result<(), String>> {
        let sampler = self.table.get(&self_)?;
        Ok(sampler.resume().map_err(|e| e.to_string()))
    }

    fn refresh(
        &mut self,
        self_: Resource<Sampler>,
        refresh: i32,
    ) -> wasmtime::Result<Result<(), String>> {
        let sampler = self.table.get(&self_)?;
        Ok(sampler.refresh(refresh).map_err(|e| e.to_string()))
    }

    fn update_period(
        &mut self,
        self_: Resource<Sampler>,
        new: u64,
    ) -> wasmtime::Result<Result<(), String>> {
        let sampler = self.table.get(&self_)?;
        Ok(sampler.update_period(new).map_err(|e| e.to_string()))
    }

    fn next_record(
        &mut self,
        self_: Resource<crate::Sampler>,
    ) -> wasmtime::Result<Result<Option<Record>, String>> {
        let sampler = self.table.get_mut(&self_)?;
        let map_err = sampler.next_record().map(Into::into);
        Ok(Ok(map_err))
    }

    fn event_id(&mut self, self_: Resource<Sampler>) -> wasmtime::Result<Result<u64, String>> {
        let sampler = self.table.get_mut(&self_)?;
        Ok(sampler.event_id().map_err(|e| e.to_string()))
    }

    fn stat(&mut self, self_: Resource<Sampler>) -> wasmtime::Result<Result<SamplerStat, String>> {
        let sampler = self.table.get_mut(&self_)?;
        Ok(sampler.stat().map(Into::into).map_err(|e| e.to_string()))
    }

    fn get_raw_fd(&mut self, self_: Resource<Sampler>) -> wasmtime::Result<i32> {
        let sampler = self.table.get(&self_)?;
        Ok(sampler.get_raw_fd())
    }
}
