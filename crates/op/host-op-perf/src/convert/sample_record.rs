use std::collections::HashMap;
use std::ffi::CString;

use crate::profiling::perf::sampler::{AnonNormal, AuxOutputHwIdBody, WeightVars};

impl From<perf_event_rs::sampling::record::Record> for crate::profiling::perf::sampler::Record {
    fn from(value: perf_event_rs::sampling::record::Record) -> Self {
        Self {
            misc: value.misc,
            body: value.body.into(),
        }
    }
}

impl From<perf_event_rs::sampling::record::RecordBody>
    for crate::profiling::perf::sampler::RecordBody
{
    fn from(value: perf_event_rs::sampling::record::RecordBody) -> Self {
        match value {
            perf_event_rs::sampling::record::RecordBody::Mmap(mmap_body) => {
                Self::Mmap((*mmap_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Lost(lost_body) => {
                Self::Lost((*lost_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Comm(comm_body) => {
                Self::Comm((*comm_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Exit(exit_body) => {
                Self::Exit((*exit_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Throttle(throttle_body) => {
                Self::Throttle((*throttle_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Unthrottle(throttle_body) => {
                Self::Unthrottle((*throttle_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Fork(fork_body) => {
                Self::Fork((*fork_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Read(read_body) => {
                Self::Read((*read_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Sample(sample_body) => {
                Self::Sample((*sample_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Mmap2(mmap2_body) => {
                Self::Mmap2((*mmap2_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Aux(aux_body) => {
                Self::Aux((*aux_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::ItraceStart(intrace_start_body) => {
                Self::ItraceStart((*intrace_start_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::LostSamples(lost_samples_body) => {
                Self::LostSamples((*lost_samples_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Switch(switch_body) => {
                Self::Switch((*switch_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::SwitchCpuWide(switch_cpu_wide_body) => {
                Self::SwitchCpuWide((*switch_cpu_wide_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Namespaces(namespaces_body) => {
                Self::Namespaces((*namespaces_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Ksymbol(ksymbol_body) => {
                Self::Ksymbol((*ksymbol_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::BpfEvent(bpf_event_body) => {
                Self::BpfEvent((*bpf_event_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::Cgroup(cgroup_body) => {
                Self::Cgroup((*cgroup_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::TextPoke(text_poke_body) => {
                Self::TextPoke((*text_poke_body).into())
            }
            perf_event_rs::sampling::record::RecordBody::AuxOutputHwId(_body) => {
                Self::AuxOutputHwId(AuxOutputHwIdBody {})
            }
        }
    }
}

impl From<perf_event_rs::sampling::record::text_poke::Body>
    for crate::profiling::perf::sampler::TextPokeBody
{
    fn from(value: perf_event_rs::sampling::record::text_poke::Body) -> Self {
        Self {
            addr: value.addr,
            old_len: value.old_len,
            new_len: value.new_len,
            bytes: value.bytes,
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::cgroup::Body>
    for crate::profiling::perf::sampler::CgroupBody
{
    fn from(value: perf_event_rs::sampling::record::cgroup::Body) -> Self {
        Self {
            id: value.id,
            path: value.path.to_string_lossy().into_owned(),
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::bpf_event::Body>
    for crate::profiling::perf::sampler::BpfEventBody
{
    fn from(value: perf_event_rs::sampling::record::bpf_event::Body) -> Self {
        Self {
            type_: value.r#type,
            flags: value.flags,
            id: value.id,
            tag: value.tag.to_vec(),
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::ksymbol::Body>
    for crate::profiling::perf::sampler::KsymbolBody
{
    fn from(value: perf_event_rs::sampling::record::ksymbol::Body) -> Self {
        Self {
            addr: value.addr,
            len: value.len,
            ksym_type: value.ksym_type,
            flags: value.flags,
            name: value.name.to_string_lossy().into_owned(),
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::namespaces::Namespace>
    for crate::profiling::perf::sampler::Namespace
{
    fn from(value: perf_event_rs::sampling::record::namespaces::Namespace) -> Self {
        Self {
            dev: value.dev,
            inode: value.inode,
        }
    }
}

impl From<perf_event_rs::sampling::record::namespaces::Body>
    for crate::profiling::perf::sampler::NamespacesBody
{
    fn from(value: perf_event_rs::sampling::record::namespaces::Body) -> Self {
        Self {
            pid: value.pid,
            tid: value.tid,
            namespaces: value.namespaces.into_iter().map(Into::into).collect(),
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::switch_cpu_wide::Body>
    for crate::profiling::perf::sampler::SwitchCpuWideBody
{
    fn from(value: perf_event_rs::sampling::record::switch_cpu_wide::Body) -> Self {
        Self {
            next_prev_pid: value.next_prev_pid,
            next_prev_tid: value.next_prev_tid,
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::switch::Body>
    for crate::profiling::perf::sampler::SwitchBody
{
    fn from(value: perf_event_rs::sampling::record::switch::Body) -> Self {
        Self {
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::lost_samples::Body>
    for crate::profiling::perf::sampler::LostSamplesBody
{
    fn from(value: perf_event_rs::sampling::record::lost_samples::Body) -> Self {
        Self {
            lost: value.lost,
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::intrace_start::Body>
    for crate::profiling::perf::sampler::IntraceStartBody
{
    fn from(value: perf_event_rs::sampling::record::intrace_start::Body) -> Self {
        Self {
            pid: value.pid,
            tid: value.tid,
        }
    }
}

impl From<perf_event_rs::sampling::record::aux::Body> for crate::profiling::perf::sampler::AuxBody {
    fn from(value: perf_event_rs::sampling::record::aux::Body) -> Self {
        Self {
            aux_offset: value.aux_offset,
            aux_size: value.aux_size,
            flags: value.flags,
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::mmap2::AnonEnum>
    for crate::profiling::perf::sampler::AnonEnum
{
    fn from(value: perf_event_rs::sampling::record::mmap2::AnonEnum) -> Self {
        match value {
            perf_event_rs::sampling::record::mmap2::AnonEnum::Normal {
                maj,
                min,
                ino,
                ino_generation,
            } => Self::Normal(AnonNormal {
                maj,
                min,
                ino,
                ino_generation,
            }),
            perf_event_rs::sampling::record::mmap2::AnonEnum::BuildId(vec) => Self::BuildId(vec),
        }
    }
}

impl From<perf_event_rs::sampling::record::mmap2::Body>
    for crate::profiling::perf::sampler::Mmap2Body
{
    fn from(value: perf_event_rs::sampling::record::mmap2::Body) -> Self {
        Self {
            pid: value.pid,
            tid: value.tid,
            addr: value.addr,
            len: value.len,
            pgoff: value.pgoff,
            anon_enum: value.anon_enum.into(),
            prot: value.prot,
            flags: value.flags,
            filename: value.filename.to_string_lossy().into_owned(),
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::sample::Abi> for crate::profiling::perf::sampler::Abi {
    fn from(value: perf_event_rs::sampling::record::sample::Abi) -> Self {
        match value {
            perf_event_rs::sampling::record::sample::Abi::AbiNone => Self::AbiNone,
            perf_event_rs::sampling::record::sample::Abi::Abi32 => Self::Abi32,
            perf_event_rs::sampling::record::sample::Abi::Abi64 => Self::Abi64,
        }
    }
}

impl From<perf_event_rs::sampling::record::sample::AbiAndRegs>
    for crate::profiling::perf::sampler::AbiAndRegs
{
    fn from(value: perf_event_rs::sampling::record::sample::AbiAndRegs) -> Self {
        Self {
            abi: value.abi.into(),
            regs: value.regs,
        }
    }
}

impl From<perf_event_rs::sampling::record::sample::Weight>
    for crate::profiling::perf::sampler::Weight
{
    fn from(value: perf_event_rs::sampling::record::sample::Weight) -> Self {
        match value {
            perf_event_rs::sampling::record::sample::Weight::Full(f) => Self::Full(f),
            perf_event_rs::sampling::record::sample::Weight::Vars {
                var1_dw,
                var2_w,
                var3_w,
            } => Self::Vars(WeightVars {
                var1_dw,
                var2_w,
                var3_w,
            }),
        }
    }
}

impl From<perf_event_rs::sampling::record::sample::MemOp>
    for crate::profiling::perf::sampler::MemOp
{
    fn from(value: perf_event_rs::sampling::record::sample::MemOp) -> Self {
        match value {
            perf_event_rs::sampling::record::sample::MemOp::Na => Self::Na,
            perf_event_rs::sampling::record::sample::MemOp::Load => Self::Load,
            perf_event_rs::sampling::record::sample::MemOp::Store => Self::Store,
            perf_event_rs::sampling::record::sample::MemOp::Pfetch => Self::Pfetch,
            perf_event_rs::sampling::record::sample::MemOp::Exec => Self::Exec,
        }
    }
}

impl From<perf_event_rs::sampling::record::sample::MemLvl>
    for crate::profiling::perf::sampler::MemLvl
{
    fn from(value: perf_event_rs::sampling::record::sample::MemLvl) -> Self {
        match value {
            perf_event_rs::sampling::record::sample::MemLvl::Na => Self::Na,
            perf_event_rs::sampling::record::sample::MemLvl::Hit => Self::Hit,
            perf_event_rs::sampling::record::sample::MemLvl::Miss => Self::Miss,
            perf_event_rs::sampling::record::sample::MemLvl::L1 => Self::L1,
            perf_event_rs::sampling::record::sample::MemLvl::Lfb => Self::Lfb,
            perf_event_rs::sampling::record::sample::MemLvl::L2 => Self::L2,
            perf_event_rs::sampling::record::sample::MemLvl::L3 => Self::L3,
            perf_event_rs::sampling::record::sample::MemLvl::LocRam => Self::LocRam,
            perf_event_rs::sampling::record::sample::MemLvl::RemRam1 => Self::RemRam1,
            perf_event_rs::sampling::record::sample::MemLvl::RemRam2 => Self::RemRam2,
            perf_event_rs::sampling::record::sample::MemLvl::RemCce1 => Self::RemCce1,
            perf_event_rs::sampling::record::sample::MemLvl::RemCce2 => Self::RemCce2,
            perf_event_rs::sampling::record::sample::MemLvl::Io => Self::Io,
            perf_event_rs::sampling::record::sample::MemLvl::Unc => Self::Unc,
        }
    }
}

impl From<perf_event_rs::sampling::record::sample::MemSnoop>
    for crate::profiling::perf::sampler::MemSnoop
{
    fn from(value: perf_event_rs::sampling::record::sample::MemSnoop) -> Self {
        match value {
            perf_event_rs::sampling::record::sample::MemSnoop::Na => Self::Na,
            perf_event_rs::sampling::record::sample::MemSnoop::None => Self::None,
            perf_event_rs::sampling::record::sample::MemSnoop::Hit => Self::Hit,
            perf_event_rs::sampling::record::sample::MemSnoop::Miss => Self::Miss,
            perf_event_rs::sampling::record::sample::MemSnoop::Hitm => Self::Hitm,
        }
    }
}
impl From<perf_event_rs::sampling::record::sample::MemLock>
    for crate::profiling::perf::sampler::MemLock
{
    fn from(value: perf_event_rs::sampling::record::sample::MemLock) -> Self {
        match value {
            perf_event_rs::sampling::record::sample::MemLock::Na => Self::Na,
            perf_event_rs::sampling::record::sample::MemLock::Locked => Self::Locked,
        }
    }
}

impl From<perf_event_rs::sampling::record::sample::MemDtlb>
    for crate::profiling::perf::sampler::MemDtlb
{
    fn from(value: perf_event_rs::sampling::record::sample::MemDtlb) -> Self {
        match value {
            perf_event_rs::sampling::record::sample::MemDtlb::Na => Self::Na,
            perf_event_rs::sampling::record::sample::MemDtlb::Hit => Self::Hit,
            perf_event_rs::sampling::record::sample::MemDtlb::Miss => Self::Miss,
            perf_event_rs::sampling::record::sample::MemDtlb::L1 => Self::L1,
            perf_event_rs::sampling::record::sample::MemDtlb::L2 => Self::L2,
            perf_event_rs::sampling::record::sample::MemDtlb::Wk => Self::Wk,
            perf_event_rs::sampling::record::sample::MemDtlb::Os => Self::Os,
        }
    }
}

impl From<perf_event_rs::sampling::record::sample::DataSrc>
    for crate::profiling::perf::sampler::DataSrc
{
    fn from(value: perf_event_rs::sampling::record::sample::DataSrc) -> Self {
        Self {
            mem_op: value.mem_op.into(),
            mem_lvl: value.mem_lvl.into(),
            mem_snoop: value.mem_snoop.into(),
            mem_lock: value.mem_lock.into(),
            mem_dtlb: value.mem_dtlb.into(),
        }
    }
}

impl From<perf_event_rs::sampling::record::sample::Body>
    for crate::profiling::perf::sampler::SampleBody
{
    fn from(value: perf_event_rs::sampling::record::sample::Body) -> Self {
        Self {
            sample_id: value.sample_id,
            ip: value.ip,
            pid: value.pid,
            tid: value.tid,
            time: value.time,
            addr: value.addr,
            id: value.id,
            stream_id: value.stream_id,
            cpu: value.cpu,
            period: value.period,
            v: value.v.map(Into::into),
            ips: value.ips,
            data_raw: value.data_raw,
            abi_and_regs_user: value.abi_and_regs_user.map(Into::into),
            data_stack_user: value.data_stack_user,
            weight: value.weight.map(Into::into),
            data_src: value.data_src.map(Into::into),
            transaction: value.transaction,
            abi_and_regs_intr: value.abi_and_regs_intr.map(Into::into),
            phys_addr: value.phys_addr,
            cgroup: value.cgroup,
            data_page_size: value.data_page_size,
            code_page_size: value.code_page_size,
        }
    }
}

impl From<perf_event_rs::sampling::MemberCount> for crate::profiling::perf::sampler::MemberCount {
    fn from(value: perf_event_rs::sampling::MemberCount) -> Self {
        Self {
            event_count: value.event_count,
            #[cfg(feature = "linux-6.0")]
            event_lost: value.event_lost,
            #[cfg(not(feature = "linux-6.0"))]
            event_lost: 0,
        }
    }
}

impl From<perf_event_rs::sampling::SamplerGroupStat>
    for crate::profiling::perf::sampler::SamplerGroupStat
{
    fn from(value: perf_event_rs::sampling::SamplerGroupStat) -> Self {
        Self {
            time_enabled: value.time_enabled,
            time_running: value.time_running,
            member_counts: value
                .member_counts
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl From<perf_event_rs::sampling::record::read::Body>
    for crate::profiling::perf::sampler::ReadBody
{
    fn from(value: perf_event_rs::sampling::record::read::Body) -> Self {
        Self {
            pid: value.pid,
            tid: value.tid,
            values: value.values.into(),
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::fork::Body>
    for crate::profiling::perf::sampler::ForkBody
{
    fn from(value: perf_event_rs::sampling::record::fork::Body) -> Self {
        Self {
            pid: value.pid,
            ppid: value.ppid,
            tid: value.tid,
            ptid: value.ptid,
            time: value.time,
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::throttle::Body>
    for crate::profiling::perf::sampler::ThrottleBody
{
    fn from(value: perf_event_rs::sampling::record::throttle::Body) -> Self {
        Self {
            time: value.time,
            id: value.id,
            stream_id: value.stream_id,
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::exit::Body>
    for crate::profiling::perf::sampler::ExitBody
{
    fn from(value: perf_event_rs::sampling::record::exit::Body) -> Self {
        Self {
            pid: value.pid,
            ppid: value.ppid,
            tid: value.tid,
            ptid: value.ptid,
            time: value.time,
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::comm::Body>
    for crate::profiling::perf::sampler::CommBody
{
    fn from(value: perf_event_rs::sampling::record::comm::Body) -> Self {
        Self {
            pid: value.pid,
            tid: value.tid,
            comm: value.comm.to_string_lossy().into_owned(),
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::SampleId> for crate::profiling::perf::sampler::SampleId {
    fn from(value: perf_event_rs::sampling::record::SampleId) -> Self {
        Self {
            pid: value.pid,
            tid: value.tid,
            time: value.time,
            id1: value.id_1,
            stream_id: value.stream_id,
            cpu: value.cpu,
            id2: value.id_2,
        }
    }
}

impl From<perf_event_rs::sampling::record::lost::Body>
    for crate::profiling::perf::sampler::LostBody
{
    fn from(value: perf_event_rs::sampling::record::lost::Body) -> Self {
        Self {
            id: value.id,
            lost: value.lost,
            sample_id: value.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::mmap::Body>
    for crate::profiling::perf::sampler::MmapBody
{
    fn from(value: perf_event_rs::sampling::record::mmap::Body) -> Self {
        Self {
            pid: value.pid,
            tid: value.tid,
            addr: value.addr,
            len: value.len,
            pgoff: value.pgoff,
            filename: value.filename.to_string_lossy().into_owned(),
        }
    }
}

impl From<perf_event_rs::sampling::SamplerStat> for crate::profiling::perf::sampler::SamplerStat {
    fn from(value: perf_event_rs::sampling::SamplerStat) -> Self {
        Self {
            event_id: value.event_id,
            event_count: value.event_count,
            #[cfg(feature = "linux-6.0")]
            event_lost: value.event_lost,
            #[cfg(not(feature = "linux-6.0"))]
            event_lost: 0,
            time_enabled: value.time_enabled,
            time_running: value.time_running,
        }
    }
}
