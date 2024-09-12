use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpcConfig {
    pub enable: bool,
    pub addr: String,
    pub duration: u64,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            enable: true,
            addr: String::new(),
            duration: 1,
        }
    }
}
