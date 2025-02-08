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

pub mod cpu;
pub mod disk;
pub mod error;
pub mod interrupt;
pub mod memory;
pub mod network;
pub mod os;
pub mod process;
pub mod rps;
mod utils;
pub mod vmstat;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct System {
    pub page_size: u64,
    pub boot_time_sec: u64,
    pub tick_per_sec: u64,
}

impl Default for System {
    fn default() -> Self {
        Self {
            page_size: procfs::page_size(),
            boot_time_sec: procfs::boot_time_secs().unwrap_or(0),
            tick_per_sec: procfs::ticks_per_second(),
        }
    }
}
