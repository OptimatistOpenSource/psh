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

type FromT = crate::profiling::perf::config::ExtraConfig;
type IntoT = perf_event_rs::counting::ExtraConfig;

impl TryFrom<&FromT> for Wrap<IntoT> {
    type Error = super::Error;

    fn try_from(value: &FromT) -> Result<Self, Self::Error> {
        #[cfg(not(feature = "linux-5.13"))]
        if value.remove_on_exec {
            return Err(Self::Error::UnsupportedOption(
                "ExtraConfig.remove_on_exec = true".to_string(),
            ));
        }

        #[rustfmt::skip]
        let val = IntoT {
            pinned:         value.pinned,
            exclusive:      value.exclusive,
            inherit:        value.inherit,
            inherit_stat:   value.inherit_stat,
            inherit_thread: value.inherit_thread,
            enable_on_exec: value.enable_on_exec,
            #[cfg(feature = "linux-5.13")]
            remove_on_exec: value.remove_on_exec,
        };
        Ok(Self(val))
    }
}
