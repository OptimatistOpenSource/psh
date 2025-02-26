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

use std::{io, str::Utf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to get system information: {0}.")]
    System(#[from] io::Error),
    #[error("Failed to accquire mutex.")]
    Sync,
    #[error("Failed to retrive information using procfs: {0}.")]
    Procfs(#[from] procfs::ProcError),
    #[error("Failed to decode utf-8 string: {0}.")]
    Utf8(#[from] Utf8Error),
    #[error("Failed to find executable binary: {0}.")]
    Which(#[from] which::Error),
    #[error("Invalid Cpu Mask: {0}")]
    InvalidCpuMask(String),
    #[error("Value is empty")]
    EmptyValue,
}

pub type Result<T> = std::result::Result<T, Error>;
