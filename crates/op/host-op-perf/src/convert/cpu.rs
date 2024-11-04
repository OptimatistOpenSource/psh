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

use crate::convert::Wrap;

type FromT = crate::profiling::perf::config::Cpu;
type IntoT = perf_event_rs::config::Cpu;

impl From<&FromT> for Wrap<IntoT> {
    fn from(value: &FromT) -> Self {
        #[rustfmt::skip]
        let val = match value {
            FromT::Any   => IntoT::Any,
            FromT::Id(n) => IntoT::Id(*n) ,
        };
        Self(val)
    }
}
