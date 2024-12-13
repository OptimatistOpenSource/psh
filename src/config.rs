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

use std::{fs, path::Path};

use anyhow::Result;
use serde::Deserialize;

const TEMPLATE: &str = include_str!("../doc/config.toml");

#[derive(Deserialize)]
pub struct Config {
    pub daemon: DaemonConfig,
    pub remote: RemoteConfig,
}

#[derive(Clone, Deserialize)]
pub struct DaemonConfig {
    pub pid_file: String,
    pub stdout: String,
    pub stderr: String,
    pub workdir: String,
    pub wasm: DaemonWasmConfig,
}

#[derive(Clone, Deserialize)]
pub struct DaemonWasmConfig {
    pub enable: bool,
    pub path: String,
    pub args: Vec<String>,
}

#[derive(Deserialize)]
pub struct RemoteConfig {
    pub token: String,
    pub rpc: RpcConfig,
    pub otlp: OtlpConfig,
}

#[derive(Deserialize)]
pub struct RpcConfig {
    pub enable: bool,
    pub addr: String,
    /// in seconds
    pub heartbeat_interval: u64,
    pub instance_id_file: String,
}

#[derive(Deserialize)]
pub struct OtlpConfig {
    pub enable: bool,
    pub addr: String,
}

pub fn read_or_gen<P>(path: P) -> Result<Config>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if !path.exists() {
        fs::write(path, TEMPLATE)?;
    }
    let cfg = fs::read_to_string(path)?;
    let cfg: Config = toml::from_str(&cfg)?;
    Ok(cfg)
}

#[test]
fn parse_config_template() {
    toml::from_str::<Config>(TEMPLATE).unwrap();
}
