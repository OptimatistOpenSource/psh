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
