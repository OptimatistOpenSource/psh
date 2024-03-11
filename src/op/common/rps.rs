use super::RpsDetails;
use std::fs;

#[allow(dead_code)]
fn do_parse_all_rps(path: &str) -> Vec<RpsDetails> {
    let mut rps: Vec<RpsDetails> = Vec::new();
    if let Ok(device) = fs::read_dir(path) {
        for dev in device.flatten() {
            let dev_path = dev.path();
            let current_path = dev_path.join("queues");
            if let Ok(rx_list) = fs::read_dir(current_path) {
                let mut rps_instance = RpsDetails {
                    dev: dev_path.file_name().unwrap().to_string_lossy().into_owned(),
                    rx: Vec::new(),
                    rps_cpus: Vec::new(),
                    rps_flow_cnt: Vec::new(),
                };
                for rx in rx_list.flatten() {
                    let rx_path = rx.path();
                    let rps_cpu_path = rx_path.join("rps_cpus");
                    let rps_flow_cnt_path = rx_path.join("rps_flow_cnt");
                    rps_instance
                        .rx
                        .push(rx_path.file_name().unwrap().to_string_lossy().into_owned());
                    rps_instance.rps_cpus.push(
                        fs::read_to_string(&rps_cpu_path)
                            .ok()
                            .map(|s| s.trim().to_string()),
                    );
                    rps_instance.rps_flow_cnt.push(
                        fs::read_to_string(&rps_flow_cnt_path)
                            .ok()
                            .map(|s| s.trim().to_string()),
                    );
                }
                rps.push(rps_instance);
            }
        }
    }
    rps
}

#[allow(unused_macros)]
macro_rules! parse_rps {
    ($path:expr) => {
        super::do_parse_all_rps($path)
    };
    () => {
        super::do_parse_all_rps("/sys/class/net/")
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_parse_rps() {
        let mut rps_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rps_path.push("test_resources/arch/x86_64/intel/net");
        let result = do_parse_all_rps(rps_path.to_str().unwrap());
        assert_eq!(result[0].dev, "lo");
        assert_eq!(result[0].rx[0], "rx-0");
        assert_eq!(result[0].rps_cpus[0], Some("00000".to_string()));
        assert_eq!(result[0].rps_flow_cnt[0], Some("0".to_string()));

        assert_eq!(result[1].dev, "enp3s0");
        assert_eq!(result[1].rx[0], "rx-0");
        assert_eq!(result[1].rps_cpus[0], Some("00000".to_string()));
        assert_eq!(result[1].rps_flow_cnt[0], Some("0".to_string()));

        assert_eq!(result[2].dev, "wlo1");
        assert_eq!(result[2].rx[0], "rx-0");
        assert_eq!(result[2].rps_cpus[0], Some("00000".to_string()));
        assert_eq!(result[2].rps_flow_cnt[0], Some("0".to_string()));
    }
}
