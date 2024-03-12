use crate::convert::Wrap;
use crate::profiling::perf::config::*;
use perf_event_rs::config::{Cpu as RawCpu, Error, Process as RawProcess};
use perf_event_rs::counting::{
    Config as RawConfig, CounterGroup, CounterGroupStat, CounterGuard, CounterStat,
    FixedCounterGroup,
};
use std::io;

pub fn counter_group_new(process: &Process, cpu: &Cpu) -> Result<CounterGroup, Error> {
    let process = Wrap::<RawProcess>::from(process).into_inner();
    let cpu = Wrap::<RawCpu>::from(cpu).into_inner();

    CounterGroup::new(&process, &cpu)
}

pub fn counter_group_add_member(
    counter_group: &mut CounterGroup,
    cfg: &Config,
) -> io::Result<CounterGuard> {
    let cfg = Wrap::<RawConfig>::from(cfg).into_inner();
    counter_group.add_member(&cfg)
}

pub fn counter_group_enable(counter_group: CounterGroup) -> io::Result<FixedCounterGroup> {
    counter_group.enable()
}

pub fn counter_group_stat(counter_group: &mut CounterGroup) -> io::Result<CounterGroupStat> {
    counter_group.stat()
}

pub fn fixed_counter_group_enable(fixed_counter_group: &FixedCounterGroup) -> io::Result<()> {
    fixed_counter_group.enable()
}

pub fn fixed_counter_group_disable(fixed_counter_group: &FixedCounterGroup) -> io::Result<()> {
    fixed_counter_group.disable()
}

pub fn fixed_counter_group_reset(fixed_counter_group: &FixedCounterGroup) -> io::Result<()> {
    fixed_counter_group.reset()
}

pub fn fixed_counter_group_stat(
    fixed_counter_group: &mut FixedCounterGroup,
) -> io::Result<CounterGroupStat> {
    fixed_counter_group.stat()
}

pub fn counter_guard_event_id(counter_guard: &CounterGuard) -> u64 {
    counter_guard.event_id()
}

pub fn counter_guard_stat(counter_guard: &mut CounterGuard) -> io::Result<CounterStat> {
    counter_guard.stat()
}
