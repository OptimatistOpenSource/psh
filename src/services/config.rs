use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
