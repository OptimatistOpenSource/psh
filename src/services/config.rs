use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LuoxiaoConfig {
    addr: String,
    token: String,
}

impl LuoxiaoConfig {
    pub fn addr(&self) -> &str {
        &self.addr
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl Default for LuoxiaoConfig {
    fn default() -> Self {
        Self {
            addr: "www.optimatist.com".to_owned(),
            token: Default::default(),
        }
    }
}
