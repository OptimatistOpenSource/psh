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
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub struct PshConfig {
    component_args: Vec<String>,
}

impl PshConfig {
    pub const DEFAULT_PATH: &'static str = "/etc/psh/config.toml";

    #[allow(dead_code)]
    pub fn new(component_path: Vec<String>) -> Self {
        Self {
            component_args: component_path,
        }
    }

    /// The configuration must specify WASM path.
    pub fn check_vaild(&self) -> bool {
        !self.component_args.is_empty()
    }

    pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            Self::default().generate_config(Self::DEFAULT_PATH, false)?;
        }
        let mut config_str = String::new();
        let mut config_file = File::open(path).context("The config not exists.")?;
        config_file.read_to_string(&mut config_str)?;

        let otlp_conf: Self = toml::from_str(&config_str)?;
        Ok(otlp_conf)
    }

    /// When force set to true, it will forcefully overwrite the config file.
    pub fn generate_config<P: AsRef<Path>>(&self, path: P, force: bool) -> Result<()> {
        let path = path.as_ref();
        if !force && path.exists() {
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

    pub fn into_component_args(mut self) -> Vec<String> {
        std::mem::take(&mut self.component_args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONFIG_STR: &str = "component_args = [\"cpu.wasm\", \"1\", \"2\", \"3\"]\n";

    #[test]
    fn cf_str_convert_work() {
        let cf = PshConfig::new(vec![
            "cpu.wasm".to_owned(),
            "1".to_owned(),
            "2".to_owned(),
            "3".to_owned(),
        ]);
        let s = toml::to_string(&cf).unwrap();
        assert_eq!(s, CONFIG_STR);

        let str_to_cf: PshConfig = toml::from_str(CONFIG_STR).unwrap();
        assert_eq!(cf, str_to_cf);
    }
}
