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
#![allow(unused_imports)]

mod config;
mod cpu;
mod event;
mod event_scope;
mod extra_config;
mod process;
mod stat;

pub use config::*;
pub use cpu::*;
pub use event::*;
pub use event_scope::*;
pub use extra_config::*;
pub use process::*;
pub use stat::*;
use thiserror::Error;

#[repr(transparent)]
pub struct Wrap<T>(T);

impl<T> Wrap<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Option is unsupported: {0}")]
    UnsupportedOption(String),
}
