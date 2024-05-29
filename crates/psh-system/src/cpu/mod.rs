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

use std::str::FromStr;

use crate::error::{Error, Result};

pub(crate) mod handle;
mod raw;

pub use handle::CpuHandle;
pub use procfs::CpuTime;

use procfs::KernelStats;

// use Vec<bool> to represent CpuMask but wrap it in a tuple struct to make it a distinct type
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CpuMask(pub Vec<bool>);

#[allow(dead_code)]
impl FromStr for CpuMask {
    type Err = Error;

    fn from_str(mask: &str) -> Result<CpuMask> {
        #[allow(clippy::identity_op)]
        let num_to_mask = |num: u32| {
            // num is guaranteed in range [0, 16)
            // one hex char corresponding to 4 bits in mask
            [
                ((num >> 0) & 1) != 0,
                ((num >> 1) & 1) != 0,
                ((num >> 2) & 1) != 0,
                ((num >> 3) & 1) != 0,
            ]
        };

        mask.chars()
            .rev() // reverse chars order
            .map(|c| match c {
                '0'..='9' => Ok(u32::from(c) - u32::from('0')),
                'a'..='f' => Ok(u32::from(c) - u32::from('a') + 10),
                'A'..='F' => Ok(u32::from(c) - u32::from('A') + 10),
                _ => Err(Error::InvalidCpuMask(mask.to_string())),
            })
            .collect::<Result<Vec<u32>>>()
            .map(|masks| CpuMask(masks.iter().flat_map(|&bits| num_to_mask(bits)).collect()))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TlbSize {
    pub count: u32,
    pub unit: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddressSizes {
    pub phy: u8,  // physical bits.
    pub virt: u8, // virtual bits.
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Clone)]
pub enum CPUInfo {
    X86_64(Vec<X86_64CpuInfo>),
    Arm64(Vec<Arm64CpuInfo>),
    Unsupported(String),
}

#[derive(Debug, Clone)]
pub struct CpuStats {
    pub total: CpuTime,
    pub per_cpu: Vec<CpuTime>,
}

impl From<&KernelStats> for CpuStats {
    fn from(value: &KernelStats) -> Self {
        Self {
            total: value.total.clone(),
            per_cpu: value.cpu_time.clone(),
        }
    }
}

impl From<KernelStats> for CpuStats {
    fn from(value: KernelStats) -> Self {
        Self {
            total: value.total,
            per_cpu: value.cpu_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CpuMask;

    #[test]
    fn test_cpu_mask() {
        let mask: CpuMask = "0".parse().unwrap();
        assert_eq!(mask, CpuMask(vec![false; 4]));

        let mask: CpuMask = "0000".parse().unwrap();
        assert_eq!(mask, CpuMask(vec![false; 16]));

        let mask: CpuMask = "1".parse().unwrap();
        assert_eq!(mask, CpuMask(vec![true, false, false, false]));

        let mask: CpuMask = "2".parse().unwrap();
        assert_eq!(mask, CpuMask(vec![false, true, false, false]));

        let mask: CpuMask = "f".parse().unwrap();
        assert_eq!(mask, CpuMask(vec![true; 4]));

        let mask: CpuMask = "11".parse().unwrap();
        assert_eq!(
            mask,
            CpuMask(vec![true, false, false, false, true, false, false, false])
        );

        let mask: CpuMask = "a3".parse().unwrap();
        assert_eq!(
            mask,
            CpuMask(vec![true, true, false, false, false, true, false, true])
        );

        let mask: Result<CpuMask, _> = "a3\n".parse();
        assert!(mask.is_err());
    }
}
