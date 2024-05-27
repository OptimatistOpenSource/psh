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

#[rustfmt::skip]
#[allow(dead_code)]
mod imports;

use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ProcessUsage {
    cpu: f64,
    mem: u64,
    wri: f64,
    rea: f64,
}

use imports::profiling::system::process::{self, ProcessStat};

fn differential(pre: &ProcessStat, post: &ProcessStat, ms: u64) -> ProcessUsage {
    ProcessUsage {
        cpu: ((post.stime + post.utime) - (pre.stime + pre.utime)) as f64 / ms as f64,
        mem: post.memory_usage,
        wri: (post.written_bytes - pre.written_bytes) as f64 * 1000.0 / ms as f64,
        rea: (post.read_bytes - pre.read_bytes) as f64 * 1000.0 / ms as f64,
    }
}

fn intersection<'p>(
    pre: &'p [ProcessStat],
    post: &'p [ProcessStat],
) -> HashMap<i32, (&'p ProcessStat, &'p ProcessStat)> {
    let pre: HashMap<i32, &ProcessStat> = pre.iter().map(|p| (p.pid, p)).collect();
    let post: HashMap<i32, &ProcessStat> = post.iter().map(|p| (p.pid, p)).collect();
    pre.iter()
        .filter_map(|(&pid, &pre_stat)| {
            post.get(&pid)
                .map(|&post_stat| (pid, (pre_stat, post_stat)))
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let interval = std::time::Duration::from_secs(1);
    let mut pre = process::all(interval.as_millis() as u64)?;
    for _ in 0..3 {
        std::thread::sleep(interval);
        let post = process::all(interval.as_millis() as u64)?;

        let common = intersection(&pre, &post);
        let mut procs: Vec<_> = common
            .values()
            .map(|&(pre, post)| (pre, differential(pre, post, 1000)))
            .collect();

        procs.sort_unstable_by(|(_, lhs), (_, rhs)| rhs.mem.cmp(&lhs.mem));
        // procs.sort_unstable_by(|(_, lhs), (_, rhs)| rhs.mem.total_cmp(&lhs.mem));
        for (proc, usage) in procs.iter().take(5) {
            let name: String = proc.name.chars().take(15).collect();
            println!(
                "{:6} [{:15}] -> Cpu: {:6.2}%  |  Mem: {:6.2}%  |  Read: {:7.2}KiB/s  |  Write: {:7.2}KiB/s",
                proc.pid,
                name,
                usage.cpu * 100.0 / 12.0,
                usage.mem as f64 / 1024.0 / 1024.0 / 1024.0 / 16.0 * 100.0,
                usage.rea / 1024.0,
                usage.wri / 1024.0
            );
        }
        println!();
        pre = post;
    }

    Ok(())
}
