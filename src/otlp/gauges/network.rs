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

use opentelemetry::{KeyValue, metrics::ObservableGauge};
use psh_system::network::NetworkHandle;

impl super::super::Otlp {
    pub fn net_gauges(&self) -> ObservableGauge<u64> {
        let interval = self.interval;
        let host = self.host.clone();
        let network = NetworkHandle::new();

        let gauge = self
            .meter
            .u64_observable_gauge("NetworkStat")
            .with_description("System profile network statistics.")
            .with_callback(move |gauge| {
                let Ok(stat) = network.stat(Some(interval)) else {
                    return;
                };
                for (dev, status) in stat {
                    let speed = Self::net_dev_speed(&dev).unwrap_or(0).into();

                    macro_rules! gauges {
                        ($($stat:ident,)+) => {
                            [
                                (
                                    speed,
                                    [
                                        KeyValue::new("interface", dev.clone()),
                                        KeyValue::new("stat", "speed"),
                                    ]
                                ),
                            $((
                                status.$stat,
                                [
                                    KeyValue::new("interface", dev.clone()),
                                    KeyValue::new("stat", stringify!($stat)),
                                ],
                            ),)*
                            ]
                        };
                    }
                    let gauges = gauges![
                        recv_bytes,
                        recv_packets,
                        recv_errs,
                        recv_drop,
                        recv_fifo,
                        recv_frame,
                        recv_compressed,
                        recv_multicast,
                        sent_bytes,
                        sent_packets,
                        sent_errs,
                        sent_drop,
                        sent_fifo,
                        sent_colls,
                        sent_carrier,
                        sent_compressed,
                    ];
                    gauges.into_iter().for_each(|(m, [kv1, kv2])| {
                        let a = [KeyValue::new("host", host.clone()), kv1, kv2];
                        gauge.observe(m, &a);
                    })
                }
            })
            .build();
        gauge
    }
}
