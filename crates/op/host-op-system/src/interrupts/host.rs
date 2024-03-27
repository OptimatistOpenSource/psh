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
use crate::interrupts::raw::{parse_interrupts, parse_irq};
use crate::profiling::system::interrupts;
use crate::SysCtx;

impl From<&IrqDetails> for interrupts::Irq {
    fn from(value: &IrqDetails) -> Self {
        interrupts::Irq {
            number: value.irq_number,
            smp_affinity: value.smp_affinity.clone(),
            smp_affinity_list: value.smp_affinity_list.clone(),
            node: value.node.clone(),
        }
    }
}

impl From<&InterruptDetails> for interrupts::Stat {
    fn from(value: &InterruptDetails) -> Self {
        interrupts::Stat {
            interrupt_type: match &value.interrupt_type {
                InterruptType::Common(irq) => interrupts::Kind::Common(*irq),
                InterruptType::ArchSpecific(irq) => interrupts::Kind::ArchSpecific(irq.clone()),
            },
            description: value.description.clone(),
            per_cpu_counts: value.cpu_counts.clone(),
        }
    }
}

impl interrupts::Host for SysCtx {
    fn get_interrupts_info(&mut self) -> wasmtime::Result<Result<Vec<interrupts::Irq>, String>> {
        match parse_irq!() {
            Ok(irq) => Ok(Ok(irq
                .iter()
                .map(interrupts::Irq::from)
                .collect::<Vec<interrupts::Irq>>())),
            Err(e) => Ok(Err(format!("{}: {}", "get interrupt info failed", e))),
        }
    }

    fn get_interrupts_stat(&mut self) -> wasmtime::Result<Result<Vec<interrupts::Stat>, String>> {
        match parse_interrupts!() {
            Ok(bindings) => Ok(Ok(bindings
                .iter()
                .map(interrupts::Stat::from)
                .collect::<Vec<interrupts::Stat>>())),
            Err(e) => Ok(Err(format!("{}: {}", "get interrupt statistics failed", e))),
        }
    }
}
