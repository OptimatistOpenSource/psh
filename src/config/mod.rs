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
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::mem;
use std::path::Path;
use std::process::exit;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::otlp::config::OtlpConfig;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct PshConfig {
    #[serde(rename = "component")]
    component_conf: ComponentConfig,
    #[serde(rename = "otlp")]
    otlp_conf: OtlpConfig,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub struct ComponentConfig {
    component_path: String,
    component_args: Vec<String>,
}

impl ComponentConfig {
    #![allow(dead_code)]
    pub fn new(component_path: String, component_args: Vec<String>) -> Self {
        Self {
            component_path,
            component_args,
        }
    }

    pub fn get_component_args(&mut self) -> Vec<String> {
        if self.component_path.is_empty() {
            eprintln!("The config `component_path` must specify WASM path.");
            exit(1);
        }
        let mut component_args = Vec::with_capacity(1 + self.component_args.len());
        component_args.push(mem::take(&mut self.component_path));
        component_args.extend(mem::take(&mut self.component_args));

        component_args
    }
}

impl PshConfig {
    pub const DEFAULT_PATH: &'static str = "/etc/psh/config.toml";

    #[allow(dead_code)]
    pub fn new(component_conf: ComponentConfig, otlp_conf: OtlpConfig) -> Self {
        Self {
            component_conf,
            otlp_conf,
        }
    }

    pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            Self::default().generate_config(Self::DEFAULT_PATH, false)?;
        }
        let mut config_str = String::new();
        let mut config_file = File::open(path).context("The config not exists.")?;
        config_file.read_to_string(&mut config_str)?;

        let conf: Self = toml::from_str(&config_str)?;
        Ok(conf)
    }

    /// When overwrite set to true, it will overwrite the config file.
    pub fn generate_config<P: AsRef<Path>>(&self, path: P, overwrite: bool) -> Result<()> {
        let path = path.as_ref();
        if !overwrite && path.exists() {
            return Ok(());
        }
        create_dir_all(path.parent().expect("no parent directory"))?;

        let s = toml::to_string(self)?;

        let mut f = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        f.write_all(s.as_bytes())?;
        Ok(())
    }

    pub fn otlp_conf(&self) -> &OtlpConfig {
        &self.otlp_conf
    }

    pub fn get_component_args(&mut self) -> Vec<String> {
        self.component_conf.get_component_args()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONFIG_STR: &str = r#"[component]
component_path = "cpu.wasm"
component_args = ["1", "2", "3"]

[otlp]
enable = true
endpoint = "http://localhost:4317"
protocol = "Grpc"

[otlp.timeout]
secs = 3
nanos = 0
"#;

    const TEST_CONF_PATH: &str = "./target/config.toml";

    #[test]
    fn conf_str_convert_work() {
        let cf = PshConfig::new(
            ComponentConfig::new(
                "cpu.wasm".to_owned(),
                vec!["1".to_owned(), "2".to_owned(), "3".to_owned()],
            ),
            OtlpConfig::default(),
        );
        let s = toml::to_string(&cf).unwrap();
        assert_eq!(s, CONFIG_STR);

        let str_to_cf: PshConfig = toml::from_str(CONFIG_STR).unwrap();
        assert_eq!(cf, str_to_cf);
    }

    #[test]
    fn generate_config_work() {
        let cf = PshConfig::new(
            ComponentConfig::new(
                "cpu.wasm".to_owned(),
                vec!["1".to_owned(), "2".to_owned(), "3".to_owned()],
            ),
            OtlpConfig::default(),
        );
        cf.generate_config(TEST_CONF_PATH, true).unwrap();
        let conf = PshConfig::read_config(TEST_CONF_PATH).unwrap();
        assert_eq!(conf, cf);
    }
}
