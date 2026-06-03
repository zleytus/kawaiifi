# kawaiifi

[![CI](https://github.com/zleytus/kawaiifi/actions/workflows/ci.yml/badge.svg)](https://github.com/zleytus/kawaiifi/actions/workflows/ci.yml)
[![MSRV: 1.88](https://img.shields.io/badge/MSRV-1.88-blue)](https://blog.rust-lang.org/2025/05/15/Rust-1.88.0/)

`kawaiifi` is a Wi-Fi scanning library for Linux, macOS, and Windows.

## Usage

```toml
[dependencies]
kawaiifi = "0.1"
```

### Triggering a Wi-Fi Scan

Both blocking and asynchronous scans are available through
`Interface::scan_blocking` and `Interface::scan`. The examples below use the
blocking API.

#### Linux

On Linux, scans can be triggered through either [NetworkManager](https://networkmanager.dev/) or [nl80211](https://wireless.docs.kernel.org/en/latest/en/developers/documentation/nl80211.html) (Netlink), so a `Backend` must be specified.

```rust
use kawaiifi::scan::Backend;

let interface = kawaiifi::default_interface().ok_or("No Wi-Fi interface found")?;
let scan = interface.scan_blocking(Backend::NetworkManager)?;

println!("Found {} BSS(s)", scan.bss_list().len());
```

#### macOS and Windows

On macOS and Windows, scans are triggered through [CoreWLAN](https://developer.apple.com/documentation/CoreWLAN) and [Native Wifi](https://learn.microsoft.com/en-us/windows/win32/nativewifi/portal) respectively.

```rust
let interface = kawaiifi::default_interface().ok_or("No Wi-Fi interface found")?;
let scan = interface.scan_blocking()?;

println!("Found {} BSS(s)", scan.bss_list().len());
```

### Accessing BSS Data

BSSs expose common properties available on all platforms, such as BSSID, SSID, channel, signal strength, and security protocols.

```rust
for bss in scan.bss_list() {
    println!("BSSID: {:?}", bss.bssid());
    println!("SSID: {:?}", bss.ssid());
    println!("Frequency: {} MHz", bss.frequency_mhz());
    println!("Channel: {}", bss.channel_number());
    println!("Channel Width: {}", bss.channel_width());
    println!("Signal: {} dBm", bss.signal_dbm());
    println!("Security: {}", bss.security_protocols());
    println!("Wi-Fi Protocols: {}", bss.wifi_protocols());
    println!("Max Rate: {} Mbps", bss.max_rate_mbps());
    println!();
}
```

### Accessing Information Elements

Each BSS contains a list of 802.11 Information Elements (IEs). `kawaiifi` parses these into typed structs accessible via `Bss::ies()`.

```rust
use kawaiifi::IeData;

for bss in scan.bss_list() {
    for ie in bss.ies() {
        println!("IE: {} (id={}, id_ext={:?})", ie.name(), ie.id, ie.id_ext);
        match &ie.data {
            IeData::Ssid(ssid) => println!("SSID: {}", ssid.to_string_lossy()),
            IeData::DsParameterSet(ds) => println!("Channel: {}", ds.current_channel),
            IeData::VhtCapabilities(vht_caps) => {
                println!("Max MPDU Length: {}", vht_caps.vht_capabilities_info.maximum_mpdu_length)
            }
            _ => {}
        }
    }
    println!();
}
```

## Troubleshooting

Wi-Fi scanning is often restricted by operating systems because nearby BSS data
can be used to determine physical location and other sensitive information.

### Linux

Scanning via the `Nl80211` backend requires either root privileges or the
`CAP_NET_ADMIN` capability. The `NetworkManager` backend does not have this
requirement as NetworkManager handles the scan on behalf of the application.

### macOS

Scanning via [CoreWLAN](https://developer.apple.com/documentation/CoreWLAN) uses
`CWInterface.scanForNetworks`. If location services are not enabled for the
application, SSIDs, BSSIDs, and information element data may be unavailable.
Enable location services for your application in **System Settings → Privacy &
Security → Location Services**.

The command-line examples in this repository are not packaged as identified
macOS applications and do not request location authorization. When run from a
terminal, their scan results therefore have unavailable SSIDs, BSSIDs, and
information element lists.

### Windows

Scanning via [Native Wifi](https://learn.microsoft.com/en-us/windows/win32/nativewifi/portal) uses `WlanScan` and `WlanGetNetworkBssList`. The first time an
application calls these APIs, Windows displays a one-time prompt for precise
location access. If the user does not grant consent, scans fail with
`ERROR_ACCESS_DENIED`.
