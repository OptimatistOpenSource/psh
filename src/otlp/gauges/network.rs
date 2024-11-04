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

use std::{borrow::Cow, time::Duration};

use opentelemetry::{
    metrics::{Meter, ObservableGauge},
    KeyValue,
};
use psh_system::network::NetworkHandle;

pub fn start(
    token: String,
    meter: Meter,
    interval: Duration,
) -> anyhow::Result<ObservableGauge<u64>> {
    let network = NetworkHandle::new();
    let gauge = meter
        .u64_observable_gauge("NetworkStat")
        .with_description("System profile network statistics.")
        // .with_unit(Unit::new("KiB"))
        .with_callback(move |gauge| {
            let Ok(stat) = network.stat(Some(interval)) else {
                return;
            };
            for (dev, status) in stat {
                // TODO
                let dev = Cow::from(dev);

                let gauges = [
                    (
                        status.recv_bytes,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "recv_bytes"),
                        ],
                    ),
                    (
                        status.recv_packets,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "recv_packets"),
                        ],
                    ),
                    (
                        status.recv_errs,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "recv_errs"),
                        ],
                    ),
                    (
                        status.recv_drop,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "recv_drop"),
                        ],
                    ),
                    (
                        status.recv_fifo,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "recv_fifo"),
                        ],
                    ),
                    (
                        status.recv_frame,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "recv_frame"),
                        ],
                    ),
                    (
                        status.recv_compressed,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "recv_compressed"),
                        ],
                    ),
                    (
                        status.recv_multicast,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "recv_multicast"),
                        ],
                    ),
                    (
                        status.sent_bytes,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "sent_bytes"),
                        ],
                    ),
                    (
                        status.sent_packets,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "sent_packets"),
                        ],
                    ),
                    (
                        status.sent_errs,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "sent_errs"),
                        ],
                    ),
                    (
                        status.sent_drop,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "sent_drop"),
                        ],
                    ),
                    (
                        status.sent_fifo,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "sent_fifo"),
                        ],
                    ),
                    (
                        status.sent_colls,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "sent_colls"),
                        ],
                    ),
                    (
                        status.sent_carrier,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "sent_carrier"),
                        ],
                    ),
                    (
                        status.sent_compressed,
                        [
                            KeyValue::new("interface", dev.clone()),
                            KeyValue::new("stat", "sent_compressed"),
                        ],
                    ),
                ];

                gauges.into_iter().for_each(|(m, [kv1, kv2])| {
                    let a = [KeyValue::new("token", token.clone()), kv1, kv2];
                    gauge.observe(m, &a);
                })
            }
        })
        .try_init()?;
    Ok(gauge)
}
