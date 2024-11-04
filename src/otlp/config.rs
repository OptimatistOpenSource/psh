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

use std::time::Duration;

use opentelemetry_otlp::{ExportConfig, Protocol};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtlpConfig {
    enable: bool,
    endpoint: String,
    timeout: Duration,
    protocol: String,
}

impl Default for OtlpConfig {
    fn default() -> Self {
        Self {
            enable: true,
            endpoint: "http://localhost:4317".to_owned(),
            timeout: Duration::from_secs(3),
            protocol: "Grpc".to_owned(),
        }
    }
}

impl OtlpConfig {
    pub fn new(enable: bool, endpoint: String, timeout: Duration, protocol: String) -> Self {
        Self {
            enable,
            endpoint,
            timeout,
            protocol,
        }
    }

    pub fn enable(&self) -> bool {
        self.enable
    }
}

impl From<OtlpConfig> for ExportConfig {
    fn from(val: OtlpConfig) -> Self {
        let protocol = if val.protocol.eq_ignore_ascii_case("grpc") {
            Protocol::Grpc
        } else if val.protocol.eq_ignore_ascii_case("httpbinary") {
            Protocol::HttpBinary
        } else if val.protocol.eq_ignore_ascii_case("httpjson") {
            Protocol::HttpJson
        } else {
            panic!("Not support protocol, just support Grpc, HttpBinary, HttpJson")
        };
        Self {
            endpoint: val.endpoint,
            protocol,
            timeout: val.timeout,
        }
    }
}
impl From<&OtlpConfig> for ExportConfig {
    fn from(val: &OtlpConfig) -> Self {
        let protocol = if val.protocol.eq_ignore_ascii_case("grpc") {
            Protocol::Grpc
        } else if val.protocol.eq_ignore_ascii_case("httpbinary") {
            Protocol::HttpBinary
        } else if val.protocol.eq_ignore_ascii_case("httpjson") {
            Protocol::HttpJson
        } else {
            panic!("Not support protocol, just support Grpc, HttpBinary, HttpJson")
        };
        Self {
            endpoint: val.endpoint.clone(),
            protocol,
            timeout: val.timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use opentelemetry_otlp::Protocol;

    use super::*;

    const CONF_STR_SECS_NANO: &str = r#"enable = true
endpoint = "http://localhost:7878"
protocol = "HttpJson"

[timeout]
secs = 2
nanos = 20
"#;

    const CONF_STR_SECS: &str = r#"enable = true
endpoint = "http://localhost:4317"
protocol = "Grpc"

[timeout]
secs = 3
nanos = 0
"#;

    #[test]
    fn otlp_conf_to_str() {
        let cf = OtlpConfig::new(
            true,
            "http://localhost:4317".to_owned(),
            Duration::from_secs(3),
            "Grpc".to_owned(),
        );

        let s = toml::to_string_pretty(&cf).unwrap();
        assert_eq!(s, CONF_STR_SECS);

        let cf = OtlpConfig::new(
            true,
            "http://localhost:7878".to_owned(),
            Duration::new(2, 20),
            "HttpJson".to_owned(),
        );

        let s = toml::to_string_pretty(&cf).unwrap();
        assert_eq!(s, CONF_STR_SECS_NANO);
    }

    #[test]
    fn str_to_otlp_conf() {
        let conf_str: &str = r#"enable = true

endpoint = "http://localhost:4317"

protocol = "Grpc"

[timeout]

secs = 3

nanos = 0
"#;

        fn test_it(s: &str, cf: &OtlpConfig, export_config: &ExportConfig) {
            let ser_cf: OtlpConfig = toml::from_str(s).unwrap();

            assert_eq!(cf, &ser_cf);

            let test_export_conf: ExportConfig = cf.into();
            assert_eq!(export_config.endpoint, test_export_conf.endpoint);
            assert_eq!(export_config.timeout, test_export_conf.timeout);
            assert_eq!(export_config.protocol, test_export_conf.protocol);
        }

        let cf = OtlpConfig::new(
            true,
            "http://localhost:4317".to_owned(),
            Duration::from_secs(3),
            "Grpc".to_owned(),
        );
        let export_config = ExportConfig {
            endpoint: "http://localhost:4317".to_owned(),
            timeout: Duration::from_secs(3),
            protocol: Protocol::Grpc,
        };
        test_it(CONF_STR_SECS, &cf, &export_config);
        test_it(conf_str, &cf, &export_config);

        let cf = OtlpConfig::new(
            true,
            "http://localhost:7878".to_owned(),
            Duration::new(2, 20),
            "HttpJson".to_owned(),
        );
        let export_config = ExportConfig {
            endpoint: "http://localhost:7878".to_owned(),
            timeout: Duration::new(2, 20),
            protocol: Protocol::HttpJson,
        };
        test_it(CONF_STR_SECS_NANO, &cf, &export_config);
    }
}
