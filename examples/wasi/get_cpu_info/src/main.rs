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
#[rustfmt::skip]
mod bindings;

use crate::bindings::profiling::system::cpu::{self, CpuInfo};

fn main() {
    println!("Hello, world!");
    println!("{}", crate::bindings::name());
    let a = cpu::get_cpu_info().unwrap();

    match a {
        CpuInfo::X64(cpus) => {
            println!("CPU architecture: x86_64");
            println!("  nr: {}", cpus.len());
            for cpu in cpus {
                println!("{:?}\n", cpu);
            }
        }
        CpuInfo::Arm64(_) => todo!(),
        CpuInfo::Unsupported(_) => todo!(),
    }

    // test if host not implemented perf imports.
    // perf::perf_new_counter(0, 0, 0);
    // panic!("test panic");
}
