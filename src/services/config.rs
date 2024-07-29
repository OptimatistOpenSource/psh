use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpcConfig {
    addr: String,
    token: String,
}

impl RpcConfig {
    pub fn addr(&self) -> &str {
        &self.addr
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            addr: String::new(),
            token: Default::default(),
        }
    }
}
