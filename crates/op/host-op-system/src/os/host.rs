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
    profiling::system::os::{
        self, DistroKind as GuestDistroKind, DistroVersion as GuestDistroVersion,
        KernelVersion as GuestKernelVersion,
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
}
