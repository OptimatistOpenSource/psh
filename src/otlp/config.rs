use std::{
    fs::{create_dir_all, File},
    io::prelude::{Read, Write},
    path::Path,
    time::Duration,
};

use anyhow::{Context, Result};
use opentelemetry_otlp::{ExportConfig, Protocol};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
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
    pub const DEFAULT_PATH: &'static str = "/etc/psh/otlp.toml";

    pub fn new(enable: bool, endpoint: String, timeout: Duration, protocol: String) -> Self {
        Self {
            enable,
            endpoint,
            timeout,
            protocol,
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

        let otlp_conf: OtlpConfig = toml::from_str(&config_str)?;
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

    #[test]
    fn generate_config_work() {
        let cf = OtlpConfig::new(
            true,
            "http://localhost:4317".to_owned(),
            Duration::from_secs(3),
            "Grpc".to_owned(),
        );
        cf.generate_config(OtlpConfig::DEFAULT_PATH, false).unwrap();

        let cf = OtlpConfig::read_config(OtlpConfig::DEFAULT_PATH).unwrap();
        assert!(cf.enable);

        let export_config = ExportConfig {
            endpoint: "http://localhost:4317".to_owned(),
            timeout: Duration::from_secs(3),
            protocol: Protocol::Grpc,
        };
        let test_export_conf: ExportConfig = cf.into();

        assert_eq!(export_config.endpoint, test_export_conf.endpoint);
        assert_eq!(export_config.timeout, test_export_conf.timeout);
        assert_eq!(export_config.protocol, test_export_conf.protocol);
    }
}
