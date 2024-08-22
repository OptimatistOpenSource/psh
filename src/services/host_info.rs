use psh_system::cpu::CpuHandle;
use psh_system::os::OsHandle;

use super::pb::HostInfoRequest;
use crate::infra::option::WrapOption;

impl From<RawInfo> for HostInfoRequest {
    fn from(value: RawInfo) -> Self {
        Self {
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
        Self {
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
    os: Option<String>,
    arch: Option<String>,
    kernel_version: Option<String>,
    hostname: Option<String>,
    token: String,
}

impl RawInfo {
    pub fn new(token: String) -> Self {
        let hostname = nix::unistd::gethostname()
            .ok()
            .map(|v| v.to_string_lossy().to_string());

        let mut raw_info = Self {
            os: None,
            hostname,
            token,
            arch: None,
            kernel_version: None,
        };

        let cpu_hd = CpuHandle::new();
        if let Ok(cpu) = cpu_hd.info() {
            raw_info.arch = cpu.to_string().wrap_some();
        }

        let os_hd = OsHandle::new();
        if let Ok(info) = os_hd.info() {
            raw_info.os = info.distro.distro.to_string().wrap_some();
            raw_info.kernel_version = info.kernel.to_string().wrap_some();

            return raw_info;
        }

        raw_info
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
