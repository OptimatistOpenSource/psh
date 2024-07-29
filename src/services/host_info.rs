use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use sysinfo::System;

use super::pb::{host_info_request, HostInfoRequest};
use crate::infra::option::WrapOption;

impl From<RawInfo> for HostInfoRequest {
    fn from(value: RawInfo) -> Self {
        use host_info_request::IpAddr;

        let ip_addr = {
            if let Some(v4) = value.ipv4 {
                IpAddr::Ipv4(v4.to_bits()).wrap_some()
            } else if let Some(v6) = value.ipv6 {
                IpAddr::Ipv6(v6.octets().to_vec()).wrap_some()
            } else {
                None
            }
        };

        Self {
            ip_addr,
            os: value.os,
            hostname: value.hostname,
            token: value.token,
            architecture: value.arch,
            kernel_version: value.kernel_version,
        }
    }
}

impl From<&RawInfo> for HostInfoRequest {
    fn from(value: &RawInfo) -> Self {
        use host_info_request::IpAddr;

        let ip_addr = {
            if let Some(v4) = value.ipv4 {
                IpAddr::Ipv4(v4.to_bits()).wrap_some()
            } else if let Some(v6) = value.ipv6 {
                IpAddr::Ipv6(v6.octets().to_vec()).wrap_some()
            } else {
                None
            }
        };

        Self {
            ip_addr,
            os: value.os.clone(),
            hostname: value.hostname.clone(),
            token: value.token.clone(),
            architecture: value.arch.clone(),
            kernel_version: value.kernel_version.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawInfo {
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
    os: Option<String>,
    arch: Option<String>,
    kernel_version: Option<String>,
    hostname: Option<String>,
    token: String,
}

impl RawInfo {
    pub fn new(token: String) -> Self {
        let ipv4 = match local_ip_address::local_ip() {
            Ok(IpAddr::V4(v4)) => v4.wrap_some(),
            _ => None, // `local_ip_address::local_ip()` get v4
        };

        let ipv6 = match local_ip_address::local_ipv6() {
            Ok(IpAddr::V6(v6)) => v6.wrap_some(),
            _ => None, // `local_ip_address::local_ipv6()` get v6
        };

        Self {
            ipv4,
            ipv6,
            os: System::name(),
            hostname: System::host_name(),
            token,
            arch: System::cpu_arch(),
            kernel_version: System::kernel_version(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_info() {
        let info = RawInfo::new("token".to_owned());
        let info: HostInfoRequest = info.into();
        dbg!(&info);
        assert_eq!(info.token, "token");
    }
}
