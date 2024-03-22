#[rustfmt::skip]
mod bindings;

use bindings::profiling::system::rps;

fn main() {
    println!("Example: get_rps_info");
    let rps_info = rps::get_rps_info();
    for info in &rps_info {
        println!("{:?}", info);
    }
}
