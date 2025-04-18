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

use psh_system::os::{
    DistroKind as HostDistroKind, DistroVersion as HostDistroVersion,
    KernelVersion as HostKernelVersion, OsInfo as HostOsInfo,
};

use crate::{
    SysCtx,
    profiling::system::os::{
        self, DistroKind as GuestDistroKind, DistroVersion as GuestDistroVersion,
        KernelVersion as GuestKernelVersion, OsInfo as GuestOsInfo,
    },
};

impl From<&HostDistroKind> for GuestDistroKind {
    fn from(value: &HostDistroKind) -> Self {
        match value {
            HostDistroKind::Arch => Self::Arch,
            HostDistroKind::CentOS => Self::CentOs,
            HostDistroKind::Debian => Self::Debian,
            HostDistroKind::Fedora => Self::Fedora,
            HostDistroKind::Gentoo => Self::Gentoo,
            HostDistroKind::Kali => Self::Kali,
            HostDistroKind::Manjaro => Self::Manjaro,
            HostDistroKind::Mint => Self::Mint,
            HostDistroKind::NixOS => Self::NixOs,
            HostDistroKind::Other(distro) => Self::Other(distro.clone()),
            HostDistroKind::PopOS => Self::PopOs,
            HostDistroKind::RedHat => Self::RedHat,
            HostDistroKind::Slackware => Self::Slackware,
            HostDistroKind::Ubuntu => Self::Ubuntu,
        }
    }
}

impl From<HostDistroKind> for GuestDistroKind {
    fn from(value: HostDistroKind) -> Self {
        match value {
            HostDistroKind::Arch => Self::Arch,
            HostDistroKind::CentOS => Self::CentOs,
            HostDistroKind::Debian => Self::Debian,
            HostDistroKind::Fedora => Self::Fedora,
            HostDistroKind::Gentoo => Self::Gentoo,
            HostDistroKind::Kali => Self::Kali,
            HostDistroKind::Manjaro => Self::Manjaro,
            HostDistroKind::Mint => Self::Mint,
            HostDistroKind::NixOS => Self::NixOs,
            HostDistroKind::Other(distro) => Self::Other(distro),
            HostDistroKind::PopOS => Self::PopOs,
            HostDistroKind::RedHat => Self::RedHat,
            HostDistroKind::Slackware => Self::Slackware,
            HostDistroKind::Ubuntu => Self::Ubuntu,
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

impl From<HostDistroVersion> for GuestDistroVersion {
    fn from(value: HostDistroVersion) -> Self {
        Self {
            distro: value.distro.into(),
            version: value.version,
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

impl From<HostKernelVersion> for GuestKernelVersion {
    fn from(value: HostKernelVersion) -> Self {
        Self {
            major: value.major,
            minor: value.minor,
            patch: value.patch,
        }
    }
}

impl From<&HostOsInfo> for GuestOsInfo {
    fn from(value: &HostOsInfo) -> Self {
        Self {
            distro_version: (&value.distro).into(),
            kernel_version: (&value.kernel).into(),
        }
    }
}

impl From<HostOsInfo> for GuestOsInfo {
    fn from(value: HostOsInfo) -> Self {
        Self {
            distro_version: value.distro.into(),
            kernel_version: value.kernel.into(),
        }
    }
}

impl os::Host for SysCtx {
    fn info(&mut self) -> Result<GuestOsInfo, String> {
        self.os
            .info()
            .map(Into::into)
            .map_err(|err| err.to_string())
    }
}
