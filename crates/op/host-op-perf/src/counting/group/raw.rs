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

use std::io;

use perf_event_rs::{
    config::{Cpu, Error, Process},
    counting::{
        Config, CounterGroup, CounterGroupStat, CounterGuard, CounterStat, FixedCounterGroup,
    },
};

pub fn counter_group_new(process: &Process, cpu: &Cpu) -> Result<CounterGroup, Error> {
    CounterGroup::new(process, cpu)
}

pub fn counter_group_add_member(
    counter_group: &mut CounterGroup,
    cfg: &mut Config,
) -> io::Result<CounterGuard> {
    counter_group.add_member(cfg)
}

pub fn counter_group_enable(counter_group: CounterGroup) -> io::Result<FixedCounterGroup> {
    counter_group.enable()
}

pub fn counter_group_stat(counter_group: &mut CounterGroup) -> io::Result<CounterGroupStat> {
    counter_group.stat()
}

pub fn counter_group_into_fixed(counter_group: CounterGroup) -> io::Result<FixedCounterGroup> {
    counter_group.into_fixed()
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

pub const fn counter_guard_event_id(counter_guard: &CounterGuard) -> u64 {
    counter_guard.event_id()
}

pub fn counter_guard_stat(counter_guard: &mut CounterGuard) -> io::Result<CounterStat> {
    counter_guard.stat()
}
