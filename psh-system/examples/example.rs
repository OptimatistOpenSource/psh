fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system = psh_system::System::default();
    println!("{:#?}", system);
    let copy = system.clone();
    let th = std::thread::spawn(move || {
        for _ in 0..6 {
            std::thread::sleep(std::time::Duration::from_secs(2));
            let networks = copy.network_stat_handle.get();
            println!("{:#?}", networks);
        }
    });
    for _ in 0..4 {
        std::thread::sleep(std::time::Duration::from_secs(3));
        let networks = system.network_stat_handle.get();
        println!("{:#?}", networks);
    }
    let result = th.join();
    println!("{:#?}", result);
    Ok(())
}
