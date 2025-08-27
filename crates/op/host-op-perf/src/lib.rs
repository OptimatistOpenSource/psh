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

use wasmtime::component::{Linker, ResourceTable};

pub mod convert;
pub mod counting;
pub mod sampling;

pub type Sampler = perf_event_rs::sampling::Sampler;
pub type Counter = perf_event_rs::counting::Counter;
pub type CounterGroup = perf_event_rs::counting::CounterGroup;
pub type FixedCounterGroup = perf_event_rs::counting::FixedCounterGroup;
pub type CounterGuard = perf_event_rs::counting::CounterGuard;

wasmtime::component::bindgen!({
    path: "../../../psh-sdk-wit/wit/deps/perf",
    world: "imports",
    with: {
        "profiling:perf/sampler/sampler"                  : Sampler,
        "profiling:perf/counter/counter"                  : Counter,
        "profiling:perf/counter-group/counter-group"      : CounterGroup,
        "profiling:perf/counter-group/fixed-counter-group": FixedCounterGroup,
        "profiling:perf/counter-group/counter-guard"      : CounterGuard,
    },
    // https://github.com/bytecodealliance/wasmtime/pull/8310
    // wasmtime have added a config in bindgen! macro to allow user specify
    // whether they want a function be able to trap(outer wasmtime::Result).
    // by default the value is false, we use true here to compatible with our
    // previous implementations.
    trappable_imports: true,
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

impl profiling::perf::sampler::Host for PerfCtx {}
impl profiling::perf::config::Host for PerfCtx {}
impl profiling::perf::counter::Host for PerfCtx {}
impl profiling::perf::counter_group::Host for PerfCtx {}

pub fn add_to_linker<T>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut PerfCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    crate::Imports::add_to_linker(l, f)
}
