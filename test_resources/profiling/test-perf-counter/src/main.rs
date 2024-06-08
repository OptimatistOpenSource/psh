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
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.
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
