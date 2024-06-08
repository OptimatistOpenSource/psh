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
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.

#[rustfmt::skip]
#[allow(dead_code)]
mod imports;

use std::{collections::HashMap, error::Error};

use imports::profiling::system::disk::{self, DiskStat};

pub struct Usage {
    bps: f64,
    ops: f64,
}

pub struct DiskUsage {
    dev: String,
    read: Usage,
    write: Usage,
}

fn usage(pre: &DiskStat, post: &DiskStat, dur: std::time::Duration) -> DiskUsage {
    let ms = dur.as_millis() as f64;
    let read_bps = (post.read.sectors - pre.read.sectors) as f64 * 512.0 * 1000.0 / ms;
    let write_bps = (post.write.sectors - pre.write.sectors) as f64 * 512.0 * 1000.0 / ms;
    let rps = (post.read.operations - pre.read.operations) as f64 * 1000.0 / ms;
    let wps = (post.write.operations - pre.write.operations) as f64 * 1000.0 / ms;
    DiskUsage {
        dev: pre.name.clone(),
        read: Usage {
            bps: read_bps,
            ops: rps,
        },
        write: Usage {
            bps: write_bps,
            ops: wps,
        },
    }
}

fn differential(pre: &Vec<DiskStat>, post: &Vec<DiskStat>, dur: std::time::Duration) {
    let pre: HashMap<_, _> = pre.iter().map(|d| (d.name.clone(), d)).collect();
    let post: HashMap<_, _> = post.iter().map(|d| (d.name.clone(), d)).collect();
    let mut usages: Vec<_> = pre
        .iter()
        .filter_map(|(name, &pre_stat)| match post.get(name) {
            Some(&post_stat) => Some(usage(pre_stat, post_stat, dur)),
            None => None,
        })
        .collect();

    usages.sort_unstable_by(|lhs, rhs| rhs.write.bps.total_cmp(&lhs.write.bps));

    for du in usages {
        println!(
            "{:16}: Read {:8.2} KiB/s {:6.2} O/s, Write: {:8.2} KiB/s {:6.2} O/s",
            du.dev,
            du.read.bps / 1024.0,
            du.read.ops,
            du.write.bps / 1024.0,
            du.write.ops
        );
    }
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut pre_now = std::time::Instant::now();
    let dur = std::time::Duration::from_secs(1);
    let mut pre = disk::stat(dur.as_millis() as u64)?;

    // only iterate 3 times for testing purpose.
    for _ in 0..3 {
        std::thread::sleep(dur);
        let post_now = std::time::Instant::now();
        let post = disk::stat(dur.as_millis() as u64)?;
        differential(&pre, &post, post_now - pre_now);
        pre = post;
        pre_now = post_now;
    }

    Ok(())
}
