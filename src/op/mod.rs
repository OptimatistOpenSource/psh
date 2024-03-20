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

pub mod common;

use std::process::Command;

use crate::infra::util::which;
use crate::op::common::memory_module::parse_memory_module;
use crate::runtime::psh::profiling::{
    cpu, interrupts, memory,
    system::{self, DistroKind, DistroVersion, KernelVersion},
};
use crate::runtime::State;

use self::common::cpu_info::parse_cpuinfo;
use self::common::interrupts::parse_interrupts;
use self::common::irq::parse_irq;
use self::common::mem_info::parse_meminfo;
use self::common::system::{get_kernel_version, parse_distro_version};
use self::common::{
    Arm64CpuInfo as HostArm64CpuInfo, DistroKind as HostDistroKind,
    DistroVersion as HostDistroVersion, InterruptDetails, InterruptType, IrqDetails,
    KernelVersion as HostKernelVersion, MemoryModule as HostMemoryModule,
    X86_64CpuInfo as HostX86_64CpuInfo,
};

impl From<&HostMemoryModule> for memory::MemoryModule {
    fn from(value: &HostMemoryModule) -> Self {
        memory::MemoryModule {
            array_handle: value.array_handle,
            error_info_handle: value.error_info_handle,
            total_width: value.total_width,
            data_width: value.data_width,
            size: value.size,
            form_factor: value.form_factor.clone(),
            set: value.set.clone(),
            locator: value.locator.clone(),
            bank_locator: value.bank_locator.clone(),
            module_type: value.r#type.clone(),
            type_detail: value.type_detail.clone(),
            speed: value.speed.clone(),
            manufacturer: value.manufacturer.clone(),
            serial_number: value.serial_number.clone(),
            asset_tag: value.asset_tag.clone(),
            part_number: value.part_number.clone(),
            rank: value.rank,
            configured_memory_speed: value.configured_memory_speed.clone(),
            min_voltage: value.min_voltage.clone(),
            max_voltage: value.max_voltage.clone(),
            configured_voltage: value.configured_voltage.clone(),
            memory_technology: value.memory_technology.clone(),
            memory_operating_mode_capability: value.memory_operating_mode_capability.clone(),
            firmware_version: value.firmware_version.clone(),
            module_manufacturer_id: value.module_manufacturer_id.clone(),
            module_product_id: value.module_product_id.clone(),
            memory_subsystem_controller_manufacturer_id: value
                .memory_subsystem_controller_manufacturer_id
                .clone(),
            memory_subsystem_controller_product_id: value
                .memory_subsystem_controller_product_id
                .clone(),
            non_volatile_size: value.non_volatile_size,
            volatile_size: value.volatile_size,
            cache_size: value.cache_size,
            logical_size: value.logical_size,
        }
    }
}

impl memory::Host for State {
    fn get_memory_info(&mut self) -> wasmtime::Result<Result<memory::MemoryInfo, String>> {
        let mem_info = parse_meminfo!().unwrap();
        Ok(Ok(memory::MemoryInfo {
            mem_total: mem_info.mem_total,
            mem_free: mem_info.mem_free,
            mem_available: mem_info.mem_available,
            buffers: mem_info.buffers,
            cached: mem_info.cached,
            swap_cached: mem_info.swap_cached,
            active: mem_info.active,
            inactive: mem_info.inactive,
            active_anon: mem_info.active_anon,
            inactive_anon: mem_info.inactive_anon,
            active_file: mem_info.active_file,
            inactive_file: mem_info.inactive_file,
            unevictable: mem_info.unevictable,
            mlocked: mem_info.mlocked,
            swap_total: mem_info.swap_total,
            swap_free: mem_info.swap_free,
            dirty: mem_info.dirty,
            writeback: mem_info.writeback,
            anon_pages: mem_info.anon_pages,
            mapped: mem_info.mapped,
            shmem: mem_info.shmem,
            kreclaimable: mem_info.kreclaimable,
            slab: mem_info.slab,
            sreclaimable: mem_info.sreclaimable,
            sunreclaim: mem_info.sunreclaim,
            kernel_stack: mem_info.kernel_stack,
            page_tables: mem_info.page_tables,
            nfs_unstable: mem_info.nfs_unstable,
            bounce: mem_info.bounce,
            writeback_tmp: mem_info.writeback_tmp,
            commit_limit: mem_info.commit_limit,
            committed_as: mem_info.committed_as,
            vmalloc_total: mem_info.vmalloc_total,
            vmalloc_used: mem_info.vmalloc_used,
            vmalloc_chunk: mem_info.vmalloc_chunk,
            percpu: mem_info.percpu,
            cma_total: mem_info.cma_total,
            cma_free: mem_info.cma_free,
            hardware_corrupted: mem_info.hardware_corrupted,
            anon_huge_pages: mem_info.anon_huge_pages,
            shmem_huge_pages: mem_info.shmem_huge_pages,
            shmem_pmd_mapped: mem_info.shmem_pmd_mapped,
            file_huge_pages: mem_info.file_huge_pages,
            file_pmd_mapped: mem_info.file_pmd_mapped,
            huge_pages_total: mem_info.huge_pages_total,
            huge_pages_free: mem_info.huge_pages_free,
            huge_pages_rsvd: mem_info.huge_pages_rsvd,
            huge_pages_surp: mem_info.huge_pages_surp,
            huge_page_size: mem_info.huge_page_size,
            huge_tlb: mem_info.huge_tlb,
            direct_map4k: mem_info.direct_map4k,
            direct_map2_m: mem_info.direct_map2_m,
            direct_map1_g: mem_info.direct_map1_g,
        }))
    }

