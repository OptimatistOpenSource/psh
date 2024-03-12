#[rustfmt::skip]
mod bindings;

use bindings::psh::profiling::system;

fn main() {
    let os_ver = system::os_version();
    let ker_ver = system::kernel_version();
    println!("os version: {:?}", os_ver);
    println!("kernel version: {:}", ker_ver);
}
