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

use crate::runtime::psh::profiling::{cpu, memory, system};
use crate::runtime::ServerWasiView;

use self::common::cpu_info::parse_cpuinfo;
use self::common::mem_info::parse_meminfo;
use self::common::system::{get_kernel_version, parse_os_version};
use self::common::{Arm64CpuInfo as HostArm64CpuInfo, X86_64CpuInfo as HostX86_64CpuInfo};

impl memory::Host for ServerWasiView {
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
}

impl system::Host for ServerWasiView {
    fn os_version(&mut self) -> wasmtime::Result<Option<String>> {
        parse_os_version!().map_err(wasmtime::Error::from)
    }

    fn kernel_version(&mut self) -> wasmtime::Result<String> {
        get_kernel_version().map_err(wasmtime::Error::from)
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

impl cpu::Host for ServerWasiView {
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
