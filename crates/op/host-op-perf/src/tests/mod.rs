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
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.
use std::process::Command;

mod counter;
mod counter_group;

pub fn compile_component(project_path: &str) {
    let toml_path = format!("{}/Cargo.toml", project_path);

    // cargo clean --manifest-path $project_path/Cargo.toml
    let mut cmd = Command::new("cargo");
    cmd.args(["clean", "--manifest-path", &toml_path]);
    cmd.output().unwrap();

    // cargo component build --manifest-path $project_path/Cargo.toml
    let mut cmd = Command::new("cargo");
    cmd.args(["component", "build", "--manifest-path", &toml_path]);
    cmd.output().unwrap();
}
