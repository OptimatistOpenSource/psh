#[rustfmt::skip]
mod bindings;

use bindings::profiling::system::os;

fn main() {
    let distro_ver = os::get_distro_version().unwrap();
    let kernel_ver = os::get_kernel_version().unwrap();
    println!("distro version: {:?}", distro_ver);
    println!("kernel version: {:?}", kernel_ver);
}
