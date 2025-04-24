use std::{ffi::OsStr, path::Path, sync::LazyLock};

use nvml_wrapper::{Nvml, enum_wrappers::device::TemperatureSensor, error::NvmlError};

use crate::error::Result;

use super::{GpuInfo, GpuStats};

#[cfg(target_arch = "x86_64")]
const LIB_PATHS: &[&str] = &[
    "/usr/lib/x86_64-linux-gnu/libnvidia-ml.so",
    "/usr/lib/x86_64-linux-gnu/libnvidia-ml.so.1",
];

#[cfg(target_arch = "aarch64")]
const LIB_PATHS: &[&str] = &[
    "/usr/lib/aarch64-linux-gnu/libnvidia-ml.so",
    "/usr/lib/aarch64-linux-gnu/libnvidia-ml.so.1",
];

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
const LIB_PATHS: &[&str] = &[];

static NVML: LazyLock<Option<Nvml>> = LazyLock::new(|| {
    // First try default initialization
    if let Ok(nvml) = Nvml::init() {
        return Some(nvml);
    }

    // If default fails, try with specific library paths
    for path in LIB_PATHS {
        if Path::new(path).exists() {
            if let Ok(nvml) = Nvml::builder().lib_path(OsStr::new(path)).init() {
                return Some(nvml);
            }
        }
    }

    tracing::warn!("Failed to initialize NVML with all available methods");
    None
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
