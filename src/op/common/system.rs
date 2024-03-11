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
            if key == "VERSION_ID" {
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
    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_kernel_version() {}

    #[test]
    fn test_parse_os_version() {}
}
