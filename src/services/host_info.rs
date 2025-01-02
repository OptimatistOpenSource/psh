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

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use psh_system::cpu::CpuHandle;
use psh_system::os::OsHandle;

use super::pb::Ipv6Addr as PbIpv6;

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
impl From<&Ipv6Addr> for PbIpv6 {
    fn from(value: &Ipv6Addr) -> Self {
        let ip = value.to_bits().to_be();
        let high = (ip >> 64) as u64;
        let low = ip as u64;
        Self {
            hi_64_bits: high,
            lo_64_bits: low,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawInfo {
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
    pub os: Option<String>,
    pub arch: Option<String>,
    pub kernel_version: Option<String>,
    pub hostname: Option<String>,
}

impl RawInfo {
    pub fn new() -> Self {
        let hostname = nix::unistd::gethostname()
            .ok()
            .map(|v| v.to_string_lossy().to_string());
        let ipv4 = match local_ip_address::local_ip() {
            Ok(IpAddr::V4(v4)) => Some(v4),
            _ => None, // `local_ip_address::local_ip()` get v4
        };

        let ipv6 = match local_ip_address::local_ipv6() {
            Ok(IpAddr::V6(v6)) => Some(v6),
            _ => None, // `local_ip_address::local_ipv6()` get v6
        };

        let mut raw_info = Self {
            ipv4,
            ipv6,
            os: None,
            hostname,
            arch: None,
            kernel_version: None,
        };

        let cpu_hd = CpuHandle::new();
        if let Ok(cpu) = cpu_hd.info() {
            raw_info.arch = Some(cpu.to_string());
        }

        let os_hd = OsHandle::new();
        if let Ok(info) = os_hd.info() {
            raw_info.os = Some(info.distro.distro.to_string());
            raw_info.kernel_version = Some(info.kernel.to_string());
        }

        raw_info
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
