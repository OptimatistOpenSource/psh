use std::{
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn parse_os_version_impl(path: &str) -> io::Result<Option<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once('=') {
            if key == "VERSION" {
                return Ok(Some(value.trim_matches('"').to_string()));
            }
        }
    }
    Ok(None)
}

pub fn get_kernel_version() -> io::Result<String> {
    let info = uname::uname()?;
    Ok(info.release)
}

#[allow(unused_macros)]
macro_rules! parse_os_version {
    ($path:expr) => {
        crate::op::common::system::parse_os_version_impl($path)
    };
    () => {
        crate::op::common::system::parse_os_version_impl("/etc/os-release")
    };
}

pub(crate) use parse_os_version;

#[cfg(test)]
mod test {
    use super::get_kernel_version;

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_kernel_version() {
        let version = get_kernel_version().unwrap();
        let splits = version.split_once('-');
        assert!(splits.is_some());
        let first = splits.unwrap().0;
        let nums: Vec<_> = first
            .split('.')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        assert_eq!(nums.len(), 3);
        assert!(nums[0] > 3);
    }

    #[test]
    fn test_parse_os_version() {
        let version_mapping: [(&str, Option<&str>); 42] = [
            ("alma", Some("9.1 (Lime Lynx)")),
            ("alpine", None),
            ("amazon", Some("2022")),
            ("antergos", Some("18.11-ISO-Rolling")),
            ("arch", None),
            ("archarm", None),
            ("arcolinux", None),
            ("centos", Some("8")),
            ("centos_stream", Some("8")),
            ("clearlinux", Some("1")),
            ("clearos", Some("7 (Final)")),
            ("cumulus", Some("Cumulus Linux 3.7.2")),
            ("debian", Some("11 (bullseye)")),
            ("elementary", Some("6 Odin")),
            ("endeavouros", None),
            ("fedora", Some("38 (Workstation Edition)")),
            ("gentoo", None),
            ("ios_xr", Some("6.0.0.14I")),
            ("kali", Some("2018.4")),
            ("linuxmint", Some("19 (Tara)")),
            ("mageia", Some("6")),
            ("manjaro", None),
            ("manjaro-arm", None),
            ("nexus", Some("7.0(BUILDER)")),
            ("nixos", Some("18.09.1436.a7fd4310c0c (Jellyfish)")),
            ("opensuseleap", Some("42.3")),
            ("oracle", Some("9.1")),
            ("pop_os", Some("22.04 LTS")),
            ("rancheros", Some("v1.4.2")),
            ("raspbian", Some("10 (buster)")),
            ("redhat", Some("9.1 (Plow)")),
            ("rocky", Some("9.1 (Blue Onyx)")),
            ("scientific", Some("7.5 (Nitrogen)")),
            ("slackware", Some("14.2")),
            ("sled", Some("15")),
            ("sles", Some("15-SP1")),
            ("sles_sap", Some("12.0.1")),
            ("ubuntu", Some("22.04 LTS (Jammy Jellyfish)")),
            ("virtuozzo", Some("7.0.16")),
            ("xbian", Some("1.0 (knockout)")),
            ("xcp-ng", Some("8.2.0")),
            ("xenserver", Some("7.6.0")),
        ];
        for (distro, version) in version_mapping {
            let path = format!("./test_resources/os-releases/{}", distro);
            let result = parse_os_version!(&path).unwrap();
            assert_eq!(result, version.map(ToOwned::to_owned));
        }
    }
}
