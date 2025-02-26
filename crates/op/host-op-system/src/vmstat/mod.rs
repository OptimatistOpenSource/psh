use std::time::Duration;

use crate::{SysCtx, profiling::system::vmstat};

impl vmstat::Host for SysCtx {
    fn stat(&mut self, interval_ms: u64) -> Result<Vec<(String, i64)>, String> {
        self.vmstat
            .stat(Duration::from_millis(interval_ms))
            .map(Vec::from_iter)
            .map_err(|e| e.to_string())
    }
}
