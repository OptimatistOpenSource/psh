use std::process::Command;

use crate::profiling::system::memory::{
    self, MemoryInfo as GuestMemoryInfo, MemoryModule as GuestMemoryModule,
};

use super::raw::{parse_meminfo, parse_memory_module};
use super::{MemInfo as HostMemoryInfo, MemoryModule as HostMemoryModule};

use crate::utils::which;
use crate::SysCtx;

impl From<&HostMemoryInfo> for GuestMemoryInfo {
    fn from(value: &HostMemoryInfo) -> Self {
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
            kreclaimable: value.kreclaimable,
            slab: value.slab,
            sreclaimable: value.sreclaimable,
            sunreclaim: value.sunreclaim,
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
            percpu: value.percpu,
            cma_total: value.cma_total,
            cma_free: value.cma_free,
            hardware_corrupted: value.hardware_corrupted,
            anon_huge_pages: value.anon_huge_pages,
            shmem_huge_pages: value.shmem_huge_pages,
            shmem_pmd_mapped: value.shmem_pmd_mapped,
            file_huge_pages: value.file_huge_pages,
            file_pmd_mapped: value.file_pmd_mapped,
            huge_pages_total: value.huge_pages_total,
            huge_pages_free: value.huge_pages_free,
            huge_pages_rsvd: value.huge_pages_rsvd,
            huge_pages_surp: value.huge_pages_surp,
            huge_page_size: value.huge_page_size,
            huge_tlb: value.huge_tlb,
            direct_map4k: value.direct_map4k,
            direct_map2_m: value.direct_map2_m,
            direct_map1_g: value.direct_map1_g,
        }
    }
}

impl From<&HostMemoryModule> for GuestMemoryModule {
    fn from(value: &HostMemoryModule) -> Self {
        memory::MemoryModule {
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

impl memory::Host for SysCtx {
    fn get_memory_info(&mut self) -> wasmtime::Result<Result<GuestMemoryInfo, String>> {
        let mem_info = parse_meminfo!().unwrap();
        Ok(Ok((&mem_info).into()))
    }

    fn get_memory_module(&mut self) -> wasmtime::Result<Result<Vec<GuestMemoryModule>, String>> {
        if let Some(dmidecode_exe) = which("dmidecode") {
            let output = Command::new(dmidecode_exe).arg("-t").arg("17").output()?;

            let res = parse_memory_module(std::str::from_utf8(&output.stdout)?)
                .iter()
                .map(GuestMemoryModule::from)
                .collect::<Vec<GuestMemoryModule>>();

            Ok(Ok(res))
        } else {
            Ok(Err("Can not find `dmidecode` executable path.".to_string()))
        }
    }
}
