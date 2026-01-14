use std::error::Error;

use kawaiifi::scan::Backend;

fn main() -> Result<(), Box<dyn Error>> {
    let interface = kawaiifi::default_interface().expect("Expected to find a wireless interface");

    let scan = interface.scan_blocking(Backend::NetworkManager)?;

    println!(
        "Found {} BSS(s) in {:#?} on {} frequencies using {}",
        scan.bss_list().len(),
        scan.duration(),
        scan.freqs_mhz().len(),
        interface.name(),
    );

    Ok(())
}
