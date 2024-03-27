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
use crate::convert::Wrap;

const _: () = {
    type FromT = perf_event_rs::counting::CounterStat;
    type IntoT = crate::profiling::perf::counter::CounterStat;

    impl From<&FromT> for Wrap<IntoT> {
        fn from(value: &FromT) -> Self {
            #[rustfmt::skip]
            let val = IntoT {
                event_id:     value.event_id,
                event_count:  value.event_count,
                time_enabled: value.time_enabled,
                time_running: value.time_running,
            };
            Self(val)
        }
    }
};

const _: () = {
    type FromT = perf_event_rs::counting::CounterGroupStat;
    type IntoT = crate::profiling::perf::counter_group::CounterGroupStat;

    impl From<&FromT> for Wrap<IntoT> {
        fn from(value: &FromT) -> Self {
            #[rustfmt::skip]
            let val = IntoT {
                time_enabled:  value.time_enabled,
                time_running:  value.time_running,
                member_counts: value.member_counts.clone().into_iter().collect(),
            };
            Self(val)
        }
    }
};
