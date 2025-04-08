use std::{sync::LazyLock, time::Duration};

use crate::{error::Result, utils::Handle};

use super::{
    GpuInfo, GpuStats,
    raw::{gpu_info, gpu_stats},
};

static GPU_INFO_HANDLE: LazyLock<Handle<GpuInfo>> = LazyLock::new(|| Handle::new(gpu_info));
static GPU_STATS_HANDLE: LazyLock<Handle<Vec<GpuStats>>> = LazyLock::new(|| Handle::new(gpu_stats));

#[derive(Clone, Debug)]
pub struct NvidiaHandle {
    info: Handle<GpuInfo>,
    stat: Handle<Vec<GpuStats>>,
}

impl NvidiaHandle {
    pub fn new() -> Self {
        Self {
            info: GPU_INFO_HANDLE.clone(),
            stat: GPU_STATS_HANDLE.clone(),
        }
    }

    pub fn info(&self) -> Result<GpuInfo> {
        self.info.get(None)
    }

    pub fn stat<D>(&self, interval: D) -> Result<Vec<GpuStats>>
    where
        D: Into<Option<Duration>>,
    {
        self.stat.get(interval.into())
    }
}

impl Default for NvidiaHandle {
    fn default() -> Self {
        Self::new()
    }
}
