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
mod disk;
mod interrupt;
mod memory;
mod network;
mod os;
mod process;
mod rps;

use std::sync::Arc;

use psh_system::cpu::CpuHandle;
use psh_system::disk::DiskHandle;
use psh_system::interrupt::InterruptHandle;
use psh_system::memory::MemoryHandle;
use psh_system::network::NetworkHandle;
use psh_system::os::OsHandle;
use psh_system::rps::RpsHandle;
use wasmtime::component::{Linker, ResourceTable};

use psh_system::process::{Process, ProcessHandle};
use psh_system::System;

pub type HostProc = Arc<Process>;

wasmtime::component::bindgen!({
    path: "../../../psh-sdk-wit/wit/deps/system",
    world: "imports",
    with: {
        "profiling:system/process/process": HostProc,
    }
});

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SysCtx {
    table: ResourceTable,
    system: System,
    os: OsHandle,
    cpu: CpuHandle,
    disk: DiskHandle,
    memory: MemoryHandle,
    process: ProcessHandle,
    rps: RpsHandle,
    network: NetworkHandle,
    interrupt: InterruptHandle,
}

pub fn add_to_linker<T>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut SysCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    crate::Imports::add_to_linker(l, f)
}
