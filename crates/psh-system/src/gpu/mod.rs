mod handle;
mod raw;

use nvml_wrapper::struct_wrappers::device::{MemoryInfo, PciInfo, Utilization};

pub use handle::NvidiaHandle;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GpuInfo {
    pub driver_version: String,
    pub cuda_driver_version: i32,
}

#[derive(Clone, Debug)]
pub struct GpuStats {
    pub uuid: String,
    /// the vec index is fan index
    pub fan_speeds: Vec<u32>,
    pub vbios_version: String,
    pub temperature: u32,
    pub name: String,
    pub pci_info: PciInfo,
    pub irq_num: u32,
    pub max_pcie_link_gen: u32,
    pub current_pcie_link_gen: u32,
    pub utilization_rates: Utilization,
    pub memory_info: MemoryInfo,
}
