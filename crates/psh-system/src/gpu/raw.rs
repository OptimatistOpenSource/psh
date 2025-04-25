use std::{ffi::OsStr, path::Path, sync::LazyLock};

use nvml_wrapper::{
    Nvml,
    enum_wrappers::device::{Clock, EccCounter, MemoryError, MemoryLocation, TemperatureSensor},
    error::NvmlError,
};

use crate::error::Result;

use super::{EccErrorInfo, GpuInfo, GpuStats};

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

        // Static fields
        let uuid = device.uuid()?;
        let name = device.name()?;
        let vbios_version = device.vbios_version()?;
        let pci_info = device.pci_info()?;
        let irq_num = device.irq_num()?;
        let max_pcie_link_gen = device.max_pcie_link_gen()?;
        let max_pcie_link_width = device.max_pcie_link_width()?;

        // Temperature and cooling
        let temperature = device.temperature(TemperatureSensor::Gpu)?;
        let num_fans = device.num_fans()?;
        let mut fan_speeds = Vec::with_capacity(num_fans as usize);
        for i in 0..num_fans {
            fan_speeds.push(device.fan_speed(i)?);
        }

        // PCIe status
        let current_pcie_link_gen = device.current_pcie_link_gen()?;
        let current_pcie_link_width = device.current_pcie_link_width()?;

        // Performance and utilization
        let utilization_rates = device.utilization_rates()?;
        let performance_state = device.performance_state()? as u32;
        let compute_mode = device.compute_mode()? as u32;

        // Memory
        let memory_info = device.memory_info()?;
        let mut ecc_errors = Vec::new();
        match device.is_ecc_enabled() {
            Ok(_) => {
                let locations = [
                    MemoryLocation::L1Cache,
                    MemoryLocation::L2Cache,
                    MemoryLocation::Device,
                    MemoryLocation::RegisterFile,
                    MemoryLocation::Texture,
                    MemoryLocation::Shared,
                    MemoryLocation::Cbu,
                    MemoryLocation::SRAM,
                ];

                for location in locations {
                    let corrected_volatile = device.memory_error_counter(
                        MemoryError::Corrected,
                        EccCounter::Volatile,
                        location.clone(),
                    )?;
                    let corrected_aggregate = device.memory_error_counter(
                        MemoryError::Corrected,
                        EccCounter::Aggregate,
                        location.clone(),
                    )?;
                    let uncorrected_volatile = device.memory_error_counter(
                        MemoryError::Uncorrected,
                        EccCounter::Volatile,
                        location.clone(),
                    )?;
                    let uncorrected_aggregate = device.memory_error_counter(
                        MemoryError::Uncorrected,
                        EccCounter::Aggregate,
                        location.clone(),
                    )?;

                    ecc_errors.push(EccErrorInfo {
                        location: location.clone(),
                        corrected_volatile,
                        corrected_aggregate,
                        uncorrected_volatile,
                        uncorrected_aggregate,
                    });
                }
            }
            Err(NvmlError::NotSupported) => {
                // device does not support ECC
            }
            Err(e) => {
                tracing::warn!("Failed to collect ecc errors: {}", e);
                return Err(e.into());
            }
        }

        // Power
        let power_usage = device.power_usage()?;
        let power_limit = device.power_management_limit()?;
        let enforced_power_limit = device.enforced_power_limit()?;

        // Clocks
        let memory_clock = device.clock_info(Clock::Memory)?;
        let graphics_clock = device.clock_info(Clock::Graphics)?;
        let sm_clock = device.clock_info(Clock::SM)?;
        let video_clock = device.clock_info(Clock::Video)?;

        // Processes
        let graphics_processes_count = device.running_graphics_processes_count()?;
        let compute_processes_count = device.running_compute_processes_count()?;

        var.push(GpuStats {
            // Static fields
            uuid,
            name,
            vbios_version,
            pci_info,
            irq_num,
            max_pcie_link_gen,
            max_pcie_link_width,

            // Temperature and cooling
            temperature,
            fan_speeds,

            // PCIe status
            current_pcie_link_gen,
            current_pcie_link_width,

            // Performance and utilization
            utilization_rates,
            performance_state,
            compute_mode,

            // Memory
            memory_info,
            ecc_errors,

            // Power
            power_usage,
            power_limit,
            enforced_power_limit,

            // Clocks
            memory_clock,
            graphics_clock,
            sm_clock,
            video_clock,

            // Processes
            graphics_processes_count,
            compute_processes_count,
        });
    }

    Ok(var)
}
