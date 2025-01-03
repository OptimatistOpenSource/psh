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

use std::time::Duration;

use psh_system::memory::{Meminfo as HostMemoryStat, MemoryModule as HostMemoryInfo};

use crate::profiling::system::memory::{
    self, MemoryInfo as GuestMemoryInfo, MemoryStat as GuestMemoryStat,
};
use crate::SysCtx;

impl From<&HostMemoryStat> for GuestMemoryStat {
    fn from(value: &HostMemoryStat) -> Self {
        Self {
            mem_total: value.mem_total,
            mem_free: value.mem_free,
            mem_available: value.mem_available,
            buffers: value.buffers,
            cached: value.cached,
            swap_cached: value.swap_cached,
            active: value.active,
            inactive: value.inactive,
            active_anon: value.active_anon,
            inactive_anon: value.inactive_anon,
            active_file: value.active_file,
            inactive_file: value.inactive_file,
            unevictable: value.unevictable,
            mlocked: value.mlocked,
            swap_total: value.swap_total,
            swap_free: value.swap_free,
            dirty: value.dirty,
            writeback: value.writeback,
            anon_pages: value.anon_pages,
            mapped: value.mapped,
            shmem: value.shmem,
            k_reclaimable: value.k_reclaimable,
            slab: value.slab,
            s_reclaimable: value.s_reclaimable,
            sunreclaim: value.s_unreclaim,
            kernel_stack: value.kernel_stack,
            page_tables: value.page_tables,
            nfs_unstable: value.nfs_unstable,
            bounce: value.bounce,
            writeback_tmp: value.writeback_tmp,
            commit_limit: value.commit_limit,
            committed_as: value.committed_as,
            vmalloc_total: value.vmalloc_total,
            vmalloc_used: value.vmalloc_used,
            vmalloc_chunk: value.vmalloc_chunk,
            per_cpu: value.per_cpu,
            cma_total: value.cma_total,
            cma_free: value.cma_free,
            hardware_corrupted: value.hardware_corrupted,
            anon_hugepages: value.anon_hugepages,
            shmem_hugepages: value.shmem_hugepages,
            shmem_pmd_mapped: value.shmem_pmd_mapped,
            file_hugepages: value.file_huge_pages,
            file_pmd_mapped: value.file_pmd_mapped,
            hugepages_total: value.hugepages_total,
            hugepages_free: value.hugepages_free,
            hugepages_rsvd: value.hugepages_rsvd,
            hugepages_surp: value.hugepages_surp,
            hugepage_size: value.hugepagesize,
            hugetlb: value.hugetlb,
            direct_map4_k: value.direct_map_4k,
            direct_map2_m: value.direct_map_2M,
            direct_map1_g: value.direct_map_1G,
        }
    }
}

impl From<HostMemoryStat> for GuestMemoryStat {
    fn from(value: HostMemoryStat) -> Self {
        Self::from(&value)
    }
}

impl From<&HostMemoryInfo> for GuestMemoryInfo {
    fn from(value: &HostMemoryInfo) -> Self {
        Self {
            array_handle: value.array_handle,
            error_info_handle: value.error_info_handle,
            total_width: value.total_width,
            data_width: value.data_width,
            size: value.size,
            form_factor: value.form_factor.clone(),
            set: value.set.clone(),
            locator: value.locator.clone(),
            bank_locator: value.bank_locator.clone(),
            module_type: value.r#type.clone(),
            type_detail: value.type_detail.clone(),
            speed: value.speed.clone(),
            manufacturer: value.manufacturer.clone(),
            serial_number: value.serial_number.clone(),
            asset_tag: value.asset_tag.clone(),
            part_number: value.part_number.clone(),
            rank: value.rank,
            configured_memory_speed: value.configured_memory_speed.clone(),
            min_voltage: value.min_voltage.clone(),
            max_voltage: value.max_voltage.clone(),
            configured_voltage: value.configured_voltage.clone(),
            memory_technology: value.memory_technology.clone(),
            memory_operating_mode_capability: value.memory_operating_mode_capability.clone(),
            firmware_version: value.firmware_version.clone(),
            module_manufacturer_id: value.module_manufacturer_id.clone(),
            module_product_id: value.module_product_id.clone(),
            memory_subsystem_controller_manufacturer_id: value
                .memory_subsystem_controller_manufacturer_id
                .clone(),
            memory_subsystem_controller_product_id: value
                .memory_subsystem_controller_product_id
                .clone(),
            non_volatile_size: value.non_volatile_size,
            volatile_size: value.volatile_size,
            cache_size: value.cache_size,
            logical_size: value.logical_size,
        }
    }
}

impl From<HostMemoryInfo> for GuestMemoryInfo {
    fn from(value: HostMemoryInfo) -> Self {
        Self {
            array_handle: value.array_handle,
            error_info_handle: value.error_info_handle,
            total_width: value.total_width,
            data_width: value.data_width,
            size: value.size,
            form_factor: value.form_factor,
            set: value.set,
            locator: value.locator,
            bank_locator: value.bank_locator,
            module_type: value.r#type,
            type_detail: value.type_detail,
            speed: value.speed,
            manufacturer: value.manufacturer,
            serial_number: value.serial_number,
            asset_tag: value.asset_tag,
            part_number: value.part_number,
            rank: value.rank,
            configured_memory_speed: value.configured_memory_speed,
            min_voltage: value.min_voltage,
            max_voltage: value.max_voltage,
            configured_voltage: value.configured_voltage,
            memory_technology: value.memory_technology,
            memory_operating_mode_capability: value.memory_operating_mode_capability,
            firmware_version: value.firmware_version,
            module_manufacturer_id: value.module_manufacturer_id,
            module_product_id: value.module_product_id,
            memory_subsystem_controller_manufacturer_id: value
                .memory_subsystem_controller_manufacturer_id,
            memory_subsystem_controller_product_id: value.memory_subsystem_controller_product_id,
            non_volatile_size: value.non_volatile_size,
            volatile_size: value.volatile_size,
            cache_size: value.cache_size,
            logical_size: value.logical_size,
        }
    }
}

impl memory::Host for SysCtx {
    fn stat(&mut self, interval_ms: u64) -> Result<GuestMemoryStat, String> {
        self.memory
            .stat(Some(Duration::from_millis(interval_ms)))
            .map(Into::into)
            .map_err(|err| err.to_string())
    }

    fn info(&mut self) -> Result<Vec<GuestMemoryInfo>, String> {
        self.memory
            .info()
            .map(|info| info.into_iter().map(Into::into).collect())
            .map_err(|err| err.to_string())
    }
}