    fn get_memory_module(&mut self) -> wasmtime::Result<Result<Vec<memory::MemoryModule>, String>> {
        if let Some(dmidecode_exe) = which("dmidecode") {
            let output = Command::new(dmidecode_exe).arg("-t").arg("17").output()?;

            let res = parse_memory_module(std::str::from_utf8(&output.stdout)?)
                .iter()
                .map(memory::MemoryModule::from)
                .collect::<Vec<memory::MemoryModule>>();

            Ok(Ok(res))
        } else {
            Ok(Err("Can not find `dmidecode` executable path.".to_string()))
        }
    }
}

impl From<HostDistroKind> for DistroKind {
    fn from(value: HostDistroKind) -> Self {
        match value {
            HostDistroKind::Arch => DistroKind::Arch,
            HostDistroKind::CentOS => DistroKind::CentOs,
            HostDistroKind::Debian => DistroKind::Debian,
            HostDistroKind::Fedora => DistroKind::Fedora,
            HostDistroKind::Gentoo => DistroKind::Gentoo,
            HostDistroKind::Kali => DistroKind::Kali,
            HostDistroKind::Manjaro => DistroKind::Manjaro,
            HostDistroKind::Mint => DistroKind::Mint,
            HostDistroKind::NixOS => DistroKind::NixOs,
            HostDistroKind::Other(distro) => DistroKind::Other(distro.clone()),
            HostDistroKind::PopOS => DistroKind::PopOs,
            HostDistroKind::RedHat => DistroKind::RedHat,
            HostDistroKind::Slackware => DistroKind::Slackware,
            HostDistroKind::Ubuntu => DistroKind::Ubuntu,
        }
    }
}

impl From<HostDistroVersion> for DistroVersion {
    fn from(value: HostDistroVersion) -> Self {
        Self {
            distro: value.distro.into(),
            version: value.version,
        }
    }
}

impl From<HostKernelVersion> for KernelVersion {
    fn from(value: HostKernelVersion) -> Self {
        Self {
            major: value.major,
            minor: value.minor,
            patch: value.patch,
        }
    }
}

impl system::Host for State {
    fn get_distro_version(&mut self) -> wasmtime::Result<Result<DistroVersion, String>> {
        match parse_distro_version!() {
            Ok(distro) => Ok(Ok(DistroVersion::from(distro))),
            Err(err) => Ok(Err(err.to_string())),
        }
    }

    fn get_kernel_version(&mut self) -> wasmtime::Result<Result<KernelVersion, String>> {
        match get_kernel_version() {
            Ok(version) => Ok(Ok(version.into())),
            Err(err) => Ok(Err(err.to_string())),
        }
    }
}

impl From<&HostArm64CpuInfo> for cpu::Arm64CpuInfo {
    fn from(value: &HostArm64CpuInfo) -> Self {
        cpu::Arm64CpuInfo {
            processor: value.processor as u32,
            bogomips: value.bogomips,
            features: value.features.clone(),
            cpu_implementer: value.cpu_implementer,
            cpu_architecture: value.cpu_architecture,
            cpu_variant: value.cpu_variant,
            cpu_part: value.cpu_part,
            cpu_revision: value.cpu_revision,
            address_sizes: cpu::AddressSizes {
                phy: value.address_sizes.phy,
                virt: value.address_sizes.virt,
            },
        }
    }
}

impl From<&HostX86_64CpuInfo> for cpu::X64CpuInfo {
    fn from(value: &HostX86_64CpuInfo) -> Self {
        cpu::X64CpuInfo {
            processor: value.processor as u32,
            vendor_id: value.vendor_id.clone(),
            model_name: value.model_name.clone(),
            cpu_family: value.cpu_family as u32,
            model: value.model as u32,
            stepping: value.stepping as u32,
            microcode: value.microcode.clone(),
            cpu_mhz: value.cpu_mhz,
            cache_size: value.cache_size,
            physical_id: value.physical_id as u32,
            siblings: value.siblings as u32,
            core_id: value.core_id as u32,
            cpu_cores: value.cpu_cores as u32,
            apicid: value.apicid as u32,
            initial_apicid: value.initial_apicid as u32,
            fpu: value.fpu,
            fpu_exception: value.fpu_exception,
            cpuid_level: value.cpuid_level as u32,
            wp: value.wp,
            flag: value.flags.clone(),
            bugs: value.bugs.clone(),
            bogomips: value.bogomips,
            tlb_size: cpu::TlbSize {
                count: value.tlb_size.count,
                unit: value.tlb_size.unit,
            },
            clflush_size: value.clflush_size,
            cache_alignment: value.cache_alignment,
            address_sizes: cpu::AddressSizes {
                phy: value.address_sizes.phy,
                virt: value.address_sizes.virt,
            },
            power_management: value.power_management.clone(),
        }
    }
}

impl cpu::Host for State {
    fn get_cpu_info(&mut self) -> wasmtime::Result<Result<cpu::CpuInfo, String>> {
        let cpu_info = parse_cpuinfo!().unwrap();
        let res = match cpu_info {
            common::CPUInfo::X86_64(x64) => {
                Ok(cpu::CpuInfo::X64(x64.iter().map(|x| x.into()).collect()))
            }
            common::CPUInfo::Arm64(arm64) => Ok(cpu::CpuInfo::Arm64(
                arm64.iter().map(|x| x.into()).collect(),
            )),
            common::CPUInfo::Unsupported(unsupported) => Err(unsupported),
        };

        Ok(res)
    }
}

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

impl interrupts::Host for State {
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
