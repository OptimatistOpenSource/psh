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

#[derive(Debug, Clone)]
pub struct MemInfo {
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_available: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_cached: u64,
    pub active: u64,
    pub inactive: u64,
    pub active_anon: u64,
    pub inactive_anon: u64,
    pub active_file: u64,
    pub inactive_file: u64,
    pub unevictable: u64,
    pub mlocked: u64,
    pub swap_total: u64,
    pub swap_free: u64,
    pub dirty: u64,
    pub writeback: u64,
    pub anon_pages: u64,
    pub mapped: u64,
    pub shmem: u64,
    pub kreclaimable: u64,
    pub slab: u64,
    pub sreclaimable: u64,
    pub sunreclaim: u64,
    pub kernel_stack: u64,
    pub page_tables: u64,
    pub nfs_unstable: u64,
    pub bounce: u64,
    pub writeback_tmp: u64,
    pub commit_limit: u64,
    pub committed_as: u64,
    pub vmalloc_total: u64,
    pub vmalloc_used: u64,
    pub vmalloc_chunk: u64,
    pub percpu: u64,
    pub cma_total: Option<u64>,
    pub cma_free: Option<u64>,
    pub hardware_corrupted: Option<u64>,
    pub anon_huge_pages: Option<u64>,
    pub shmem_huge_pages: Option<u64>,
    pub shmem_pmd_mapped: Option<u64>,
    pub file_huge_pages: Option<u64>,
    pub file_pmd_mapped: Option<u64>,
    pub huge_pages_total: u64,
    pub huge_pages_free: u64,
    pub huge_pages_rsvd: u64,
    pub huge_pages_surp: u64,
    pub huge_page_size: u64,
    pub huge_tlb: u64,
    pub direct_map4k: Option<u64>,
    pub direct_map2_m: Option<u64>,
    pub direct_map1_g: Option<u64>,
}

impl MemInfo {
    const fn new() -> Self {
        Self {
            mem_total: 0,
            mem_free: 0,
            mem_available: 0,
            buffers: 0,
            cached: 0,
            swap_cached: 0,
            active: 0,
            inactive: 0,
            active_anon: 0,
            inactive_anon: 0,
            active_file: 0,
            inactive_file: 0,
            unevictable: 0,
            mlocked: 0,
            swap_total: 0,
            swap_free: 0,
            dirty: 0,
            writeback: 0,
            anon_pages: 0,
            mapped: 0,
            shmem: 0,
            kreclaimable: 0,
            slab: 0,
            sreclaimable: 0,
            sunreclaim: 0,
            kernel_stack: 0,
            page_tables: 0,
            nfs_unstable: 0,
            bounce: 0,
            writeback_tmp: 0,
            commit_limit: 0,
            committed_as: 0,
            vmalloc_total: 0,
            vmalloc_used: 0,
            vmalloc_chunk: 0,
            percpu: 0,
            cma_total: None,
            cma_free: None,
            hardware_corrupted: None,
            anon_huge_pages: None,
            shmem_huge_pages: None,
            shmem_pmd_mapped: None,
            file_huge_pages: None,
            file_pmd_mapped: None,
            huge_pages_total: 0,
            huge_pages_free: 0,
            huge_pages_rsvd: 0,
            huge_pages_surp: 0,
            huge_page_size: 0,
            huge_tlb: 0,
            direct_map4k: None,
            direct_map2_m: None,
            direct_map1_g: None,
        }
    }
}
