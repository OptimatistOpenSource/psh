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

pub(crate) mod handle;
mod raw;

use crate::cpu::CpuMask;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct RpsQueue {
    pub name: String,
    // FIXME: better to use more expressive type than raw string
    pub cpus: Option<CpuMask>,
    pub flow_cnt: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct RpsDetails {
    pub dev: String,
    pub queues: Vec<RpsQueue>,
}
