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

use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    mem,
    path::Path,
    process::exit,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{daemon::DaemonConfig, otlp::config::OtlpConfig, services::config::RpcConfig};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PshConfig {
    #[serde(rename = "auth")]
    auth: AuthConfig,
    #[serde(rename = "component")]
    component_conf: ComponentConfig,
    daemon: DaemonConfig,
    pub remote: RemoteConfig,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub enable: bool,
    pub rpc: RpcConfig,
    #[serde(rename = "otlp")]
    pub otlp_conf: OtlpConfig,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AuthConfig {
    token: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ComponentConfig {
    path: String,
    args: Vec<String>,
}

impl ComponentConfig {
    #![allow(dead_code)]
    pub fn new(path: String, args: Vec<String>) -> Self {
        Self { path, args }
    }

    pub fn get_component_args(&mut self) -> Vec<String> {
        if self.path.is_empty() {
            tracing::error!("The config `component.path` must specify WASM path.");
            exit(1);
        }
        let mut args = Vec::with_capacity(1 + self.args.len());
        args.push(mem::take(&mut self.path));
        args.extend(mem::take(&mut self.args));

        args
    }
}

impl PshConfig {
    pub const DEFAULT_PATH: &'static str = "/etc/psh/config.toml";

    #[allow(dead_code)]
    pub fn new(
        component_conf: ComponentConfig,
        otlp_conf: OtlpConfig,
        daemon: DaemonConfig,
        rpc: RpcConfig,
        auth: AuthConfig,
        enable_remote: bool,
    ) -> Self {
        Self {
            component_conf,
            daemon,
            auth,
            remote: RemoteConfig {
                enable: enable_remote,
                rpc,
                otlp_conf,
            },
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

    pub fn otlp_conf(&mut self) -> OtlpConfig {
        mem::take(&mut self.remote.otlp_conf)
    }

    pub fn get_component_args(&mut self) -> Vec<String> {
        self.component_conf.get_component_args()
    }

    pub fn daemon(&self) -> &DaemonConfig {
        &self.daemon
    }

    pub fn rpc(&mut self) -> RpcConfig {
        mem::take(&mut self.remote.rpc)
    }

    pub fn take_token(&mut self) -> String {
        mem::take(&mut self.auth.token)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{AuthConfig, ComponentConfig, PshConfig},
        daemon::DaemonConfig,
        otlp::config::OtlpConfig,
        services::config::RpcConfig,
    };

    const CONFIG_STR: &str = r#"[auth]
token = ""

[component]
path = "cpu.wasm"
args = ["1", "2", "3"]

[otlp]
enable = true
endpoint = "http://localhost:4317"
protocol = "Grpc"

[otlp.timeout]
secs = 3
nanos = 0

[daemon]
pid_file = "/tmp/psh.pid"
stdout_file = "/tmp/psh.stdout"
stderr_file = "/tmp/psh.stderr"
working_directory = "/"

[rpc]
enable = true
addr = ""
duration = 1
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
            DaemonConfig::default(),
            RpcConfig::default(),
            AuthConfig::default(),
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
            DaemonConfig::default(),
            RpcConfig::default(),
            AuthConfig::default(),
        );
        cf.generate_config(TEST_CONF_PATH, true).unwrap();
        let conf = PshConfig::read_config(TEST_CONF_PATH).unwrap();
        assert_eq!(conf, cf);
    }
}
