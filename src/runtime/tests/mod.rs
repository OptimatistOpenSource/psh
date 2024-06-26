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

use std::{path::Path, process::Command};

use anyhow::Context as _;

use super::{PshEngine, PshEngineBuilder};

// FIXME(Chengdong Li): This function is no longer used in `cargo test` as
// host-op-perf requires root permission to run test. But there is often no `cargo`
// command in root user's $PATH.
#[allow(dead_code)]
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

fn engine() -> anyhow::Result<PshEngine> {
    PshEngineBuilder::new()
        .allow_perf_op(true)
        .allow_system_op(true)
        .build()
        .context("Failed to build PshEngine.")
}

fn test_wasm_component(wasm: &str) {
    let Ok(mut engine) = engine() else {
        panic!();
    };
    let path = format!("./test_resources/profiling/{wasm}/target/wasm32-wasi/debug/{wasm}.wasm");
    assert!(Path::new(&path).exists());
    assert!(engine.run(&path).is_ok());
}

#[test]
fn test_get_cpu_info() {
    test_wasm_component("test-get-cpu-info")
}

#[test]
fn test_get_disks() {
    test_wasm_component("test-get-disks")
}

#[test]
fn test_get_interrupts_info() {
    test_wasm_component("test-get-interrupts-info");
}

#[test]
fn test_get_memory_info() {
    test_wasm_component("test-get-memory-info");
}

#[test]
fn test_get_networks() {
    test_wasm_component("test-get-networks");
}

#[test]
fn test_get_processes() {
    test_wasm_component("test-get-processes");
}

#[test]
fn test_get_rps_info() {
    test_wasm_component("test-get-rps-info");
}

#[test]
fn test_get_system_info() {
    test_wasm_component("test-get-system-info");
}

#[test]
fn test_perf_counter() {
    test_wasm_component("test-perf-counter");
}

#[test]
fn test_perf_counter_group() {
    test_wasm_component("test-perf-counter-group");
}
