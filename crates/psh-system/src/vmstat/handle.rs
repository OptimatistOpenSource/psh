use std::{collections::HashMap, sync::LazyLock, time::Duration};

use crate::{error::Result, utils::Handle};

static INFO_GLOBAL: LazyLock<Handle<HashMap<String, i64>>> =
    LazyLock::new(|| Handle::new(|| procfs::vmstat().map_err(Into::into)));

#[derive(Clone, Debug)]
pub struct VmstatHandle {
    stat: Handle<HashMap<String, i64>>,
}

impl Default for VmstatHandle {
    fn default() -> Self {
        Self {
            stat: INFO_GLOBAL.clone(),
        }
    }
}

impl VmstatHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stat<D: Into<Option<Duration>>>(&self, interval: D) -> Result<HashMap<String, i64>> {
        self.stat.get(interval.into())
    }
}
