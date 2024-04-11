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

use super::{InterruptDetails, InterruptType, IrqDetails};
use crate::interrupt::raw::{parse_interrupts, parse_irq};
use crate::profiling::system::interrupt;
use crate::SysCtx;

impl From<&IrqDetails> for interrupt::InterruptInfo {
    fn from(value: &IrqDetails) -> Self {
        Self {
            number: value.irq_number,
            smp_affinity: value.smp_affinity.clone(),
            smp_affinity_list: value.smp_affinity_list.clone(),
            node: value.node.clone(),
        }
    }
}

impl From<&InterruptDetails> for interrupt::InterruptStat {
    fn from(value: &InterruptDetails) -> Self {
        Self {
            interrupt_type: match &value.interrupt_type {
                InterruptType::Common(irq) => interrupt::InterruptType::Common(*irq),
                InterruptType::ArchSpecific(irq) => {
                    interrupt::InterruptType::ArchSpecific(irq.clone())
                }
            },
            description: value.description.clone(),
            per_cpu_counts: value.cpu_counts.clone(),
        }
    }
}

impl interrupt::Host for SysCtx {
    fn info(&mut self) -> wasmtime::Result<Result<Vec<interrupt::InterruptInfo>, String>> {
        let info = match parse_irq!() {
            Ok(irq) => Ok(irq
                .iter()
                .map(interrupt::InterruptInfo::from)
                .collect::<Vec<interrupt::InterruptInfo>>()),
            Err(e) => Err(format!("{}: {}", "get interrupt info failed", e)),
        };
        Ok(info)
    }

    fn stat(&mut self) -> wasmtime::Result<Result<Vec<interrupt::InterruptStat>, String>> {
        let stat = match parse_interrupts!() {
            Ok(bindings) => Ok(bindings
                .iter()
                .map(interrupt::InterruptStat::from)
                .collect::<Vec<interrupt::InterruptStat>>()),
            Err(e) => Err(format!("{}: {}", "get interrupt statistics failed", e)),
        };
        Ok(stat)
    }
}
