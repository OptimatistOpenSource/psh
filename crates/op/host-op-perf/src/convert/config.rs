use crate::convert::{Error, Wrap};
use perf_event_rs::config::{Cpu as RawCpu, Process as RawProcess};
use perf_event_rs::counting::{
    Config as RawConfig, Counter, CounterStat, ExtraConfig as RawExtraConfig,
};
use perf_event_rs::event::Event as RawEv;
use perf_event_rs::{config, EventScope as RawEvScope};
use std::io;

type FromT = crate::profiling::perf::config::Config;
type IntoT = perf_event_rs::counting::Config;

impl TryFrom<&FromT> for Wrap<IntoT> {
    type Error = Error;

    fn try_from(value: &FromT) -> Result<Self, Self::Error> {
        let scopes: Vec<_> = value
            .scopes
            .iter()
            .map(|it| Wrap::<RawEvScope>::from(it).into_inner())
            .collect();
        let event = Wrap::<RawEv>::try_from(&value.event)?.into_inner();
        let extra_config = Wrap::<RawExtraConfig>::try_from(&value.extra_config)?.into_inner();

        Ok(Self(RawConfig::extra_new(&event, &scopes, &extra_config)))
    }
}
