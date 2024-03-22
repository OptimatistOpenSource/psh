use std::{fs, ops::Not};

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
        cpus: cpu.ok().map(|s| s.trim().to_string()),
        flow_cnt: flow.ok().map(|s| s.trim().to_string()),
    })
}

fn parse_device_impl(dir: fs::DirEntry) -> Option<RpsDetails> {
    let dev_path = dir.path();
    let cur_path = dev_path.join("queues");
    match (dev_path.file_name(), fs::read_dir(cur_path)) {
        (Some(dev_name), Ok(rx_list)) => {
            let dev = dev_name.to_string_lossy().into_owned();
            let queues: Vec<_> = rx_list
                .filter_map(|rx| match rx {
                    Ok(rx) => parse_queue_impl(rx),
                    Err(_) => None,
                })
                .collect();
            Some(RpsDetails { dev, queues })
        }
        _ => None,
    }
}

pub(crate) fn parse_rps_impl(path: &str) -> Vec<RpsDetails> {
    let Ok(folder) = fs::read_dir(path) else {
        return vec![];
    };
    folder
        .filter_map(|dev| match dev {
            Ok(dev) => parse_device_impl(dev),
            Err(_) => None,
        })
        .collect()
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
    use super::{parse_rps_impl, RpsDetails, RpsQueue};
    use std::path::PathBuf;

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
                    cpus: Some("00000".to_string()),
                    flow_cnt: Some("0".to_string()),
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
                    cpus: Some("00000".to_string()),
                    flow_cnt: Some("0".to_string()),
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
                    cpus: Some("00000".to_string()),
                    flow_cnt: Some("0".to_string()),
                }]
            })
        );
    }
}
