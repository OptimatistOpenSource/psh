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

use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use psh_system::process::{ProcState, Process};
use wasmtime::component::Resource;

use crate::profiling::system::process::{
    self, ProcessStat as GuestProcessStat, ProcessState as GuestProcessState,
};
use crate::SysCtx;

impl From<&ProcState> for GuestProcessState {
    fn from(value: &ProcState) -> Self {
        match value {
            ProcState::Running => Self::Running,
            ProcState::Sleeping => Self::Sleeping,
            ProcState::Waiting => Self::Waiting,
            ProcState::Zombie => Self::Zombie,
            ProcState::Stopped => Self::Stopped,
            ProcState::Tracing => Self::Tracing,
            ProcState::Dead => Self::Dead,
            ProcState::Wakekill => Self::Wakekill,
            ProcState::Waking => Self::Waking,
            ProcState::Parked => Self::Parked,
            ProcState::Idle => Self::Idle,
        }
    }
}

impl From<ProcState> for GuestProcessState {
    fn from(value: ProcState) -> Self {
        match value {
            ProcState::Running => Self::Running,
            ProcState::Sleeping => Self::Sleeping,
            ProcState::Waiting => Self::Waiting,
            ProcState::Zombie => Self::Zombie,
            ProcState::Stopped => Self::Stopped,
            ProcState::Tracing => Self::Tracing,
            ProcState::Dead => Self::Dead,
            ProcState::Wakekill => Self::Wakekill,
            ProcState::Waking => Self::Waking,
            ProcState::Parked => Self::Parked,
            ProcState::Idle => Self::Idle,
        }
    }
}

fn path_to_str(path: PathBuf) -> String {
    path.to_string_lossy().to_string()
}

fn env_to_tuple((name, val): (OsString, OsString)) -> (String, String) {
    let to_str = |os_str: OsString| os_str.to_string_lossy().to_string();
    (to_str(name), to_str(val))
}

fn envs_to_vec(vars: HashMap<OsString, OsString>) -> Vec<(String, String)> {
    vars.into_iter().map(env_to_tuple).collect::<Vec<_>>()
}

impl process::HostProcess for SysCtx {
    fn pid(&mut self, self_: Resource<Arc<Process>>) -> wasmtime::Result<i32> {
        let process = self.table.get(&self_)?;
        Ok(process.pid)
    }

    fn cmd(
        &mut self,
        self_: Resource<Arc<Process>>,
    ) -> wasmtime::Result<Result<Vec<String>, String>> {
        let proc = self.table.get(&self_)?;
        Ok(proc.cmdline().map_err(|err| err.to_string()))
    }

    fn exe(&mut self, self_: Resource<Arc<Process>>) -> wasmtime::Result<Result<String, String>> {
        let proc = self.table.get(&self_)?;
        Ok(proc.exe().map_err(|err| err.to_string()).map(path_to_str))
    }

    fn environ(
        &mut self,
        self_: Resource<Arc<Process>>,
    ) -> wasmtime::Result<Result<Vec<(String, String)>, String>> {
        let proc = self.table.get(&self_)?;
        Ok(proc
            .environ()
            .map_err(|err| err.to_string())
            .map(envs_to_vec))
    }

    fn cwd(&mut self, self_: Resource<Arc<Process>>) -> wasmtime::Result<Result<String, String>> {
        let proc = self.table.get(&self_)?;
        Ok(proc.cwd().map_err(|err| err.to_string()).map(path_to_str))
    }

    fn root(&mut self, self_: Resource<Arc<Process>>) -> wasmtime::Result<Result<String, String>> {
        let proc = self.table.get(&self_)?;
        Ok(proc.root().map_err(|err| err.to_string()).map(path_to_str))
    }

    fn user_id(&mut self, self_: Resource<Arc<Process>>) -> wasmtime::Result<Result<u32, String>> {
        let proc = self.table.get(&self_)?;
        Ok(proc.uid().map_err(|err| err.to_string()))
    }

    fn drop(&mut self, rep: Resource<Arc<Process>>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl process::Host for SysCtx {
    fn all(&mut self, interval_ms: u64) -> wasmtime::Result<Result<Vec<GuestProcessStat>, String>> {
        // don't return top level Error unless it's not our fault
        // example: self.table.(push/get/delete)
        let procs = match self.process.all(Some(Duration::from_millis(interval_ms))) {
            Ok(procs) => procs,
            Err(err) => return Ok(Err(err.to_string())),
        };

        let processes: Vec<_> = procs
            .into_iter()
            .filter_map(|proc| {
                let (Ok(stat), Ok(io), Ok(mem)) = (proc.stat(), proc.io(), proc.statm()) else {
                    return None;
                };
                let Ok(state) = stat.state() else {
                    return None;
                };
                Some((proc, stat, io, mem, state))
            })
            .collect();

        let processes: Vec<_> = processes
            .into_iter()
            .map(|(proc, stat, io, mem, ref state)| {
                let (pid, parent_id) = (proc.pid, stat.ppid);
                match self.table.push(proc) {
                    Ok(proc) => Ok(GuestProcessStat {
                        pid,
                        proc,
                        name: stat.comm,
                        utime: stat.utime * 1000 / self.system.tick_per_sec,
                        stime: stat.stime * 1000 / self.system.tick_per_sec,
                        cutime: stat.cutime * 1000 / self.system.tick_per_sec as i64,
                        cstime: stat.cstime * 1000 / self.system.tick_per_sec as i64,
                        priority: stat.priority,
                        nice: stat.nice,
                        num_threads: stat.num_threads,
                        start_time: stat.starttime * 1000 / self.system.tick_per_sec,
                        state: state.into(),
                        written_bytes: io.write_bytes,
                        read_bytes: io.read_bytes,
                        memory_usage: mem.resident * self.system.page_size,
                        virtual_memory_usage: mem.size * self.system.page_size,
                        parent_id,
                    }),
                    Err(err) => Err(err),
                }
            })
            .collect::<Result<_, _>>()?; // failure of this collect means self.table.push failed, so we give up

        Ok(Ok(processes))
    }

    fn current(&mut self) -> wasmtime::Result<Result<Resource<Arc<Process>>, String>> {
        let proc = match self.process.myself() {
            Ok(proc) => Ok(self.table.push(proc)?),
            Err(err) => Err(err.to_string()),
        };
        Ok(proc)
    }
}
