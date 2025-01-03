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

pub(crate) mod handle;
mod mem_info;
mod memory_module;
mod raw;

pub use handle::MemoryHandle;
pub use procfs::Meminfo;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MemoryModule {
    pub array_handle: u32,
    pub error_info_handle: Option<u32>,
    pub total_width: Option<u8>,
    pub data_width: Option<u8>,
    pub size: u64,
    pub form_factor: String,
    pub set: Option<String>,
    pub locator: String,
    pub bank_locator: Option<String>,
    pub r#type: String,
    pub type_detail: String,
    pub speed: Option<String>,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub asset_tag: Option<String>,
    pub part_number: Option<String>,
    pub rank: Option<u16>,
    pub configured_memory_speed: Option<String>,
    pub min_voltage: Option<String>,
    pub max_voltage: Option<String>,
    pub configured_voltage: Option<String>,
    pub memory_technology: Option<String>,
    pub memory_operating_mode_capability: Option<String>,
    pub firmware_version: Option<String>,
    pub module_manufacturer_id: Option<String>,
    pub module_product_id: Option<String>,
    pub memory_subsystem_controller_manufacturer_id: Option<String>,
    pub memory_subsystem_controller_product_id: Option<String>,
    pub non_volatile_size: Option<u64>,
    pub volatile_size: Option<u64>,
    // There is no need to define cache & logical size to
    // Option<u64>, Option<u32> is sufficient, but to reuse
    // parse_size_str() closure and satisfy Rust type inference
    // requirements, we expand them.
    pub cache_size: Option<u64>,
    pub logical_size: Option<u64>,
}

impl MemoryModule {
    const fn new() -> Self {
        Self {
            array_handle: 0,
            error_info_handle: None,
            total_width: None,
            data_width: None,
            size: 0,
            form_factor: String::new(),
            set: None,
            locator: String::new(),
            bank_locator: None,
            r#type: String::new(),
            type_detail: String::new(),
            speed: None,
            manufacturer: None,
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
        }
    }
}
