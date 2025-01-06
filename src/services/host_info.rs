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

use std::net::{IpAddr, Ipv6Addr};

use psh_system::cpu::CpuHandle;
use psh_system::os::OsHandle;

use super::pb::{Ipv6Addr as PbIpv6, SendHostInfoReq};

impl From<Ipv6Addr> for PbIpv6 {
    fn from(value: Ipv6Addr) -> Self {
        let ip = value.to_bits().to_be();
        let high = (ip >> 64) as u64;
        let low = ip as u64;
        Self {
            hi_64_bits: high,
            lo_64_bits: low,
        }
    }
}

impl SendHostInfoReq {
    pub fn new(instance_id: String) -> Self {
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

        let architecture = CpuHandle::new().info().ok().map(|it| it.to_string());

        let mut req = Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ip_transform() {
        let var: u128 = 1;

        let raw = Ipv6Addr::from_bits(var);

        let pb_ip: PbIpv6 = raw.into();

        let hi = (pb_ip.hi_64_bits as u128) << 64;
        let lo = pb_ip.lo_64_bits as u128;
        let ip = Ipv6Addr::from_bits(u128::from_be(hi | lo));

        assert_eq!(ip, Ipv6Addr::from_bits(1));
    }
}
