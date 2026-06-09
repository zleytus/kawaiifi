use std::error::Error;

use kawaiifi::Scan;

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
    use kawaiifi::Backend;

    let interface = kawaiifi::default_interface().expect("Expected to find a wireless interface");
    let scan = interface.scan_blocking(Backend::NetworkManager)?;
    print_bss_data(&scan);

    Ok(())
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn main() -> Result<(), Box<dyn Error>> {
    let interface = kawaiifi::default_interface().expect("Expected to find a wireless interface");
    let scan = interface.scan_blocking()?;
    print_bss_data(&scan);

    Ok(())
}

fn print_bss_data(scan: &Scan) {
    for bss in scan.bss_list() {
        let bssid = bss.bssid();
        println!(
            "BSSID: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            bssid[0], bssid[1], bssid[2], bssid[3], bssid[4], bssid[5]
        );
        println!("SSID: {}", bss.ssid().unwrap_or_default());
        println!("Frequency: {} MHz", bss.frequency_mhz());
        println!("Band: {}", bss.band());
        println!("Channel: {}", bss.channel_number());
        println!("Channel Width: {}", bss.channel_width());
        println!("Signal: {} dBm", bss.signal_dbm());
        println!("Security: {}", bss.security_protocols());
        println!("Protocols: {}", bss.wifi_protocols());
        println!("Amendments: {}", bss.wifi_amendments());
        println!("Max Rate: {:.2} Mbps", bss.max_rate_mbps());
        println!();
    }
}
