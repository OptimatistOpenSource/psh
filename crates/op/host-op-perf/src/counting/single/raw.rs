use perf_event_rs::config;
use perf_event_rs::config::{Cpu, Process};
use perf_event_rs::counting::{Config, Counter, CounterStat};
use std::io;

pub fn counter_new(process: &Process, cpu: &Cpu, cfg: &mut Config) -> config::Result<Counter> {
    Counter::new(process, cpu, cfg)
}

pub fn counter_enable(counter: &Counter) -> io::Result<()> {
    counter.enable()
}

pub fn counter_disable(counter: &Counter) -> io::Result<()> {
    counter.disable()
}

pub fn counter_reset(counter: &Counter) -> io::Result<()> {
    counter.reset()
}

pub fn counter_stat(counter: &mut Counter) -> io::Result<CounterStat> {
    counter.stat()
}
