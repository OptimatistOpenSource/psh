use wasmtime::component::{Linker, ResourceTable};

pub mod convert;
pub mod counting;

#[cfg(test)]
mod tests;

pub type Counter = perf_event_rs::counting::Counter;
pub type CounterGroup = perf_event_rs::counting::CounterGroup;
pub type FixedCounterGroup = perf_event_rs::counting::FixedCounterGroup;
pub type CounterGuard = perf_event_rs::counting::CounterGuard;

wasmtime::component::bindgen!({
    path: "wit",
    world: "imports",
    with: {
        "profiling:perf/counter/counter"                  : Counter,
        "profiling:perf/counter-group/counter-group"      : CounterGroup,
        "profiling:perf/counter-group/fixed-counter-group": FixedCounterGroup,
        "profiling:perf/counter-group/counter-guard"      : CounterGuard,
    }
});

pub struct PerfCtx {
    table: ResourceTable,
}

#[allow(clippy::new_without_default)]
impl PerfCtx {
    pub fn new() -> Self {
        Self {
            table: ResourceTable::new(),
        }
    }
}

impl profiling::perf::config::Host for PerfCtx {}
impl profiling::perf::counter::Host for PerfCtx {}
impl profiling::perf::counter_group::Host for PerfCtx {}

pub fn add_to_linker<T>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut PerfCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    crate::Imports::add_to_linker(l, f)
}
