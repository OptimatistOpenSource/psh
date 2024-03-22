use std::fs;

/// Parse `LINUX_VERSION_CODE` of `linux/version.h` to (major, patch_level, sub_level)
pub fn parse_linux_version_h(path: &str) -> (usize, usize, usize) {
    let first_line = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read {}", path))
        .lines()
        .next()
        .unwrap_or_else(|| panic!("No lines in {}", path))
        .to_string();
    let linux_version_code = first_line
        .split(' ')
        .nth(2)
        .unwrap_or_else(|| panic!("Invalid line {}", first_line))
        .to_string();
    let linux_version_code = linux_version_code.parse::<usize>().unwrap_or_else(|e| {
        panic!(
            "Invalid LINUX_VERSION_CODE `{}` ({})",
            linux_version_code, e
        )
    });

    let major = linux_version_code >> 16;
    let patch_lv = (linux_version_code & 65535) >> 8;
    let sub_lv = linux_version_code & 255;
    (major, patch_lv, sub_lv)
}
