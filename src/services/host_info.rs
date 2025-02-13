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

use std::net::IpAddr;

use psh_system::cpu::{CpuHandle, CpuInfo};
use psh_system::os::OsHandle;

use psh_proto::SendHostInfoReq;

pub fn new_info_req(instance_id: String) -> SendHostInfoReq {
    let hostname = nix::unistd::gethostname()
        .ok()
        .map(|v| v.to_string_lossy().to_string());

    let local_ipv4_addr = match local_ip_address::local_ip() {
        Ok(IpAddr::V4(it)) => Some(it.to_bits().to_be()),
        _ => None, // `local_ip_address::local_ip()` get v4
    };

    let local_ipv6_addr = match local_ip_address::local_ipv6() {
        Ok(IpAddr::V6(it)) => Some(it.into()),
        _ => None, // `local_ip_address::local_ipv6()` get v6
    };

    let architecture = CpuHandle::new().info().ok().map(|it| match it {
        CpuInfo::X86_64(_) => "x86_64".to_string(),
        CpuInfo::Arm64(_) => "aarch64".to_string(),
        CpuInfo::Unsupported(u) => u,
    });

    let mut req = SendHostInfoReq {
        local_ipv4_addr,
        local_ipv6_addr,
        os: None,
        hostname,
        architecture,
        kernel_version: None,
        instance_id,
    };

    if let Ok(it) = OsHandle::new().info() {
        req.os = Some(it.distro.distro.to_string());
        req.kernel_version = Some(it.kernel.to_string());
    }

    req
}
