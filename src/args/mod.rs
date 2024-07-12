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

use std::mem;
use std::process::exit;

use clap::Parser;

#[derive(Parser, Debug)]
#[non_exhaustive]
pub struct Args {
    /// Path to PSH wasm component
    #[arg(short, long)]
    #[arg(value_name = "PATH")]
    pub psh_wasm_component: Option<String>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub extra_args: Vec<String>,

    #[arg(short, long)]
    daemon: bool,
}

impl Args {
    /// whether run as daemon
    pub fn daemon(&self) -> bool {
        self.daemon
    }

    pub fn get_component_args(&mut self) -> Vec<String> {
        if self.psh_wasm_component.is_none() {
            eprintln!("The cli must specify WASM path.");
            exit(1);
        }
        let mut component_args = Vec::with_capacity(1 + self.extra_args.len());
        component_args.push( self.psh_wasm_component.take().unwrap());
        component_args.extend(mem::take(&mut self.extra_args));

        component_args
    }
}
