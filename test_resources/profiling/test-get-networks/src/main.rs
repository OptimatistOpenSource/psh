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

use std::collections::HashMap;

use imports::profiling::system::network::{self, NetworkStat};

fn differential(pre: &Vec<NetworkStat>, post: &Vec<NetworkStat>, dur: std::time::Duration) {
    let pre: HashMap<_, _> = pre.iter().map(|net| (net.name.clone(), net)).collect();
    let post: HashMap<_, _> = post.iter().map(|net| (net.name.clone(), net)).collect();
    let bytes = |pr: u64, po: u64, du: std::time::Duration| {
        (po - pr) as f64 / 1024.0 * 1000.0 / du.as_millis() as f64
    };
    let packets = |pr: u64, po: u64, du: std::time::Duration| {
        (po - pr) as f64 * 1000.0 / du.as_millis() as f64
    };
    let mut data: Vec<_> = pre
        .iter()
        .filter_map(|(name, pre_stat)| {
            let Some(post_stat) = post.get(name) else {
                return None;
            };
            let recv_byte_per_sec = bytes(pre_stat.recv_bytes, post_stat.recv_bytes, dur);
            let recv_packets_per_sec = packets(pre_stat.recv_packets, post_stat.recv_packets, dur);
            let sent_byte_per_sec = bytes(pre_stat.sent_bytes, post_stat.sent_bytes, dur);
            let sent_packets_per_sec = packets(pre_stat.sent_packets, post_stat.sent_packets, dur);
            Some((
                recv_byte_per_sec,
                format!(
                    "{:20} => recv: {:8.2} KiB/s {:5.2} Pck/s, sent: {:8.2} KiB/s {:5.2} Pck/s",
                    name,
                    recv_byte_per_sec,
                    recv_packets_per_sec,
                    sent_byte_per_sec,
                    sent_packets_per_sec
                ),
            ))
        })
        .collect();
    data.sort_unstable_by(|lhs, rhs| rhs.0.total_cmp(&lhs.0));
    for (_, s) in data {
        println!("{}", s);
    }
    println!();
}

fn main() {
    let duration = std::time::Duration::from_secs(1);
    let mut pre_now = std::time::Instant::now();
    let mut pre_networks = network::stat().unwrap();
    for _ in 0..3 {
        std::thread::sleep(duration);
        let now = std::time::Instant::now();
        let networks = network::stat().unwrap();
        differential(&pre_networks, &networks, now - pre_now);
        pre_now = now;
        pre_networks = networks;
    }
}
