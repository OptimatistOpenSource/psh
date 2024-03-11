#[rustfmt::skip]
mod bindings;

use crate::bindings::psh::profiling::cpu::{self, CpuInfo};

fn main() {
    println!("Hello, world!");
    println!("{}", crate::bindings::name());
    let a = cpu::get_cpu_info().unwrap();

    match a {
        CpuInfo::X64(cpus) => {
            println!("CPU architecture: x86_64");
            println!("  nr: {}", cpus.len());
            for cpu in cpus {
                println!("{:?}\n", cpu);
            }
        }
        CpuInfo::Arm64(_) => todo!(),
        CpuInfo::Unsupported(_) => todo!(),
    }

    // test if host not implemented perf imports.
    // perf::perf_new_counter(0, 0, 0);
    // panic!("test panic");
}
