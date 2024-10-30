use std::{borrow::Cow, time::Duration};

use opentelemetry::{
    metrics::{Meter, ObservableGauge},
    KeyValue,
};
use psh_system::interrupt::InterruptHandle;

pub fn start(meter: Meter, interval: Duration) -> anyhow::Result<ObservableGauge<u64>> {
    let interrupt = InterruptHandle::new();
    let gauge = meter
        .u64_observable_gauge("InterruptStat")
        .with_description("System profile interrupt statistics.")
        // .with_unit(Unit::new("KiB"))
        .with_callback(move |gauge| {
            let Ok(irqs) = interrupt.stat(Some(interval)) else {
                return;
            };
            for int in irqs {
                // TODO
                let desc = Cow::from(int.description);
                for (cpu, &cnt) in int.cpu_counts.iter().enumerate() {
                    gauge.observe(
                        cnt,
                        &[
                            KeyValue::new("desc", desc.clone()),
                            KeyValue::new("cpu", cpu as i64),
                        ],
                    )
                }
            }
        })
        .try_init()?;
    Ok(gauge)
}
