use std::fs::File;
use std::io::prelude::Write;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::Path;

use psh_system::cpu::CpuHandle;
use psh_system::os::OsHandle;

use super::pb::HostInfoRequest;
use super::pb::Ipv6Addr as PbIpv6;
use crate::infra::option::WrapOption;

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

impl From<RawInfo> for HostInfoRequest {
    fn from(value: RawInfo) -> Self {
        Self {
            os: value.os,
            hostname: value.hostname,
            architecture: value.arch,
            kernel_version: value.kernel_version,
            local_ipv4_addr: value.ipv4.map(|v| v.to_bits().to_be()),
            local_ipv6_addr: value.ipv6.map(|v| v.into()),
            instance_id: value.instance_id,
        }
    }
}

impl From<&RawInfo> for HostInfoRequest {
    fn from(value: &RawInfo) -> Self {
        Self {
            os: value.os.clone(),
            hostname: value.hostname.clone(),
            architecture: value.arch.clone(),
            kernel_version: value.kernel_version.clone(),
            local_ipv4_addr: value.ipv4.map(|v| v.to_bits().to_be()),
            local_ipv6_addr: value.ipv6.map(|v| v.into()),
            instance_id: value.instance_id.clone(),
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
    instance_id: Option<String>,
}

impl RawInfo {
    pub const INSTANCE_ID_FILE: &'static str = "/etc/psh/instance.id";

    pub fn new() -> Self {
        let hostname = nix::unistd::gethostname()
            .ok()
            .map(|v| v.to_string_lossy().to_string());
        let ipv4 = match local_ip_address::local_ip() {
            Ok(IpAddr::V4(v4)) => v4.wrap_some(),
            _ => None, // `local_ip_address::local_ip()` get v4
        };

        let ipv6 = match local_ip_address::local_ipv6() {
            Ok(IpAddr::V6(v6)) => v6.wrap_some(),
            _ => None, // `local_ip_address::local_ipv6()` get v6
        };
        let instance_id = Self::get_instance_id().ok();

        let mut raw_info = Self {
            ipv4,
            ipv6,
            os: None,
            hostname,
            arch: None,
            kernel_version: None,
            instance_id,
        };

        let cpu_hd = CpuHandle::new();
        if let Ok(cpu) = cpu_hd.info() {
            raw_info.arch = cpu.to_string().wrap_some();
        }

        let os_hd = OsHandle::new();
        if let Ok(info) = os_hd.info() {
            raw_info.os = info.distro.distro.to_string().wrap_some();
            raw_info.kernel_version = info.kernel.to_string().wrap_some();
        }

        raw_info
    }

    pub fn get_instance_id() -> anyhow::Result<String> {
        let s = std::fs::read_to_string(Self::INSTANCE_ID_FILE)?;
        Ok(s)
    }

    pub fn write_instance_id(id: &str) -> anyhow::Result<()> {
        let path = Path::new(Self::INSTANCE_ID_FILE);
        std::fs::create_dir_all(path.parent().expect("No parent dir"))?;
        let mut f = File::create(path)?;
        f.write_all(id.as_bytes())?;

        Ok(())
    }

    pub fn to_heartbeat(&self) -> Self {
        Self {
            ipv4: None,
            ipv6: None,
            os: None,
            arch: None,
            kernel_version: None,
            hostname: None,
            instance_id: self.instance_id.clone(),
        }
    }

    /// Update instance_id when get a new instance_id
    pub fn set_instance_id(&mut self, instance_id: String) {
        if Some(&instance_id) != self.instance_id.as_ref() {
            _ = Self::write_instance_id(&instance_id);
            self.instance_id = Some(instance_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_info() {
        let info = RawInfo::new();
        let info: HostInfoRequest = info.into();
        dbg!(&info);
    }

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
