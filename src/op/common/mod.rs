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

pub mod cpu_info;
pub mod interrupts;
pub mod irq;
pub mod mem_info;
pub mod memory_module;
pub mod rps;
pub mod system;
pub mod utils;

#[derive(Debug, PartialEq)]
pub struct TlbSize {
    pub count: u32,
    pub unit: u32,
}

#[derive(Debug, PartialEq)]
pub struct AddressSizes {
    pub phy: u8,  // physical bits.
    pub virt: u8, // virtual bits.
}

#[derive(Debug, PartialEq)]
pub struct Arm64CpuInfo {
    pub processor: usize,
    pub bogomips: f32,
    pub features: Vec<String>,
    pub cpu_implementer: u16,
    pub cpu_architecture: u16,
    pub cpu_variant: u16,
    pub cpu_part: u16,
    pub cpu_revision: u16,
    pub address_sizes: AddressSizes,
}

impl Arm64CpuInfo {
    fn new() -> Arm64CpuInfo {
        Arm64CpuInfo {
            processor: 0,
            bogomips: 0.0,
            features: Vec::<String>::new(),
            cpu_implementer: 0,
            cpu_architecture: 0,
            cpu_variant: 0,
            cpu_part: 0,
            cpu_revision: 0,
            address_sizes: AddressSizes { phy: 0, virt: 0 },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct X86_64CpuInfo {
    pub processor: usize,
    pub vendor_id: String,
    pub model_name: String,
    pub cpu_family: usize,
    pub model: usize,
    pub stepping: usize,
    pub microcode: String,
    pub cpu_mhz: f64,
    pub cache_size: u32,
    pub physical_id: usize,
    pub siblings: usize,
    pub core_id: usize,
    pub cpu_cores: usize,
    pub apicid: usize,
    pub initial_apicid: usize,
    pub fpu: bool,
    pub fpu_exception: bool,
    pub cpuid_level: usize,
    pub wp: bool, // wp stands for ?
    pub flags: Vec<String>,
    pub bugs: Vec<String>,
    pub bogomips: f32,
    pub tlb_size: TlbSize,
    pub clflush_size: u8,
    pub cache_alignment: u8,
    pub address_sizes: AddressSizes,
    pub power_management: Vec<String>, // Add other fields you want to extract
}

impl X86_64CpuInfo {
    fn new() -> X86_64CpuInfo {
        X86_64CpuInfo {
            processor: 0,
            vendor_id: String::new(),
            model_name: String::new(),
            cpu_family: 0,
            model: 0,
            stepping: 0,
            microcode: String::new(),
            cpu_mhz: 0.0,
            cache_size: 0,
            physical_id: 0,
            siblings: 0,
            core_id: 0,
            cpu_cores: 0,
            apicid: 0,
            initial_apicid: 0,
            fpu: false,
            fpu_exception: false,
            cpuid_level: 0,
            wp: false,
            flags: Vec::<String>::new(),
            bugs: Vec::<String>::new(),
            bogomips: 0.0,
            tlb_size: TlbSize { count: 0, unit: 0 },
            clflush_size: 0,
            cache_alignment: 0,
            address_sizes: AddressSizes { phy: 0, virt: 0 },
            power_management: Vec::<String>::new(),
        }
    }
}

#[derive(Debug)]
pub enum CPUInfo {
    X86_64(Vec<X86_64CpuInfo>),
    Arm64(Vec<Arm64CpuInfo>),
    Unsupported(String),
}

#[derive(Debug, PartialEq)]
pub struct MemoryModule {
    array_handle: u32,
    error_info_handle: Option<u32>,
    total_width: Option<u8>,
    data_width: Option<u8>,
    size: u64,
    form_factor: String,
    set: Option<String>,
    locator: String,
    bank_locator: Option<String>,
    r#type: String,
    type_detail: String,
    speed: Option<String>,
    manufacturer: Option<String>,
    serial_number: Option<String>,
    asset_tag: Option<String>,
    part_number: Option<String>,
    rank: Option<u16>,
    configured_memory_speed: Option<String>,
    min_voltage: Option<String>,
    max_voltage: Option<String>,
    configured_voltage: Option<String>,
    memory_technology: Option<String>,
    memory_operating_mode_capability: Option<String>,
    firmware_version: Option<String>,
    module_manufacturer_id: Option<String>,
    module_product_id: Option<String>,
    memory_subsystem_controller_manufacturer_id: Option<String>,
    memory_subsystem_controller_product_id: Option<String>,
    non_volatile_size: Option<u64>,
    volatile_size: Option<u64>,
    // There is no need to define cache & logical size to
    // Option<u64>, Option<u32> is sufficient, but to reuse
    // parse_size_str() closure and satisfy Rust type inference
    // requirements, we expand them.
    cache_size: Option<u64>,
    logical_size: Option<u64>,
}

impl MemoryModule {
    fn new() -> MemoryModule {
        MemoryModule {
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
    fn new() -> MemInfo {
        MemInfo {
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
#[allow(dead_code)]
struct InterruptDetails {
    cpu_counts: Vec<u64>,
    interrupt_type: String,
    description: String,
}

impl InterruptDetails {
    fn new(cpu_counts: Vec<u64>, interrupt_type: String, description: String) -> InterruptDetails {
        InterruptDetails {
            cpu_counts,
            interrupt_type,
            description,
        }
    }
}

#[allow(dead_code)]
struct IrqDetails {
    irq_number: String,
    smp_affinity: Option<String>,
    smp_affinity_list: Option<String>,
    node: Option<String>,
}

impl IrqDetails {
    fn new(irq_number: String) -> IrqDetails {
        IrqDetails {
            irq_number,
            smp_affinity: None,
            smp_affinity_list: None,
            node: None,
        }
    }
}

#[allow(dead_code)]
struct RpsDetails {
    dev: String,
    rx: Vec<String>,
    rps_cpus: Vec<Option<String>>,
    rps_flow_cnt: Vec<Option<String>>,
}
