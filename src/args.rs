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

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Config file
    /// └╴Will be generated if it does not exist
    #[arg(short, long)]
    #[arg(value_name = "PATH")]
    #[arg(default_value = "/etc/psh/config.toml")]
    #[arg(verbatim_doc_comment)]
    pub config: String,

    /// Run as daemon
    /// └╴WASM binary and it's args are read from the config file in this mode
    ///   (Auto applies --wasm-from-daemon-config)
    #[arg(short, long)]
    #[arg(verbatim_doc_comment)]
    pub daemon: bool,

    /// Use WASM from daemon config
    /// └╴This is useful when run as systemd daemon and you don't want to config
    ///   the WASM in the service file
    #[arg(long)]
    #[clap(visible_alias = "wdc")]
    #[arg(verbatim_doc_comment)]
    pub wasm_from_daemon_config: bool,

    /// WASM binary followed with arguments
    /// └╴e.g. /path/to/your.wasm foo bar baz
    ///   Invalid in daemon mode (--daemon)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    #[arg(verbatim_doc_comment)]
    pub wasm_with_args: Option<Vec<String>>,
}
