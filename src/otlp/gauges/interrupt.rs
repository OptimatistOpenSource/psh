use std::{borrow::Cow, time::Duration};

use opentelemetry::{
    metrics::{Meter, ObservableGauge},
    KeyValue,
};
use psh_system::interrupt::InterruptHandle;

pub fn start(
    instance_id: Option<String>,
    meter: Meter,
    interval: Duration,
) -> anyhow::Result<ObservableGauge<u64>> {
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
                    if let Some(instance_id) = &instance_id {
                        let a = [
                            KeyValue::new("instance_id", instance_id.clone()),
                            KeyValue::new("desc", desc.clone()),
                            KeyValue::new("cpu", cpu as i64),
                        ];
                        gauge.observe(cnt, &a)
                    } else {
                        let a = [
                            KeyValue::new("desc", desc.clone()),
                            KeyValue::new("cpu", cpu as i64),
                        ];
                        gauge.observe(cnt, &a)
                    };
                }
            }
        })
        .try_init()?;
    Ok(gauge)
}
