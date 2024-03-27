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
#[rustfmt::skip]
mod bindings;

use crate::bindings::profiling::system::interrupts::{get_interrupts_info, get_interrupts_stat};

fn main() {
    println!("Example: get_interrupts_info");

    let interrupts = get_interrupts_info().unwrap();
    for interrupt in interrupts {
        println!("{:?}", interrupt);
    }

    let mut c = 0;
    loop {
        let interrupts_stat = get_interrupts_stat().unwrap();
        for stat in interrupts_stat {
            println!("{:?}", stat);
        }
        println!();

        std::thread::sleep(std::time::Duration::from_secs(1));
        c += 1;
        if c > 3 {
            break;
        }
    }
}
