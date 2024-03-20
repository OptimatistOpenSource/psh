mod raw;

#[allow(dead_code)]
struct RpsDetails {
    dev: String,
    rx: Vec<String>,
    rps_cpus: Vec<Option<String>>,
    rps_flow_cnt: Vec<Option<String>>,
}
