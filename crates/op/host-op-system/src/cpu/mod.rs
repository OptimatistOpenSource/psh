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
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.

use std::time::Duration;

use psh_system::cpu::{
    AddressSizes as HostAddressSizes, Arm64CpuInfo as HostArm64CpuInfo, CpuInfo as HostCpuInfo,
    CpuMask as HostCpuMask, CpuStats as HostCpuStats, CpuTime as HostCpuStat,
    TlbSize as HostTlbSize, X86_64CpuInfo as HostX86_64CpuInfo,
};

use crate::profiling::system::cpu::{
    self, AddressSizes as GuestAddressSizes, Arm64CpuInfo as GuestArm64CpuInfo,
    CpuInfo as GuestCpuInfo, CpuMask as GuestCpuMask, CpuStat as GuestCpuStat,
    CpuStats as GuestCpuStats, TlbSize as GuestTlbSize, X64CpuInfo as GuestX64CpuInfo,
};
use crate::SysCtx;

impl From<&HostCpuMask> for GuestCpuMask {
    fn from(value: &HostCpuMask) -> Self {
        Self {
            mask: value.0.clone(),
        }
    }
}

impl From<HostCpuMask> for GuestCpuMask {
    fn from(value: HostCpuMask) -> Self {
        Self { mask: value.0 }
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

impl From<HostAddressSizes> for GuestAddressSizes {
    fn from(value: HostAddressSizes) -> Self {
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

impl From<HostTlbSize> for GuestTlbSize {
    fn from(value: HostTlbSize) -> Self {
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

impl From<HostArm64CpuInfo> for GuestArm64CpuInfo {
    fn from(value: HostArm64CpuInfo) -> Self {
        Self {
            processor: value.processor as u32,
            bogomips: value.bogomips,
            features: value.features,
            cpu_implementer: value.cpu_implementer,
            cpu_architecture: value.cpu_architecture,
            cpu_variant: value.cpu_variant,
            cpu_part: value.cpu_part,
            cpu_revision: value.cpu_revision,
            address_sizes: value.address_sizes.into(),
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

impl From<HostX86_64CpuInfo> for GuestX64CpuInfo {
    fn from(value: HostX86_64CpuInfo) -> Self {
        Self {
            processor: value.processor as u32,
            vendor_id: value.vendor_id,
            model_name: value.model_name,
            cpu_family: value.cpu_family as u32,
            model: value.model as u32,
            stepping: value.stepping as u32,
            microcode: value.microcode,
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
            flag: value.flags,
            bugs: value.bugs,
            bogomips: value.bogomips,
            tlb_size: value.tlb_size.into(),
            clflush_size: value.clflush_size,
            cache_alignment: value.cache_alignment,
            address_sizes: value.address_sizes.into(),
            power_management: value.power_management,
        }
    }
}

impl From<&HostCpuInfo> for GuestCpuInfo {
    fn from(value: &HostCpuInfo) -> Self {
        match value {
            HostCpuInfo::X86_64(x64) => Self::X64(x64.iter().map(Into::into).collect()),
            HostCpuInfo::Arm64(arm64) => Self::Arm64(arm64.iter().map(Into::into).collect()),
            HostCpuInfo::Unsupported(unsupported) => Self::Unsupported(unsupported.clone()),
        }
    }
}

impl From<HostCpuInfo> for GuestCpuInfo {
    fn from(value: HostCpuInfo) -> Self {
        match value {
            HostCpuInfo::X86_64(x64) => Self::X64(x64.into_iter().map(Into::into).collect()),
            HostCpuInfo::Arm64(arm64) => Self::Arm64(arm64.into_iter().map(Into::into).collect()),
            HostCpuInfo::Unsupported(unsupported) => Self::Unsupported(unsupported),
        }
    }
}

impl From<&HostCpuStat> for GuestCpuStat {
    fn from(value: &HostCpuStat) -> Self {
        Self {
            user: value.user_ms(),
            nice: value.nice_ms(),
            system: value.system_ms(),
            idle: value.idle_ms(),
            iowait: value.iowait_ms(),
            irq: value.irq_ms(),
            softirq: value.softirq_ms(),
            steal: value.steal_ms(),
            guest: value.guest_ms(),
            guest_nice: value.guest_nice_ms(),
        }
    }
}

impl From<HostCpuStat> for GuestCpuStat {
    fn from(value: HostCpuStat) -> Self {
        Self {
            user: value.user_ms(),
            nice: value.nice_ms(),
            system: value.system_ms(),
            idle: value.idle_ms(),
            iowait: value.iowait_ms(),
            irq: value.irq_ms(),
            softirq: value.softirq_ms(),
            steal: value.steal_ms(),
            guest: value.guest_ms(),
            guest_nice: value.guest_nice_ms(),
        }
    }
}

impl From<&HostCpuStats> for GuestCpuStats {
    fn from(value: &HostCpuStats) -> Self {
        value.clone().into()
    }
}

impl From<HostCpuStats> for GuestCpuStats {
    fn from(value: HostCpuStats) -> Self {
        Self {
            total: (&value.total).into(),
            per_cpu: value.per_cpu.iter().map(Into::into).collect(),
            ctxt: value.ctxt,
            btime: value.btime,
            processes: value.processes,
            procs_running: value.procs_running,
            procs_blocked: value.procs_blocked,
        }
    }
}

impl cpu::Host for SysCtx {
    fn info(&mut self) -> Result<GuestCpuInfo, String> {
        self.cpu
            .info()
            .map(Into::into)
            .map_err(|err| err.to_string())
    }

    fn stat(&mut self, interval_ms: u64) -> Result<GuestCpuStats, String> {
        self.cpu
            .stat(Some(Duration::from_millis(interval_ms)))
            .map(Into::into)
            .map_err(|err| err.to_string())
    }
}
