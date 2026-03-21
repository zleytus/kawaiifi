use std::error::Error;

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use kawaiifi::scan::Backend;

    let interface = kawaiifi::default_interface().expect("Expected to find a wireless interface");

    let scan = interface.scan(Backend::NetworkManager).await?;

    println!(
        "Found {} BSS(s) in {:#?} on {} frequencies using {}",
        scan.bss_list().len(),
        scan.duration(),
        scan.freqs_mhz().len(),
        interface.name(),
    );

    Ok(())
}

#[cfg(target_os = "windows")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let interface = kawaiifi::default_interface().expect("Expected to find a wireless interface");

    let scan = interface.scan().await?;

    println!(
        "Found {} BSS(s) in {:#?} using {}",
        scan.bss_list().len(),
        scan.duration(),
        interface.description()
    );

    Ok(())
}
