mod host;
mod raw;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct RpsQueue {
    name: String,
    // FIXME: better to use more expressive type than raw string
    cpus: Option<String>,
    flow_cnt: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct RpsDetails {
    dev: String,
    queues: Vec<RpsQueue>,
}
