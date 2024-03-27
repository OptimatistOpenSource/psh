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
