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

use crate::{
    profiling::system::cpu::{
        self, AddressSizes as GuestAddressSizes, Arm64CpuInfo as GuestArm64CpuInfo,
        CpuInfo as GuestCpuInfo, CpuMask as GuestCpuMask, TlbSize as GuestTlbSize,
        X64CpuInfo as GuestX64CpuInfo,
    },
    SysCtx,
};

use super::{
    raw::parse_cpuinfo, AddressSizes as HostAddressSizes, Arm64CpuInfo as HostArm64CpuInfo,
    CPUInfo as HostCpuInfo, CpuMask as HostCpuMask, TlbSize as HostTlbSize,
    X86_64CpuInfo as HostX86_64CpuInfo,
};

impl From<&HostCpuMask> for GuestCpuMask {
    fn from(value: &HostCpuMask) -> Self {
        Self {
            mask: value.0.clone(),
        }
    }
}

impl From<&HostAddressSizes> for GuestAddressSizes {
    fn from(value: &HostAddressSizes) -> Self {
        Self {
            phy: value.phy,
            virt: value.virt,
        }
    }
}

impl From<&HostTlbSize> for GuestTlbSize {
    fn from(value: &HostTlbSize) -> Self {
        Self {
            count: value.count,
            unit: value.unit,
        }
    }
}

impl From<&HostArm64CpuInfo> for GuestArm64CpuInfo {
    fn from(value: &HostArm64CpuInfo) -> Self {
        Self {
            processor: value.processor as u32,
            bogomips: value.bogomips,
            features: value.features.clone(),
            cpu_implementer: value.cpu_implementer,
            cpu_architecture: value.cpu_architecture,
            cpu_variant: value.cpu_variant,
            cpu_part: value.cpu_part,
            cpu_revision: value.cpu_revision,
            address_sizes: (&value.address_sizes).into(),
        }
    }
}

impl From<&HostX86_64CpuInfo> for GuestX64CpuInfo {
    fn from(value: &HostX86_64CpuInfo) -> Self {
        Self {
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
            tlb_size: (&value.tlb_size).into(),
            clflush_size: value.clflush_size,
            cache_alignment: value.cache_alignment,
            address_sizes: (&value.address_sizes).into(),
            power_management: value.power_management.clone(),
        }
    }
}

impl From<&HostCpuInfo> for GuestCpuInfo {
    fn from(value: &HostCpuInfo) -> Self {
        match value {
            HostCpuInfo::X86_64(x64) => GuestCpuInfo::X64(x64.iter().map(|x| x.into()).collect()),
            HostCpuInfo::Arm64(arm64) => {
                GuestCpuInfo::Arm64(arm64.iter().map(|x| x.into()).collect())
            }
            HostCpuInfo::Unsupported(unsupported) => GuestCpuInfo::Unsupported(unsupported.clone()),
        }
    }
}

impl cpu::Host for SysCtx {
    fn info(&mut self) -> wasmtime::Result<Result<GuestCpuInfo, String>> {
        let cpu_info = match parse_cpuinfo!() {
            Ok(ref info) => Ok(info.into()),
            Err(err) => Err(err.to_string()),
        };
        Ok(cpu_info)
    }
}
