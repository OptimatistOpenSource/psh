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

use std::ffi::OsStr;
use std::io::Error;
use std::process::Command;

/// Run program with args
///
/// Example:
///
/// ```rust
/// let bytes = run_program("echo", ["hello world"]).unwrap();
/// assert_eq!("hello world\n", String::from_utf8(bytes).unwrap());
/// ```
pub fn run_program<P, A>(program: P, args: A) -> Result<Vec<u8>, Error>
where
    P: AsRef<OsStr>,
    A: IntoIterator<Item = P>,
{
    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd.output().map(|output| output.stdout)
}

#[cfg(test)]
mod tests {
    use super::run_program;

    #[test]
    fn test_run_program() {
        let output = run_program("echo", ["hello world"]).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert_eq!("hello world\n", output_str);
    }
}
