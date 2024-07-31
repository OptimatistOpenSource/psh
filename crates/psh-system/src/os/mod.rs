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

pub(crate) mod handle;
mod raw;

use std::fmt::Display;

pub use handle::OsHandle;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct KernelVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
}

impl Display for KernelVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{}.{}.{}", self.major, self.minor, self.patch).fmt(f)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum DistroKind {
    Arch,
    CentOS,
    Debian,
    Fedora,
    Gentoo,
    Kali,
    Manjaro,
    Mint,
    NixOS,
    Other(String),
    PopOS,
    RedHat,
    Slackware,
    Ubuntu,
}

impl Display for DistroKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistroKind::Arch => f.write_str("Arch Linux"),
            DistroKind::CentOS => f.write_str("CentOS Linux"),
            DistroKind::Debian => f.write_str("Debian GNU/Linux"),
            DistroKind::Fedora => f.write_str("Fedora Linux"),
            DistroKind::Gentoo => f.write_str("Gentoo"),
            DistroKind::Kali => f.write_str("Kali GNU/Linux"),
            DistroKind::Manjaro => f.write_str("Manjaro Linux"),
            DistroKind::Mint => f.write_str("Linux Mint"),
            DistroKind::NixOS => f.write_str("NixOS"),
            DistroKind::PopOS => f.write_str("Pop!_OS"),
            DistroKind::RedHat => f.write_str("Red Hat Enterprise Linux"),
            DistroKind::Slackware => f.write_str("Slackware"),
            DistroKind::Ubuntu => f.write_str("Ubuntu"),
            DistroKind::Other(s) => f.write_str(s),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct DistroVersion {
    pub distro: DistroKind,
    pub version: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct OsInfo {
    pub distro: DistroVersion,
    pub kernel: KernelVersion,
}
