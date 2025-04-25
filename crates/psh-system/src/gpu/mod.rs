mod handle;
mod raw;

use nvml_wrapper::enum_wrappers::device::MemoryLocation;
use nvml_wrapper::struct_wrappers::device::{MemoryInfo, PciInfo, Utilization};

pub use handle::NvidiaHandle;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GpuInfo {
    pub driver_version: String,
    pub cuda_driver_version: i32,
}

#[derive(Clone, Debug)]
pub struct EccErrorInfo {
    pub location: MemoryLocation,
    pub corrected_volatile: u64,
    pub corrected_aggregate: u64,
    pub uncorrected_volatile: u64,
    pub uncorrected_aggregate: u64,
}

#[derive(Clone, Debug)]
pub struct GpuStats {
    // Static fields (rarely change)
    pub uuid: String,
    pub name: String,
    pub vbios_version: String,
    pub pci_info: PciInfo,
    pub irq_num: u32,
    pub max_pcie_link_gen: u32,
    pub max_pcie_link_width: u32,

    // Dynamic fields (change frequently)
    // Temperature and cooling
    pub temperature: u32,
    pub fan_speeds: Vec<u32>,

    // PCIe status
    pub current_pcie_link_gen: u32,
    pub current_pcie_link_width: u32,

    // Performance and utilization
    pub utilization_rates: Utilization,
    pub performance_state: u32,
    pub compute_mode: u32,

    // Memory
    pub memory_info: MemoryInfo,
    pub ecc_errors: Vec<EccErrorInfo>,

    // Power
    pub power_usage: u32,
    pub power_limit: u32,
    pub enforced_power_limit: u32,

    // Clocks
    pub memory_clock: u32,
    pub graphics_clock: u32,
    pub sm_clock: u32,
    pub video_clock: u32,

    // Processes
    pub graphics_processes_count: u32,
    pub compute_processes_count: u32,
}
