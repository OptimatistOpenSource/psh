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
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.
mod cpu;
mod interrupts;
mod memory;
mod os;
mod rps;
mod utils;

use wasmtime::component::Linker;

wasmtime::component::bindgen!({
    path: "../../../src/psh-sdk-wit/wit/deps/system",
    world: "imports",
});

pub struct SysCtx {
    // TODO: add more fields
}

pub fn add_to_linker<T>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut SysCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    crate::Imports::add_to_linker(l, f)
}
