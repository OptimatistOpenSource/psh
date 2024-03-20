#[rustfmt::skip]
mod bindings;

use crate::bindings::profiling::system::interrupts::{get_interrupts_info, get_interrupts_stat};

fn main() {
    println!("Example: get_interrupts_info");

    let interrupts = get_interrupts_info().unwrap();
    for interrupt in interrupts {
        println!("{:?}", interrupt);
    }

    let mut c = 0;
    loop {
        let interrupts_stat = get_interrupts_stat().unwrap();
        for stat in interrupts_stat {
            println!("{:?}", stat);
        }
        println!();

        std::thread::sleep(std::time::Duration::from_secs(1));
        c += 1;
        if c > 3 {
            break;
        }
    }
}
