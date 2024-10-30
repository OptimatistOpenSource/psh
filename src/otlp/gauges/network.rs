use std::{borrow::Cow, time::Duration};

use opentelemetry::{
    metrics::{Meter, ObservableGauge},
    KeyValue,
};
use psh_system::network::NetworkHandle;

pub fn start(meter: Meter, interval: Duration) -> anyhow::Result<ObservableGauge<u64>> {
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
                gauge.observe(
                    status.recv_bytes,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_bytes"),
                    ],
                );
                gauge.observe(
                    status.recv_packets,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_packets"),
                    ],
                );
                gauge.observe(
                    status.recv_errs,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_errs"),
                    ],
                );
                gauge.observe(
                    status.recv_drop,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_drop"),
                    ],
                );
                gauge.observe(
                    status.recv_fifo,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_fifo"),
                    ],
                );
                gauge.observe(
                    status.recv_frame,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_frame"),
                    ],
                );
                gauge.observe(
                    status.recv_compressed,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_compressed"),
                    ],
                );
                gauge.observe(
                    status.recv_multicast,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "recv_multicast"),
                    ],
                );
                gauge.observe(
                    status.sent_bytes,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_bytes"),
                    ],
                );
                gauge.observe(
                    status.sent_packets,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_packets"),
                    ],
                );
                gauge.observe(
                    status.sent_errs,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_errs"),
                    ],
                );
                gauge.observe(
                    status.sent_drop,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_drop"),
                    ],
                );
                gauge.observe(
                    status.sent_fifo,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_fifo"),
                    ],
                );
                gauge.observe(
                    status.sent_colls,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_colls"),
                    ],
                );
                gauge.observe(
                    status.sent_carrier,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_carrier"),
                    ],
                );
                gauge.observe(
                    status.sent_compressed,
                    &[
                        KeyValue::new("interface", dev.clone()),
                        KeyValue::new("stat", "sent_compressed"),
                    ],
                );
            }
        })
        .try_init()?;
    Ok(gauge)
}
