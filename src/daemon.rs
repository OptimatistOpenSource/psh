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

use std::fs::File;

use anyhow::Result;
use daemonize::Daemonize;

use crate::config::{DaemonConfig, DaemonWasmConfig};

/// run the process as daemon
pub fn spawn_daemon(cfg: DaemonConfig) -> Result<()> {
    let stdout = File::create(cfg.stdout)?;
    let stderr = File::create(cfg.stderr)?;

    let daemonize = Daemonize::new()
        .pid_file(cfg.pid_file)
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory(cfg.workdir)
        .user("root")
        .group("root")
        .umask(0o027) // Set umask, `0o027` by default.
        .stdout(stdout) // by default, stdout is redirect to `/tmp/psh.stdout`.
        .stderr(stderr); // by default, stderr is redirect to `/tmp/psh.stderr`.

    daemonize.start()?;

    Ok(())
}

pub fn get_daemon_wasm_args(cfg: DaemonWasmConfig) -> Option<Vec<String>> {
    if !cfg.enable {
        return None;
    }
    let mut vec = Vec::with_capacity(cfg.args.len() + 1);
    vec.push(cfg.path);
    vec.extend(cfg.args);
    Some(vec)
}
