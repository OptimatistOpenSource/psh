#[allow(dead_code)]
mod bindings;

use crate::bindings::profiling;
use profiling::perf::config::{Config, Cpu, Event, EventScope, ExtraConfig, HardwareEvent, Process};
use profiling::perf::counter::Counter;

fn main() {
    let cfg = Config {
        event: Event::Hardware(HardwareEvent::CpuCycles),
        scopes: vec![
            EventScope::User,
            EventScope::Kernel,
            EventScope::Hv,
            EventScope::Idle,
            EventScope::Host,
            EventScope::Guest,
        ],
        extra_config: ExtraConfig {
            pinned: false,
            exclusive: false,
            inherit: false,
            inherit_stat: false,
            inherit_thread: false,
            enable_on_exec: false,
            remove_on_exec: false,
        },
    };

    let counter = Counter::new(Process::Current, Cpu::Any, &cfg).unwrap();

    counter.enable().unwrap();
    println!("do something here...");
    counter.disable().unwrap();

    let result = counter.stat().unwrap();

    println!("event_count: {}", result.event_count);
    println!("time_enabled: {}", result.time_enabled);
    println!("time_running: {}", result.time_running);
    assert!(result.event_count > 0);
    assert!(result.time_enabled > 0);
    assert!(result.time_running > 0);
}
