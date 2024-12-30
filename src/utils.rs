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
use nix::unistd::geteuid;

pub fn check_root_privilege() -> bool {
    let euid = geteuid();
    euid.is_root()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_check_root_privilege() {
        use super::check_root_privilege;
        // Test when the user has root privilege
        assert!(check_root_privilege());

        // Test when the user does not have root privilege
        // You can modify this test case to simulate a non-root user
        // by returning a non-root euid from geteuid() function
        // assert_eq!(check_root_privilege(), false);
    }
}
