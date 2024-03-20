mod host;
mod irq;
mod raw;
mod stat;

#[derive(Debug, PartialEq)]
pub enum InterruptType {
    Common(u32),
    ArchSpecific(String),
}

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

#[derive(Debug)]
pub struct IrqDetails {
    pub irq_number: u32,
    pub smp_affinity: Option<String>,
    pub smp_affinity_list: Option<String>,
    // Fixme (Chengdong Li): I think it would be better to
    // specify a concrete type instead String.
    pub node: Option<String>,
}

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
