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

pub use handle::OsHandle;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct KernelVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
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
