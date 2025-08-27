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
use crate::profiling::perf;
use crate::profiling::perf::config::{
    ClockId, SampleIpSkid, SampleRecordFields, Wakeup, WeightRepr,
};

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

impl From<crate::profiling::perf::config::SamplingExtraConfig>
    for perf_event_rs::sampling::ExtraConfig
{
    fn from(val: crate::profiling::perf::config::SamplingExtraConfig) -> Self {
        Self {
            pinned: val.pinned,
            exclusive: val.exclusive,
            mmap_data: val.mmap_data,
            comm: val.comm,
            comm_exec: val.comm_exec,
            inherit: val.inherit,
            inherit_stat: val.inherit_stat,
            inherit_thread: val.inherit_thread,
            build_id: val.build_id,
            enable_on_exec: val.enable_on_exec,
            remove_on_exec: val.remove_on_exec,
            include_callchain_kernel: val.include_callchain_kernel,
            include_callchain_user: val.include_callchain_user,
            clockid: val.clockid.map(Into::into),
            precise_ip: val.precise_ip.into(),
            wakeup: val.wakeup.into(),
            sigtrap: val.sigtrap,
            sample_record_fields: val.sample_record_fields.into(),
            extra_record_types: val.extra_record_types.into_iter().map(Into::into).collect(),
            extra_record_with_sample_id: val.extra_record_with_sample_id,
        }
    }
}

type HostExtraRecord = perf_event_rs::sampling::ExtraRecord;

impl From<crate::profiling::perf::config::ExtraRecord> for HostExtraRecord {
    fn from(val: crate::profiling::perf::config::ExtraRecord) -> Self {
        match val {
            perf::config::ExtraRecord::Mmap => Self::Mmap,
            perf::config::ExtraRecord::Mmap2 => Self::Mmap2,
            perf::config::ExtraRecord::ContextSwitch => Self::ContextSwitch,
            perf::config::ExtraRecord::Namespaces => Self::Namespaces,
            perf::config::ExtraRecord::Ksymbol => Self::Ksymbol,
            perf::config::ExtraRecord::BpfEvent => Self::BpfEvent,
            perf::config::ExtraRecord::Cgroup => Self::Cgroup,
            perf::config::ExtraRecord::TextPoke => Self::TextPoke,
            perf::config::ExtraRecord::ForkAndExit => Self::ForkAndExit,
        }
    }
}

type HostWeightRepr = perf_event_rs::sampling::record::sample::WeightRepr;

impl From<WeightRepr> for HostWeightRepr {
    fn from(val: WeightRepr) -> Self {
        match val {
            WeightRepr::Full => Self::Full,
            WeightRepr::Vars => Self::Vars,
        }
    }
}

type HostSampleRecordFields = perf_event_rs::sampling::SampleRecordFields;

impl From<SampleRecordFields> for HostSampleRecordFields {
    fn from(val: SampleRecordFields) -> Self {
        HostSampleRecordFields {
            sample_id: val.sample_id,
            ip: val.ip,
            pid_and_tid: val.pid_and_tid,
            time: val.time,
            addr: val.addr,
            id: val.id,
            stream_id: val.stream_id,
            cpu: val.cpu,
            period: val.period,
            v: val.v,
            ips: val.ips,
            data_raw: val.data_raw,
            abi_and_regs_user: val.abi_and_regs_user,
            data_stack_user: val.data_stack_user,
            weight: val.weight.map(Into::into),
            data_src: val.data_src,
            transaction: val.transaction,
            abi_and_regs_intr: val.abi_and_regs_intr,
            phys_addr: val.phys_addr,
            cgroup: val.cgroup,
            data_page_size: val.data_page_size,
            code_page_size: val.code_page_size,
        }
    }
}

type HostWakeup = perf_event_rs::sampling::Wakeup;

impl From<Wakeup> for HostWakeup {
    fn from(val: Wakeup) -> Self {
        match val {
            Wakeup::Events(e) => Self::Events(e),
            Wakeup::Watermark(w) => Self::Watermark(w),
        }
    }
}

type HostSampleIpSkid = perf_event_rs::sampling::SampleIpSkid;

impl From<SampleIpSkid> for HostSampleIpSkid {
    fn from(val: SampleIpSkid) -> Self {
        match val {
            SampleIpSkid::Arbitrary => Self::Arbitrary,
            SampleIpSkid::Constant => Self::Constant,
            SampleIpSkid::TryZero => Self::TryZero,
            SampleIpSkid::Zero => Self::Zero,
        }
    }
}

type HostClockId = perf_event_rs::sampling::ClockId;

impl From<ClockId> for HostClockId {
    fn from(val: ClockId) -> Self {
        match val {
            ClockId::Monotonic => Self::Monotonic,
            ClockId::MonotonicRaw => Self::MonotonicRaw,
            ClockId::RealTime => Self::RealTime,
            ClockId::BootTime => Self::BootTime,
            ClockId::Tai => Self::Tai,
        }
    }
}
