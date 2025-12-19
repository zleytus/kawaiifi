use std::error::Error;

use kawaiifi::scan::Backend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let interface = kawaiifi::default_interface().expect("Expected to find a wireless interface");

    let scan_results = interface
        .scan_and_get_results(Backend::NetworkManager)
        .await?;

    println!("Found {} BSS(s)", scan_results.len());

    Ok(())
}
