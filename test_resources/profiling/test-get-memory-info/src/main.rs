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
#[rustfmt::skip]
#[allow(dead_code)]
mod imports;

use crate::imports::profiling::system::memory;

fn main() {
    let memory_info_vec = memory::info().unwrap();
    for memory_info in memory_info_vec {
        println!("{:?}", memory_info)
    }

    let memory_stat_vec = memory::stat(1).unwrap();

    println!("Dump Memory Statistics:");
    println!("{:?}", memory_stat_vec);
}
