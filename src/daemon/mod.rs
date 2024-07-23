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
