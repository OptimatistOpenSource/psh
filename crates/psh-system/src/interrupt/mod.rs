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
mod irq;
mod raw;
mod stat;

pub use handle::InterruptHandle;

#[derive(Debug, PartialEq, Clone)]
pub enum InterruptType {
    Common(u32),
    ArchSpecific(String),
}

#[derive(Debug, Clone)]
pub struct InterruptDetails {
    pub cpu_counts: Vec<u64>,
    pub interrupt_type: InterruptType,
    pub description: String,
}

impl InterruptDetails {
    fn new(cpu_counts: Vec<u64>, interrupt_type: InterruptType, description: String) -> Self {
        Self {
            cpu_counts,
            interrupt_type,
            description,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IrqDetails {
    pub irq_number: u32,
    pub smp_affinity: Option<String>,
    pub smp_affinity_list: Option<String>,
    // Fixme (Chengdong Li): I think it would be better to
    // specify a concrete type instead String.
    pub node: Option<String>,
}

#[allow(dead_code)]
impl IrqDetails {
    fn new(irq_number: u32) -> Self {
        Self {
            irq_number,
            smp_affinity: None,
            smp_affinity_list: None,
            node: None,
        }
    }
}
