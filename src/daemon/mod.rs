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

use daemonize::Daemonize;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Daemon {
    pid_file: String,
    stdout_file: String,
    stderr_file: String,
    working_directory: String,
}

impl Default for Daemon {
    fn default() -> Self {
        Self {
            pid_file: "/tmp/psh.pid".to_owned(),
            stdout_file: "/tmp/psh.stdout".to_owned(),
            stderr_file: "/tmp/psh.stderr".to_owned(),
            working_directory: "/".to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaemonConfig {
    pid_file: String,
    stdout_file: String,
    stderr_file: String,
    working_directory: String,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            pid_file: "/tmp/psh.pid".to_owned(),
            stdout_file: "/tmp/psh.stdout".to_owned(),
            stderr_file: "/tmp/psh.stderr".to_owned(),
            working_directory: "/".to_owned(),
        }
    }
}

impl Daemon {
    pub fn new(config: DaemonConfig) -> Self {
        Self {
            pid_file: config.pid_file,
            stdout_file: config.stdout_file,
            stderr_file: config.stderr_file,
            working_directory: config.working_directory,
        }
    }

    /// run the process as daemon
    pub fn daemon(self) -> anyhow::Result<()> {
        let stdout = File::create(self.stdout_file)?;
        let stderr = File::create(self.stderr_file)?;

        let daemonize = Daemonize::new()
            .pid_file(self.pid_file)
            .chown_pid_file(true) // is optional, see `Daemonize` documentation
            .working_directory(self.working_directory)
            .user("root")
            .group("root")
            .umask(0o027) // Set umask, `0o027` by default.
            .stdout(stdout) // by default, stdout is redirect to `/tmp/psh.stdout`.
            .stderr(stderr); // by default, stderr is redirect to `/tmp/psh.stderr`.

        daemonize.start()?;

        Ok(())
    }
}
