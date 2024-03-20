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
