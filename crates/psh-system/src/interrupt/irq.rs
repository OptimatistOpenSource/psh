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
use std::fs;

use super::IrqDetails;

pub fn do_parse_all_irq(path: &str) -> std::io::Result<Vec<IrqDetails>> {
    let folder_names: Vec<String> = fs::read_dir(path)?
        .filter_map(|entry| {
            entry.ok().and_then(|entry| {
                let path = entry.path();
                path.is_dir()
                    .then(|| entry.file_name().to_string_lossy().into_owned())
            })
        })
        .collect();

    let parsed_irq = folder_names
        .iter()
        .filter_map(|irq_number| {
            let irq_number = irq_number.parse::<u32>().ok()?;
            let path_to_content = |path: String| {
                fs::read_to_string(path)
                    .ok()
                    .map(|content| content.trim().to_owned())
            };

            let smp_path = format!("{}/{}/smp_affinity", path, irq_number);
            let smp_list_path = format!("{}/{}/smp_affinity_list", path, irq_number);
            let node_path = format!("{}/{}/node", path, irq_number);
            Some(IrqDetails {
                irq_number,
                smp_affinity: path_to_content(smp_path),
                smp_affinity_list: path_to_content(smp_list_path),
                node: path_to_content(node_path),
            })
        })
        .collect::<Vec<IrqDetails>>();

    Ok(parsed_irq)
}

macro_rules! parse_irq {
    ($path:expr) => {
        crate::interrupt::irq::do_parse_all_irq($path)
    };
    () => {
        crate::interrupt::irq::do_parse_all_irq("/proc/irq")
    };
}

pub(crate) use parse_irq;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_all_irq() {
        let mut irq_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        irq_path.push("./test_resources/arch/x86_64/intel/irq");

        let result = do_parse_all_irq(irq_path.to_str().unwrap()).unwrap();
        assert_eq!(result.len(), 3);

        let irq0 = result.iter().find(|&irq| irq.irq_number == 12);
        assert!(irq0.is_some());
        let irq0 = irq0.unwrap();
        assert_eq!(
            irq0.smp_affinity,
            Some("ffffffff,ffffffff,ffffffff,ffffffff,ffffffff".to_string())
        );
        assert_eq!(irq0.smp_affinity_list, Some("0-159".to_string()));
        assert_eq!(irq0.node, Some("0".to_string()));

        let irq1 = result.iter().find(|&irq| irq.irq_number == 9);
        assert!(irq1.is_some());
        let irq1 = irq1.unwrap();
        assert_eq!(
            irq1.smp_affinity,
            Some("00000000,00ffffff,ffff0000,000000ff,ffffffff".to_string())
        );
        assert_eq!(irq1.smp_affinity_list, Some("0-39,80-119".to_string()));
        assert_eq!(irq1.node, Some("0".to_string()));

        let irq2 = result.iter().find(|&irq| irq.irq_number == 1);
        assert!(irq2.is_some());
        let irq2 = irq2.unwrap();
        assert_eq!(
            irq2.smp_affinity,
            Some("ffffffff,ffffffff,ffffffff,ffffffff,ffffffff".to_string())
        );
        assert_eq!(irq2.smp_affinity_list, Some("0-159".to_string()));
        assert_eq!(irq2.node, Some("0".to_string()));
    }
}
