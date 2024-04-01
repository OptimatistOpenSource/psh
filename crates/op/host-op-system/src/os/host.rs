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

use sysinfo::{
    DiskUsage as HostDiskUsage, Process as HostProcess, ProcessStatus as HostProcessStatus,
};

use crate::{
    profiling::system::os::{
        self, DiskUsage as GuestDiskUsage, DistroKind as GuestDistroKind,
        DistroVersion as GuestDistroVersion, KernelVersion as GuestKernelVersion,
        Process as GuestProcess, ProcessStatus as GuestProcessStatus,
    },
    SysCtx,
};

use crate::os::{
    DistroKind as HostDistroKind, DistroVersion as HostDistroVersion,
    KernelVersion as HostKernelVersion,
};

use super::raw::{get_kernel_version, parse_distro_version};

impl From<&HostDistroKind> for GuestDistroKind {
    fn from(value: &HostDistroKind) -> Self {
        match value {
            HostDistroKind::Arch => GuestDistroKind::Arch,
            HostDistroKind::CentOS => GuestDistroKind::CentOs,
            HostDistroKind::Debian => GuestDistroKind::Debian,
            HostDistroKind::Fedora => GuestDistroKind::Fedora,
            HostDistroKind::Gentoo => GuestDistroKind::Gentoo,
            HostDistroKind::Kali => GuestDistroKind::Kali,
            HostDistroKind::Manjaro => GuestDistroKind::Manjaro,
            HostDistroKind::Mint => GuestDistroKind::Mint,
            HostDistroKind::NixOS => GuestDistroKind::NixOs,
            HostDistroKind::Other(distro) => GuestDistroKind::Other(distro.clone()),
            HostDistroKind::PopOS => GuestDistroKind::PopOs,
            HostDistroKind::RedHat => GuestDistroKind::RedHat,
            HostDistroKind::Slackware => GuestDistroKind::Slackware,
            HostDistroKind::Ubuntu => GuestDistroKind::Ubuntu,
        }
    }
}

impl From<&HostDistroVersion> for GuestDistroVersion {
    fn from(value: &HostDistroVersion) -> Self {
        Self {
            distro: (&value.distro).into(),
            version: value.version.clone(),
        }
    }
}

impl From<&HostKernelVersion> for GuestKernelVersion {
    fn from(value: &HostKernelVersion) -> Self {
        Self {
            major: value.major,
            minor: value.minor,
            patch: value.patch,
        }
    }
}

impl From<&HostDiskUsage> for GuestDiskUsage {
    fn from(value: &HostDiskUsage) -> Self {
        Self {
            written_bytes: value.total_written_bytes,
            read_bytes: value.total_read_bytes,
        }
    }
}

impl From<&HostProcessStatus> for GuestProcessStatus {
    fn from(value: &HostProcessStatus) -> Self {
        match value {
            HostProcessStatus::Idle => GuestProcessStatus::Idle,
            HostProcessStatus::Run => GuestProcessStatus::Run,
            HostProcessStatus::Sleep => GuestProcessStatus::Sleep,
            HostProcessStatus::Stop => GuestProcessStatus::Stop,
            HostProcessStatus::Zombie => GuestProcessStatus::Zombie,
            HostProcessStatus::Tracing => GuestProcessStatus::Tracing,
            HostProcessStatus::Dead => GuestProcessStatus::Dead,
            HostProcessStatus::Wakekill => GuestProcessStatus::Wakekill,
            HostProcessStatus::Waking => GuestProcessStatus::Waking,
            HostProcessStatus::Parked => GuestProcessStatus::Parked,
            HostProcessStatus::LockBlocked => GuestProcessStatus::LockBlocked,
            HostProcessStatus::UninterruptibleDiskSleep => {
                GuestProcessStatus::UninterruptibleDiskSleep
            }
            HostProcessStatus::Unknown(status) => GuestProcessStatus::Unknown(*status),
        }
    }
}

impl From<&HostProcess> for GuestProcess {
    fn from(value: &HostProcess) -> Self {
        Self {
            pid: value.pid().as_u32().into(),
            name: value.name().to_owned(),
            cmd: value.cmd().iter().map(|s| s.to_owned()).collect(),
            exe: value.exe().and_then(|p| p.to_str().map(|s| s.to_owned())),
            environ: value.environ().iter().map(|e| e.to_owned()).collect(),
            cwd: value.cwd().and_then(|p| p.to_str().map(|s| s.to_owned())),
            root: value.root().and_then(|p| p.to_str().map(|s| s.to_owned())),
            start_time: value.start_time(),
            parent_id: value.parent().map(|p| p.as_u32().into()),
            user_id: value.user_id().map(|u| (*u.clone()).into()),
            effective_user_id: value.effective_user_id().map(|u| (*u.clone()).into()),
            group_id: value.group_id().map(|u| u64::from(*u)),
            effective_group_id: value.effective_group_id().map(|u| u64::from(*u)),
            run_time: value.run_time(),
            status: (&value.status()).into(),
            cpu_usage: value.cpu_usage(),
            disk_usage: (&value.disk_usage()).into(),
            memory_usage: value.memory(),
            virtual_memory_usage: value.virtual_memory(),
        }
    }
}

impl os::Host for SysCtx {
    fn get_distro_version(&mut self) -> wasmtime::Result<Result<GuestDistroVersion, String>> {
        let res = match parse_distro_version!() {
            Ok(distro) => Ok((&distro).into()),
            Err(err) => Err(err.to_string()),
        };
        Ok(res)
    }

    fn get_kernel_version(&mut self) -> wasmtime::Result<Result<GuestKernelVersion, String>> {
        let res = match get_kernel_version() {
            Ok(version) => Ok((&version).into()),
            Err(err) => Err(err.to_string()),
        };
        Ok(res)
    }

    fn get_processes(&mut self) -> wasmtime::Result<Result<Vec<GuestProcess>, String>> {
        self.sys.refresh_processes();
        Ok(Ok(self
            .sys
            .processes()
            .values()
            .map(|p| p.into())
            .collect()))
    }
}
