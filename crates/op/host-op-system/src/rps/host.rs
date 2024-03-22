use crate::profiling::system::rps::{self, RpsInfo as GuestRpsInfo, RpsQueue as GuestRpsQueue};

use crate::rps::{raw::parse_rps, RpsDetails as HostRpsInfo, RpsQueue as HostRpsQueue};

use crate::SysCtx;

impl From<&HostRpsQueue> for GuestRpsQueue {
    fn from(value: &HostRpsQueue) -> Self {
        Self {
            name: value.name.clone(),
            cpus: value.cpus.clone(),
            flow_cnt: value.flow_cnt.clone(),
        }
    }
}

impl From<&HostRpsInfo> for GuestRpsInfo {
    fn from(value: &HostRpsInfo) -> Self {
        Self {
            device: value.dev.clone(),
            queues: value.queues.iter().map(|q| q.into()).collect(),
        }
    }
}

impl rps::Host for SysCtx {
    fn get_rps_info(&mut self) -> wasmtime::Result<Vec<GuestRpsInfo>> {
        Ok(parse_rps!().iter().map(|rps| rps.into()).collect())
    }
}
