#[allow(dead_code)]
mod bindings;

use crate::bindings::profiling;
use profiling::perf::config::{Config, Cpu, Event, EventScope, ExtraConfig, HardwareEvent, Process};
use profiling::perf::counter_group::CounterGroup;

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

    // Test counter_group_new
    let counter_group = CounterGroup::new(Process::Current, Cpu::Any).unwrap();

    // Test counter_group_add_member
    let cpu_cycles_guard = counter_group.add_member(&cfg).unwrap();
    let instructions_guard = counter_group.add_member(&cfg).unwrap();

    // Test counter_group_stat
    let stat = counter_group.stat().unwrap();
    assert_eq!(stat.time_enabled, 0);
    assert_eq!(stat.time_running, 0);
    let cpu_cycles = stat
        .member_counts
        .iter()
        .find(|(id, _)| *id == cpu_cycles_guard.event_id())
        .map(|(_, v)| *v)
        .unwrap();
    let instructions = stat
        .member_counts
        .iter()
        .find(|(id, _)| *id == instructions_guard.event_id())
        .map(|(_, v)| *v)
        .unwrap();
    assert_eq!(cpu_cycles, 0);
    assert_eq!(instructions, 0);

    // Test counter_group_enable
    let fixed_counter_group = CounterGroup::enable(counter_group).unwrap();
    println!("do something here...");
    fixed_counter_group.disable().unwrap();

    // Test fixed_counter_group_stat
    let stat = fixed_counter_group.stat().unwrap();
    assert!(stat.time_enabled > 0);
    assert!(stat.time_running > 0);
    let cpu_cycles = stat
        .member_counts
        .iter()
        .find(|(id, _)| *id == cpu_cycles_guard.event_id())
        .map(|(_, v)| *v)
        .unwrap();
    let instructions = stat
        .member_counts
        .iter()
        .find(|(id, _)| *id == instructions_guard.event_id())
        .map(|(_, v)| *v)
        .unwrap();
    assert!(cpu_cycles > 0);
    assert!(instructions > 0);
    println!("time_enabled: {}", stat.time_enabled);
    println!("time_running: {}", stat.time_running);
    println!(
        "cpu cycles / instructions = {} / {}",
        cpu_cycles, instructions
    );

    // Test fixed_counter_group_disable
    assert_eq!(
        fixed_counter_group.stat().unwrap().time_enabled,
        stat.time_enabled
    );

    // Test fixed_counter_group_reset
    fixed_counter_group.reset().unwrap();
    let stat = fixed_counter_group.stat().unwrap();
    let cpu_cycles = stat
        .member_counts
        .iter()
        .find(|(id, _)| *id == cpu_cycles_guard.event_id())
        .map(|(_, v)| *v)
        .unwrap();
    let instructions = stat
        .member_counts
        .iter()
        .find(|(id, _)| *id == instructions_guard.event_id())
        .map(|(_, v)| *v)
        .unwrap();
    assert_eq!(cpu_cycles, 0);
    assert_eq!(instructions, 0);

    // Test fixed_counter_group_enable
    fixed_counter_group.enable().unwrap();
    let stat = fixed_counter_group.stat().unwrap();
    assert!(stat.time_enabled > 0);
    assert!(stat.time_running > 0);
    let cpu_cycles = stat
        .member_counts
        .iter()
        .find(|(id, _)| *id == cpu_cycles_guard.event_id())
        .map(|(_, v)| *v)
        .unwrap();
    let instructions = stat
        .member_counts
        .iter()
        .find(|(id, _)| *id == instructions_guard.event_id())
        .map(|(_, v)| *v)
        .unwrap();
    assert!(cpu_cycles > 0);
    assert!(instructions > 0);
}
