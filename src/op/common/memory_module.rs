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

use super::MemoryModule;

#[allow(dead_code)]
pub fn parse_memory_module(input: &str) -> Vec<MemoryModule> {
    let mut memory_modules = Vec::new();
    let mut current_module = MemoryModule::new();

    fn is_unknown_or<T>(value: &str, conv: impl FnOnce(&str) -> T) -> Option<T> {
        match value {
            "Not Provided" | "Unknown" | "Not Specified" | "None" => None,
            _ => Some(conv(value)),
        }
    }

    macro_rules! hex_str_to_number {
        ($value:expr, $to_type:ty) => {
            <$to_type>::from_str_radix($value.trim_start_matches("0x"), 16).unwrap_or(0)
        };
    }

    let parse_width_str = |val: &str| {
        let v: Vec<&str> = val.split_ascii_whitespace().collect();
        if v.len() < 2 {
            return 0;
        }
        match v[1] {
            "bits" => v[0].parse().unwrap_or(0),
            _ => 0,
        }
    };

    let parse_size_str = |val: &str| {
        let v: Vec<&str> = val.split_ascii_whitespace().collect();
        if v.len() < 2 {
            return 0;
        }
        v[0].parse().unwrap_or(0)
            * match v[1] {
                "KB" => 1024,
                "MB" => 1024 * 1024,
                "GB" => 1024 * 1024 * 1024,
                _ => 0,
            }
    };

    for line in input.lines().skip(4) {
        let parts: Vec<&str> = line.trim().split(':').map(|s| s.trim()).collect();

        if parts.len() == 2 {
            let key = parts[0];
            let value = parts[1];

            match key {
                "Array Handle" => current_module.array_handle = hex_str_to_number!(value, u32),
                "Error Information Handle" => {
                    current_module.error_info_handle =
                        is_unknown_or(value, |v: &str| hex_str_to_number!(v, u32));
                }
                "Total Width" => {
                    current_module.total_width = is_unknown_or(value, parse_width_str);
                }
                "Data Width" => {
                    current_module.data_width = is_unknown_or(value, parse_width_str);
                }
                "Size" => {
                    current_module.size = parse_size_str(value);
                }
                "Form Factor" => current_module.form_factor = value.to_string(),
                "Set" => current_module.set = is_unknown_or(value, |v: &str| v.to_string()),
                "Locator" => current_module.locator = value.to_string(),
                "Bank Locator" => {
                    current_module.bank_locator = is_unknown_or(value, |v: &str| v.to_string())
                }
                "Type" => current_module.r#type = value.to_string(),
                "Type Detail" => current_module.type_detail = value.to_string(),
                "Speed" => current_module.speed = is_unknown_or(value, |v: &str| v.to_string()),
                "Manufacturer" => {
                    current_module.manufacturer = is_unknown_or(value, |v: &str| v.to_string())
                }
                "Serial Number" => {
                    current_module.serial_number = is_unknown_or(value, |v: &str| v.to_string())
                }
                "Asset Tag" => {
                    current_module.asset_tag = is_unknown_or(value, |v: &str| v.to_string())
                }
                "Part Number" => {
                    current_module.part_number = is_unknown_or(value, |v: &str| v.to_string())
                }
                "Rank" => {
                    current_module.rank = is_unknown_or(value, |v: &str| v.parse().unwrap_or(0))
                }
                "Configured Memory Speed" => {
                    current_module.configured_memory_speed =
                        is_unknown_or(value, |v: &str| v.to_string())
                }
                "Minimum Voltage" => {
                    current_module.min_voltage = is_unknown_or(value, |v: &str| v.to_string())
                }
                "Maximum Voltage" => {
                    current_module.max_voltage = is_unknown_or(value, |v: &str| v.to_string())
                }
                "Configured Voltage" => {
                    current_module.configured_voltage =
                        is_unknown_or(value, |v: &str| v.to_string())
                }
                "Memory Technology" => {
                    current_module.memory_technology = is_unknown_or(value, |v: &str| v.to_string())
                }
                "Memory Operating Mode Capability" => {
                    current_module.memory_operating_mode_capability =
                        is_unknown_or(value, |v: &str| v.to_string())
                }
                "Firmware Version" => {
                    current_module.firmware_version = is_unknown_or(value, |v: &str| v.to_string())
                }
                "Module Manufacturer ID" => {
                    current_module.module_manufacturer_id =
                        is_unknown_or(value, |v: &str| v.to_string())
                }
                "Module Product ID" => {
                    current_module.module_product_id = is_unknown_or(value, |v: &str| v.to_string())
                }
                "Memory Subsystem Controller Manufacturer ID" => {
                    current_module.memory_subsystem_controller_manufacturer_id =
                        is_unknown_or(value, |v: &str| v.to_string())
                }
                "Memory Subsystem Controller Product ID" => {
                    current_module.memory_subsystem_controller_product_id =
                        is_unknown_or(value, |v: &str| v.to_string())
                }
                "Non-Volatile Size" => {
                    current_module.non_volatile_size = is_unknown_or(value, parse_size_str);
                }
                "Volatile Size" => {
                    current_module.volatile_size = is_unknown_or(value, parse_size_str);
                }
                "Cache Size" => {
                    current_module.cache_size = is_unknown_or(value, parse_size_str);
                }
                "Logical Size" => {
                    current_module.logical_size = is_unknown_or(value, parse_size_str);
                }
                // Add more fields as needed...
                _ => {}
            }
        } else if line.is_empty() {
            memory_modules.push(current_module);
            current_module = MemoryModule::new();
        }
    }

    memory_modules
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    use crate::op::common::MemoryModule;

    use super::parse_memory_module;

    #[test]
    #[cfg(target_os = "linux")]
    fn test_dmidecode_memory_local() {
        use super::parse_memory_module;

        // TODO find dmidecode command path
        let dmidecode_exe = "/usr/sbin/dmidecode";
        let output = Command::new(dmidecode_exe)
            .arg("-t")
            .arg("16")
            .output()
            .expect("failed to execute dmidecode -t 16");

        let dmidecode_res = String::from_utf8(output.stdout).unwrap();
        let mut num_of_dimms = 0;
        for line in dmidecode_res.lines() {
            let parts: Vec<&str> = line.trim().split(':').map(|s| s.trim()).collect();

            if parts.len() == 2 {
                let key = parts[0];
                let value = parts[1];

                match key {
                    "Number Of Devices" => num_of_dimms = value.parse().unwrap_or(0),
                    _ => {}
                }
            }
        }

        let output = Command::new(dmidecode_exe)
            .arg("-t")
            .arg("17")
            .output()
            .expect("failed to execute dmidecode -t 17");

        let memory_modules = parse_memory_module(std::str::from_utf8(&output.stdout).unwrap());
        assert_eq!(num_of_dimms, memory_modules.len());
        println!("{:?}", memory_modules[0])
    }

    #[test]
    fn test_dmidecode_memory_amd() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_resources/arch/x86_64/amd/dmidecode_memory");
        let binding = d.into_os_string();
        let dmidecode_memory_path = binding.to_str().unwrap();

        let num_of_dimms = 4;
        let contents = fs::read_to_string(dmidecode_memory_path).unwrap();
        let memory_modules = parse_memory_module(&contents.as_str());
        assert_eq!(num_of_dimms, memory_modules.len());
        let dimm0 = MemoryModule {
            array_handle: 19,
            error_info_handle: Some(26),
            total_width: Some(64),
            data_width: Some(64),
            size: 8589934592,
            form_factor: "DIMM".to_string(),
            set: None,
            locator: "DIMM 0".to_string(),
            bank_locator: Some("P0 CHANNEL A".to_string()),
            r#type: "DDR4".to_string(),
            type_detail: "Synchronous Unbuffered (Unregistered)".to_string(),
            speed: Some("3200 MT/s".to_string()),
            manufacturer: Some("Ramaxel Technology".to_string()),
            serial_number: Some("11A2152C".to_string()),
            asset_tag: None,
            part_number: Some("RMUA5190MF96HAF-3200".to_string()),
            rank: Some(1),
            configured_memory_speed: Some("2667 MT/s".to_string()),
            min_voltage: Some("1.2 V".to_string()),
            max_voltage: Some("1.2 V".to_string()),
            configured_voltage: Some("1.2 V".to_string()),
            memory_technology: Some("DRAM".to_string()),
            memory_operating_mode_capability: Some("Volatile memory".to_string()),
            firmware_version: None,
            module_manufacturer_id: Some("Bank 5, Hex 0x43".to_string()),
            module_product_id: None,
            memory_subsystem_controller_manufacturer_id: None,
            memory_subsystem_controller_product_id: None,
            non_volatile_size: None,
            volatile_size: Some(8589934592),
            cache_size: None,
            logical_size: None,
        };
        assert_eq!(dimm0, memory_modules[0])
    }

    #[test]
    fn test_dmidecode_memory_intel() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_resources/arch/x86_64/intel/dmidecode_memory");
        let binding = d.into_os_string();
        let dmidecode_memory_path = binding.to_str().unwrap();

        let num_of_dimms = 1;
        let contents = fs::read_to_string(dmidecode_memory_path).unwrap();
        let memory_modules = parse_memory_module(&contents.as_str());
        assert_eq!(num_of_dimms, memory_modules.len());
        let dimm0 = MemoryModule {
            array_handle: 4096,
            error_info_handle: None,
            total_width: None,
            data_width: None,
            size: 4294967296,
            form_factor: "DIMM".to_string(),
            set: None,
            locator: "DIMM 0".to_string(),
            bank_locator: None,
            r#type: "RAM".to_string(),
            type_detail: "Other".to_string(),
            speed: None,
            manufacturer: Some("Alibaba Cloud".to_string()),
            serial_number: None,
            asset_tag: None,
            part_number: None,
            rank: None,
            configured_memory_speed: None,
            min_voltage: None,
            max_voltage: None,
            configured_voltage: None,
            memory_technology: None,
            memory_operating_mode_capability: None,
            firmware_version: None,
            module_manufacturer_id: None,
            module_product_id: None,
            memory_subsystem_controller_manufacturer_id: None,
            memory_subsystem_controller_product_id: None,
            non_volatile_size: None,
            volatile_size: None,
            cache_size: None,
            logical_size: None,
        };
        assert_eq!(dimm0, memory_modules[0]);
    }

    #[test]
    fn test_dmidecode_memory_yitian() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_resources/arch/aarch64/t-head/dmidecode_memory");
        let binding = d.into_os_string();
        let dmidecode_memory_path = binding.to_str().unwrap();

        let num_of_dimms = 16;
        let contents = fs::read_to_string(dmidecode_memory_path).unwrap();
        let memory_modules = parse_memory_module(&contents.as_str());
        assert_eq!(num_of_dimms, memory_modules.len());

        let dimm0 = MemoryModule {
            array_handle: 0,
            error_info_handle: None,
            total_width: Some(80),
            data_width: Some(64),
            size: 34359738368,
            form_factor: "DIMM".to_string(),
            set: None,
            locator: "DIMM000".to_string(),
            bank_locator: Some("SOCKET0 IMC0 DIMM0".to_string()),
            r#type: "DDR5".to_string(),
            type_detail: "Synchronous".to_string(),
            speed: Some("4800 MT/s".to_string()),
            manufacturer: Some("Hynix".to_string()),
            serial_number: Some("2A0B811280AD012319878E8DB4".to_string()),
            asset_tag: Some("DIMM000_Asserttag".to_string()),
            part_number: Some("HMCG88MEBRA115N".to_string()),
            rank: Some(2),
            configured_memory_speed: Some("4800 MT/s".to_string()),
            min_voltage: Some("1.1 V".to_string()),
            max_voltage: Some("1.1 V".to_string()),
            configured_voltage: Some("1.1 V".to_string()),
            memory_technology: Some("DRAM".to_string()),
            memory_operating_mode_capability: None,
            firmware_version: None,
            module_manufacturer_id: None,
            module_product_id: None,
            memory_subsystem_controller_manufacturer_id: None,
            memory_subsystem_controller_product_id: None,
            non_volatile_size: None,
            volatile_size: None,
            cache_size: None,
            logical_size: None,
        };

        assert_eq!(dimm0, memory_modules[0]);
    }
}
