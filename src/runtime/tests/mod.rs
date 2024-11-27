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

use std::{fs, path::Path, process::Command};

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
    let Ok(engine) = engine() else {
        panic!();
    };
    let path = format!("./test_resources/profiling/{wasm}/target/wasm32-wasip1/debug/{wasm}.wasm");
    assert!(Path::new(&path).exists());
    let binary = fs::read(path).unwrap();
    assert!(engine.run(&binary, 60 * 1000).is_ok());
}

#[ignore]
#[test]
fn test_get_cpu_info() {
    let wasm = "test-get-cpu-info";
    compile_component(&format!("./test_resources/profiling/{wasm}"));
    test_wasm_component(wasm)
}

#[ignore]
#[test]
fn test_get_disks() {
    let wasm = "test-get-disks";
    compile_component(&format!("./test_resources/profiling/{wasm}"));
    test_wasm_component(wasm)
}

#[ignore]
#[test]
fn test_get_interrupts_info() {
    let wasm = "test-get-interrupts-info";
    compile_component(&format!("./test_resources/profiling/{wasm}"));
    test_wasm_component(wasm);
}

#[ignore]
#[test]
fn test_get_memory_info() {
    let wasm = "test-get-memory-info";
    compile_component(&format!("./test_resources/profiling/{wasm}"));
    test_wasm_component(wasm);
}

#[ignore]
#[test]
fn test_get_networks() {
    let wasm = "test-get-networks";
    compile_component(&format!("./test_resources/profiling/{wasm}"));
    test_wasm_component(wasm);
}

#[ignore]
#[test]
fn test_get_processes() {
    let wasm = "test-get-processes";
    compile_component(&format!("./test_resources/profiling/{wasm}"));
    test_wasm_component(wasm);
}

#[ignore]
#[test]
fn test_get_rps_info() {
    let wasm = "test-get-rps-info";
    compile_component(&format!("./test_resources/profiling/{wasm}"));
    test_wasm_component(wasm);
}

#[ignore]
#[test]
fn test_get_system_info() {
    let wasm = "test-get-system-info";
    compile_component(&format!("./test_resources/profiling/{wasm}"));
    test_wasm_component(wasm);
}

#[ignore]
#[test]
fn test_perf_counter() {
    let wasm = "test-perf-counter";
    compile_component(&format!("./test_resources/profiling/{wasm}"));
    test_wasm_component(wasm);
}

#[ignore]
#[test]
fn test_perf_counter_group() {
    let wasm = "test-perf-counter-group";
    compile_component(&format!("./test_resources/profiling/{wasm}"));
    test_wasm_component(wasm);
}
