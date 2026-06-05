# kawaiifi

[![CI](https://github.com/zleytus/kawaiifi/actions/workflows/ci.yml/badge.svg)](https://github.com/zleytus/kawaiifi/actions/workflows/ci.yml)
[![MSRV: 1.88](https://img.shields.io/badge/MSRV-1.88-blue)](https://blog.rust-lang.org/2025/05/15/Rust-1.88.0/)

`kawaiifi` is a Wi-Fi scanning library for Linux, macOS, and Windows.

It discovers local Basic Service Sets (BSSs) and reports their SSID, BSSID, signal strength, channel, channel width, security protocols, and information elements (IEs).

## Usage

```toml
[dependencies]
kawaiifi = "0.1"
```

### Obtaining a Wi-Fi Interface

Use `kawaiifi::default_interface()` to get the first available interface.

```rust
use kawaiifi::Interface;

let interface: Interface = kawaiifi::default_interface().ok_or("No Wi-Fi interface found")?;
```

Use `kawaiifi::interfaces()` to get all available interfaces.

```rust
use kawaiifi::Interface;

let interfaces: Vec<Interface> = kawaiifi::interfaces();
```

Some `Interface` properties are platform-specific.

```rust
#[cfg(target_os = "linux")]
println!("Index: {}", interface.index());

#[cfg(target_os = "macos")]
println!("Noise: {} dBm", interface.noise_dbm());

#[cfg(target_os = "windows")]
println!("Description: {}", interface.description());
```

### Triggering a Wi-Fi Scan

Blocking scans are available through `Interface::scan_blocking()` and asynchronous scans
through `Interface::scan()`.

On Linux, scans can be triggered through either [NetworkManager](https://networkmanager.dev/) or [nl80211](https://wireless.docs.kernel.org/en/latest/en/developers/documentation/nl80211.html) (Netlink), so a `Backend` must be specified.

```rust
use kawaiifi::{Scan, scan::Backend};

let scan: Scan = interface.scan_blocking(Backend::NetworkManager)?;
```

On macOS and Windows, scans are triggered through [CoreWLAN](https://developer.apple.com/documentation/CoreWLAN) and [Native Wifi](https://learn.microsoft.com/en-us/windows/win32/nativewifi/portal) respectively.

```rust
use kawaiifi::Scan;

let scan: Scan = interface.scan_blocking()?;
```

### Accessing BSS Data

`Scan` contains a list of BSSs that are accessed through `Scan::bss_list()`.

```rust
use kawaiifi::Bss;

let bss_list: &[Bss] = scan.bss_list();
println!("Found {} BSS(s)", bss_list.len());
```

`Bss` exposes common properties that are available on all platforms.

```rust
println!("BSSID: {:?}", bss.bssid());
println!("SSID: {:?}", bss.ssid());
println!("Frequency: {} MHz", bss.frequency_mhz());
println!("Channel: {}", bss.channel_number());
println!("Channel Width: {}", bss.channel_width());
println!("Signal: {} dBm", bss.signal_dbm());
println!("Security: {}", bss.security_protocols());
println!("Wi-Fi Protocols: {}", bss.wifi_protocols());
println!("Max Rate: {} Mbps", bss.max_rate_mbps());
```

Some `Bss` properties are platform-specific.

```rust
#[cfg(target_os = "linux")]
println!("Status: {:?}", bss.status());

#[cfg(target_os = "macos")]
println!("Noise: {} dBm", bss.noise_dbm());

#[cfg(target_os = "windows")]
println!("Link Quality: {}", bss.link_quality());
```

### Accessing Information Elements

`Bss` contains a list of 802.11 Information Elements (IEs) that are accessed through `Bss::ies()`.

```rust
use kawaiifi::Ie;

let ies: &[Ie] = bss.ies();
println!("Found {} IE(s)", ies.len());
```

`Ie` exposes basic properties such as the information element's name and ID.

```rust
println!("IE: {} ({})", ie.name(), ie.id);
```

`Ie` also exposes the information element's underlying data through `Ie::data`.

```rust
use kawaiifi::IeData;

match &ie.data {
    IeData::Ssid(ssid) => println!("SSID: {}", ssid.to_string_lossy()),
    IeData::DsParameterSet(ds) => println!("Channel: {}", ds.current_channel),
    IeData::Tim(tim) => println!("DTIM Period: {}", tim.dtim_period),
    IeData::VhtCapabilities(vht_caps) => {
        println!("Max MPDU Length: {}", vht_caps.vht_capabilities_info.maximum_mpdu_length)
    }
    _ => {}
}
```

## Troubleshooting

See the repository [troubleshooting notes](https://github.com/zleytus/kawaiifi#troubleshooting) for platform-specific permissions and location-services behavior.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
