#[rustfmt::skip]
mod bindings;

use crate::bindings::profiling::system::memory;

fn main() {
    println!("Hello, world!");
    println!("{}", crate::bindings::name());
    let memory_info = memory::get_memory_info().unwrap();
    println!("{:?}", memory_info);

    let memory_module_vec = memory::get_memory_module().unwrap();

    println!("Dump Memory Modules:");
    for memory_module in memory_module_vec {
        println!("{:?}", memory_module);
    }
}
