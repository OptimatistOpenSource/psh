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
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.
use std::{fs, ops::Not};

use crate::cpu::CpuMask;

use super::{RpsDetails, RpsQueue};

fn parse_queue_impl(dir: fs::DirEntry) -> Option<RpsQueue> {
    let rx_path = dir.path();
    let Some(rx_name) = rx_path.file_name() else {
        return None;
    };
    let rx_name = rx_name.to_string_lossy().into_owned();
    if rx_name.starts_with("rx").not() {
        return None;
    }
    let cpu = fs::read_to_string(rx_path.join("rps_cpus"));
    let flow = fs::read_to_string(rx_path.join("rps_flow_cnt"));
    Some(RpsQueue {
        name: rx_name,
        cpus: cpu
            .ok()
            .and_then(|mask| CpuMask::from_str(mask.trim()).ok()),
        flow_cnt: flow.ok().and_then(|s| s.trim().parse().ok()),
    })
}

fn parse_device_impl(dir: fs::DirEntry) -> Option<RpsDetails> {
    let dev_path = dir.path();
    let cur_path = dev_path.join("queues");
    match (dev_path.file_name(), fs::read_dir(cur_path)) {
        (Some(dev_name), Ok(rx_list)) => {
            let dev = dev_name.to_string_lossy().into_owned();
            let queues: Vec<_> = rx_list
                .filter_map(|rx| rx.ok().and_then(parse_queue_impl))
                .collect();
            Some(RpsDetails { dev, queues })
        }
        _ => None,
    }
}

pub(crate) fn parse_rps_impl(path: &str) -> Vec<RpsDetails> {
    fs::read_dir(path).map_or(vec![], |folder| {
        folder
            .filter_map(|dev| dev.ok().and_then(parse_device_impl))
            .collect()
    })
}

macro_rules! parse_rps {
    ($path:expr) => {
        crate::rps::raw::parse_rps_impl($path)
    };
    () => {
        crate::rps::raw::parse_rps_impl("/sys/class/net/")
    };
}

pub(crate) use parse_rps;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{parse_rps_impl, RpsDetails, RpsQueue};
    use crate::cpu::CpuMask;

    #[test]
    fn test_parse_rps() {
        let mut rps_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rps_path.push("test_resources/arch/x86_64/intel/net");
        let result = parse_rps_impl(rps_path.to_str().unwrap());

        assert_eq!(result.len(), 3);

        let rps_lo = result.iter().find(|rps| rps.dev == "lo");
        assert_eq!(
            rps_lo,
            Some(&RpsDetails {
                dev: "lo".to_string(),
                queues: vec![RpsQueue {
                    name: "rx-0".to_string(),
                    cpus: Some(CpuMask(vec![false; 20])),
                    flow_cnt: Some(0),
                }]
            })
        );

        let rps_enp3s0 = result.iter().find(|rps| rps.dev == "enp3s0");
        assert_eq!(
            rps_enp3s0,
            Some(&RpsDetails {
                dev: "enp3s0".to_string(),
                queues: vec![RpsQueue {
                    name: "rx-0".to_string(),
                    cpus: Some(CpuMask(vec![false; 20])),
                    flow_cnt: Some(0),
                }]
            })
        );

        let rps_wlo1 = result.iter().find(|rps| rps.dev == "wlo1");
        assert_eq!(
            rps_wlo1,
            Some(&RpsDetails {
                dev: "wlo1".to_string(),
                queues: vec![RpsQueue {
                    name: "rx-0".to_string(),
                    cpus: Some(CpuMask(vec![false; 20])),
                    flow_cnt: Some(0),
                }]
            })
        );
    }
}
