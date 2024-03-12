use crate::convert::Wrap;
use perf_event_rs::config::{Cpu as RawCpu, Process as RawProcess};
use perf_event_rs::counting::{
    Config as RawConfig, Counter, CounterStat, ExtraConfig as RawExtraConfig,
};
use perf_event_rs::event::Event as RawEv;
use perf_event_rs::{config, EventScope as RawEvScope};
use std::io;

type FromT = crate::profiling::perf::config::Config;
type IntoT = perf_event_rs::counting::Config;

impl From<&FromT> for Wrap<IntoT> {
    fn from(value: &FromT) -> Self {
        let scopes: Vec<_> = value
            .scopes
            .iter()
            .map(|it| Wrap::<RawEvScope>::from(it).into_inner())
            .collect();
        let event = Wrap::<RawEv>::from(&value.event).into_inner();
        let extra_config = Wrap::<RawExtraConfig>::from(&value.extra_config).into_inner();

        Self(RawConfig::extra_new(&event, &scopes, &extra_config))
    }
}
