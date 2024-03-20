use std::fs;
use std::path::Path;

use super::IrqDetails;

pub fn do_parse_all_irq(path: &str) -> std::io::Result<Vec<IrqDetails>> {
    let folder_names: Vec<String> = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();

    let parsed_irq = folder_names
        .iter()
        .map(|irq_number| {
            let num = (irq_number).parse::<u32>().unwrap();
            let mut irq_current = IrqDetails::new(num);

            let file_path = format!("{}/{}/smp_affinity", path, irq_number);
            if Path::new(&file_path).exists() {
                let smp_affinity = fs::read_to_string(&file_path).unwrap().trim().to_string();
                irq_current.smp_affinity = Some(smp_affinity);
            }

            let file_path = format!("{}/{}/smp_affinity_list", path, irq_number);
            if Path::new(&file_path).exists() {
                let smp_affinity_list = fs::read_to_string(&file_path).unwrap().trim().to_string();
                irq_current.smp_affinity_list = Some(smp_affinity_list);
            }

            let file_path = format!("{}/{}/node", path, irq_number);
            if Path::new(&file_path).exists() {
                let node = fs::read_to_string(&file_path).unwrap().trim().to_string();
                irq_current.node = Some(node);
            }

            irq_current
        })
        .collect::<Vec<IrqDetails>>();

    Ok(parsed_irq)
}

macro_rules! parse_irq {
    ($path:expr) => {
        crate::interrupts::irq::do_parse_all_irq($path)
    };
    () => {
        crate::interrupts::irq::do_parse_all_irq("/proc/irq")
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
        irq_path.push("test_resources/arch/x86_64/intel/irq");

        let result = do_parse_all_irq(irq_path.to_str().unwrap()).unwrap();
        assert_eq!(result.len(), 3);

        assert_eq!(result[0].irq_number, 12);
        assert_eq!(
            result[0].smp_affinity,
            Some("ffffffff,ffffffff,ffffffff,ffffffff,ffffffff".to_string())
        );
        assert_eq!(result[0].smp_affinity_list, Some("0-159".to_string()));
        assert_eq!(result[0].node, Some("0".to_string()));

        assert_eq!(result[1].irq_number, 9);
        assert_eq!(
            result[1].smp_affinity,
            Some("00000000,00ffffff,ffff0000,000000ff,ffffffff".to_string())
        );
        assert_eq!(result[1].smp_affinity_list, Some("0-39,80-119".to_string()));
        assert_eq!(result[1].node, Some("0".to_string()));

        assert_eq!(result[2].irq_number, 1);
        assert_eq!(
            result[2].smp_affinity,
            Some("ffffffff,ffffffff,ffffffff,ffffffff,ffffffff".to_string())
        );
        assert_eq!(result[2].smp_affinity_list, Some("0-159".to_string()));
        assert_eq!(result[2].node, Some("0".to_string()));
    }
}
