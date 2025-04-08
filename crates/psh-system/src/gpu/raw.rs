use std::sync::LazyLock;

use nvml_wrapper::{Nvml, enum_wrappers::device::TemperatureSensor, error::NvmlError};

use crate::error::Result;

use super::{GpuInfo, GpuStats};

static NVML: LazyLock<Option<Nvml>> = LazyLock::new(|| match Nvml::init() {
    Ok(n) => Some(n),
    Err(e) => {
        tracing::warn!("{e}");
        None
    }
});

pub fn gpu_info() -> Result<GpuInfo> {
    let nvml = NVML.as_ref().ok_or(NvmlError::Uninitialized)?;
    Ok(GpuInfo {
        driver_version: nvml.sys_driver_version()?,
        cuda_driver_version: nvml.sys_cuda_driver_version()?,
    })
}

pub fn gpu_stats() -> Result<Vec<GpuStats>> {
    let nvml = NVML.as_ref().ok_or(NvmlError::Uninitialized)?;

    let count = nvml.device_count()?;

    let mut var = Vec::with_capacity(count as usize);
    for i in 0..count {
        let device = nvml.device_by_index(i)?;
        let num_fans = device.num_fans()?;
        let mut fan_speeds = Vec::with_capacity(num_fans as usize);
        for i in 0..num_fans {
            fan_speeds.push(device.fan_speed(i)?);
        }

        var.push(GpuStats {
            uuid: device.uuid()?,
            fan_speeds,
            vbios_version: device.vbios_version()?,
            temperature: device.temperature(TemperatureSensor::Gpu)?,
            name: device.name()?,
            pci_info: device.pci_info()?,
            irq_num: device.irq_num()?,
            max_pcie_link_gen: device.max_pcie_link_gen()?,
            current_pcie_link_gen: device.current_pcie_link_gen()?,
            utilization_rates: device.utilization_rates()?,
            memory_info: device.memory_info()?,
        });
    }

    Ok(var)
}
