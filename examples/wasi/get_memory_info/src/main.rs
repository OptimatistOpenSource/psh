#[rustfmt::skip]
mod bindings;

use crate::bindings::psh::profiling::memory;
//use crate::bindings::psh::profiling::perf;

fn main() {
    println!("Hello, world!");
    println!("{}", crate::bindings::name());
    let a = memory::get_memory_info().unwrap();
    println!("{:?}", a);

    // test if host not implemented perf imports.
    // perf::perf_new_counter(0, 0, 0);
    // panic!("test panic");
}
