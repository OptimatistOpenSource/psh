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

use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use super::{DistroKind, DistroVersion, KernelVersion};

impl FromStr for DistroKind {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Arch Linux" => Self::Arch,
            "CentOS Linux" => Self::CentOS,
            "Debian GNU/Linux" => Self::Debian,
            "Fedora Linux" => Self::Fedora,
            "Gentoo" => Self::Gentoo,
            "Kali GNU/Linux" => Self::Kali,
            "Linux Mint" => Self::Mint,
            "Manjaro Linux" => Self::Manjaro,
            "NixOS" => Self::NixOS,
            "Pop!_OS" => Self::PopOS,
            "Red Hat Enterprise Linux" => Self::RedHat,
            "Slackware" => Self::Slackware,
            "Ubuntu" => Self::Ubuntu,
            distro => Self::Other(distro.to_owned()),
        })
    }
}

pub fn parse_distro_version_impl(path: &str) -> anyhow::Result<DistroVersion> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut version = DistroVersion {
        distro: DistroKind::Other("Unknown".to_owned()),
        version: None,
    };
    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once('=') {
            if key == "VERSION" {
                version.version = Some(value.trim_matches('"').to_string());
            } else if key == "NAME" {
                let name = value.trim_matches('"');
                let Ok(distro) = name.parse();
                version.distro = distro;
            }
        }
    }
    Ok(version)
}

pub fn get_kernel_version() -> anyhow::Result<KernelVersion> {
    use procfs::sys::kernel::Version;
    let info = uname::uname()?;
    let version: Version = info.release.parse().map_err(|e: &str| anyhow::anyhow!(e))?;
    Ok(version.into())
}

macro_rules! parse_distro_version {
    ($path:expr) => {
        crate::os::raw::parse_distro_version_impl($path)
    };
    () => {
        crate::os::raw::parse_distro_version_impl("/etc/os-release")
    };
}

pub(crate) use parse_distro_version;

#[cfg(test)]
mod test {
    use super::{DistroKind, DistroVersion};

    macro_rules! distro_other {
        ($name: literal, $version: literal) => {
            DistroVersion {
                distro: DistroKind::Other($name.to_owned()),
                version: Some($version.to_owned()),
            }
        };
        ($name: literal) => {
            DistroVersion {
                distro: DistroKind::Other($name.to_owned()),
                version: None,
            }
        };
    }

    macro_rules! distro_known {
        ($name: tt, $version: literal) => {
            DistroVersion {
                distro: DistroKind::$name,
                version: Some($version.to_owned()),
            }
        };
        ($name: tt) => {
            DistroVersion {
                distro: DistroKind::$name,
                version: None,
            }
        };
    }

    #[test]
    fn test_parse_distro_version() {
        let version_mapping: &[(&str, DistroVersion)] = &[
            ("alma", distro_other!("AlmaLinux", "9.1 (Lime Lynx)")),
            ("alpine", distro_other!("Alpine Linux")),
            ("amazon", distro_other!("Amazon Linux", "2022")),
            (
                "antergos",
                distro_other!("Antergos Linux", "18.11-ISO-Rolling"),
            ),
            ("arch", distro_known!(Arch)),
            ("archarm", distro_other!("Arch Linux ARM")),
            ("arcolinux", distro_other!("ArcoLinux")),
            ("centos", distro_known!(CentOS, "8")),
            ("centos_stream", distro_other!("CentOS Stream", "8")),
            ("clearlinux", distro_other!("Clear Linux OS", "1")),
            ("clearos", distro_other!("ClearOS", "7 (Final)")),
            (
                "cumulus",
                distro_other!("Cumulus Linux", "Cumulus Linux 3.7.2"),
            ),
            ("debian", distro_known!(Debian, "11 (bullseye)")),
            ("elementary", distro_other!("elementary OS", "6 Odin")),
            ("endeavouros", distro_other!("EndeavourOS")),
            ("fedora", distro_known!(Fedora, "38 (Workstation Edition)")),
            ("gentoo", distro_known!(Gentoo)),
            ("ios_xr", distro_other!("IOS XR", "6.0.0.14I")),
            ("kali", distro_known!(Kali, "2018.4")),
            ("linuxmint", distro_known!(Mint, "19 (Tara)")),
            ("mageia", distro_other!("Mageia", "6")),
            ("manjaro", distro_known!(Manjaro)),
            ("manjaro-arm", distro_other!("Manjaro ARM")),
            ("nexus", distro_other!("Nexus", "7.0(BUILDER)")),
            (
                "nixos",
                distro_known!(NixOS, "18.09.1436.a7fd4310c0c (Jellyfish)"),
            ),
            ("opensuseleap", distro_other!("openSUSE Leap", "42.3")),
            ("oracle", distro_other!("Oracle Linux Server", "9.1")),
            ("pop_os", distro_known!(PopOS, "22.04 LTS")),
            ("rancheros", distro_other!("RancherOS", "v1.4.2")),
            (
                "raspbian",
                distro_other!("Raspbian GNU/Linux", "10 (buster)"),
            ),
            ("redhat", distro_known!(RedHat, "9.1 (Plow)")),
            ("rocky", distro_other!("Rocky Linux", "9.1 (Blue Onyx)")),
            (
                "scientific",
                distro_other!("Scientific Linux", "7.5 (Nitrogen)"),
            ),
            ("slackware", distro_known!(Slackware, "14.2")),
            ("sled", distro_other!("SLED", "15")),
            ("sles", distro_other!("SLES", "15-SP1")),
            ("sles_sap", distro_other!("SLES_SAP", "12.0.1")),
            (
                "ubuntu",
                distro_known!(Ubuntu, "22.04 LTS (Jammy Jellyfish)"),
            ),
            ("virtuozzo", distro_other!("Virtuozzo", "7.0.16")),
            ("xbian", distro_other!("XBian", "1.0 (knockout)")),
            ("xcp-ng", distro_other!("XCP-ng", "8.2.0")),
            ("xenserver", distro_other!("XenServer", "7.6.0")),
        ];
        for (distro, version) in version_mapping {
            let path = format!("./test_resources/os-releases/{}", distro);
            let result = parse_distro_version!(&path).unwrap();
            assert_eq!(result, *version);
        }
    }
}
