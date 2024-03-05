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

mod cmd;
mod file;

mod args;
mod infra;
mod op;
mod resources;
mod runtime;
mod security;
mod services;

use args::Args;
use clap::Parser;
use cmd::run_program;
use file::write_file;
use netdata_plugin::collector::Collector;
use netdata_plugin::{Chart, Dimension};
use std::path::Path;

use wasmtime_wasi::preview2::WasiCtxBuilder;

fn main() {
    fn run_script(script_path: &str) {
        let log_path = format!("{}.log", script_path);
        let log_path = Path::new(&log_path);

        let result = run_program("bash", ["-f", script_path])
            .map(|bytes| String::from_utf8(bytes.clone()).unwrap());

        match result {
            Ok(output) => {
                println!("{} output: {}", script_path, output);
                write_file(log_path, output.as_bytes())
            }
            Err(error) => {
                println!("{} error: {}", script_path, error);
                write_file(log_path, error.to_string().as_bytes())
            }
        }
        .unwrap()
    }

    // detect if we were ran as netdata plugin
    let netdata_plugin = std::env::var("NETDATA_HOST_PREFIX").is_ok();

    if netdata_plugin {
        // format of netdata external plugin command line parameter
        //     # external_plugin update_freq command_options...
        //
        // `update_freq` controls the granularity of the external plugin
        // `command_options...` allows giving additional command line options to the plugin.
        //
        // see https://learn.netdata.cloud/docs/contributing/external-plugins
        //
        // for debug purpose, you can save command line parameters to a file, for example:
        // ```rust
        //  use std::io::Write;
        //  let args: Args = Args::parse();
        //  let mut args_save_file = File::create("/tmp/psh_nd_argv.txt").unwrap();
        //  args_save_file.write_all(format!("{}", args.netdata_freq).as_bytes()).unwrap();
        // ```
        let mut args: Args = Args::parse();
        args.netdata_plugin = Some(true);

        // FIXME(Chengdong Li) This is demostrate code for CMCC project.
        let mut writer = std::io::stdout();
        let mut c = Collector::new(&mut writer);

        c.add_chart(&Chart {
            type_id: "arm64.PMU",
            name: "Arm64 PMU ",
            title: "Arm64 CPU PMU Statistics",
            units: "counts/s",
            familiy: "hardware",
            ..Default::default()
        })
        .unwrap();

        c.add_dimension(
            "arm64.PMU",
            &Dimension {
                id: "instructions",
                ..Default::default()
            },
        )
        .unwrap();
        c.add_dimension(
            "arm64.PMU",
            &Dimension {
                id: "cycles",
                ..Default::default()
            },
        )
        .unwrap();
        c.add_dimension(
            "arm64.PMU",
            &Dimension {
                id: "loads",
                ..Default::default()
            },
        )
        .unwrap();
        c.add_dimension(
            "arm64.PMU",
            &Dimension {
                id: "stores",
                ..Default::default()
            },
        )
        .unwrap();
        c.add_dimension(
            "arm64.PMU",
            &Dimension {
                id: "SIMD_instrs",
                ..Default::default()
            },
        )
        .unwrap();
        c.add_dimension(
            "arm64.PMU",
            &Dimension {
                id: "Integer_instrs",
                ..Default::default()
            },
        )
        .unwrap();
        c.add_dimension(
            "arm64.PMU",
            &Dimension {
                id: "Floats_instrs",
                ..Default::default()
            },
        )
        .unwrap();

        loop {
            c.prepare_value("arm64.PMU", "instructions", 100000)
                .unwrap();
            c.prepare_value("arm64.PMU", "cycles", (100000.0 * 1.1) as i64)
                .unwrap();
            c.prepare_value("arm64.PMU", "loads", (100000.0 * 0.29) as i64)
                .unwrap();
            c.prepare_value("arm64.PMU", "stores", (100000.0 * 0.15) as i64)
                .unwrap();
            c.prepare_value("arm64.PMU", "SIMD_instrs", (100000.0 * 0.15) as i64)
                .unwrap();
            c.prepare_value("arm64.PMU", "Integer_instrs", (100000.0 * 0.15) as i64)
                .unwrap();
            c.prepare_value("arm64.PMU", "Floats_instrs", (100000.0 * 0.15) as i64)
                .unwrap();
            c.commit_chart("arm64.PMU").unwrap();

            std::thread::sleep(std::time::Duration::from_secs(args.netdata_freq));
        }
    } else {
        let args: Args = Args::parse();

        if let Some(path) = args.install {
            run_script(&path);
        }

        if let Some(path) = args.get_sysinfo {
            run_script(&path);
        }
    }

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();

    let mut wasi_builder = runtime::PshWasiConfigBuilder::new(wasi_ctx);
    wasi_builder
        .set_component_path("target/wasm32-wasi/debug/get_memory_info.wasm")
        .enable_memory_ops();
    let wasi_config = wasi_builder.build();

    runtime::run_wasmtime_engine(wasi_config).unwrap();
}
