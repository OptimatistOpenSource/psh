mod host;
mod raw;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct KernelVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub struct DistroVersion {
    pub(crate) distro: DistroKind,
    pub(crate) version: Option<String>,
}
