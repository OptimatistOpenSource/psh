mod host;
mod raw;

use crate::cpu::CpuMask;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct RpsQueue {
    name: String,
    // FIXME: better to use more expressive type than raw string
    cpus: Option<CpuMask>,
    flow_cnt: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct RpsDetails {
    dev: String,
    queues: Vec<RpsQueue>,
}
