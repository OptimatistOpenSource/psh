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
use std::ops::Not;
use std::process::Command;

fn main() {
    let _ = fs::remove_file("src/bindings.rs");
    let mut cmd = Command::new("wit-bindgen");
    cmd.args(["rust", "--out-dir", "src", "wit"]);

    let output = cmd
        .output()
        .unwrap_or_else(|it| panic!("Failed to generate bindings: \n{}", it));
    if output.status.success().not() {
        panic!(
            "Failed to generate bindings: \n{}",
            String::from_utf8(output.stderr).unwrap()
        );
    }
}
