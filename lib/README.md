# kawaiifi

`kawaiifi` is a cross-platform Wi-Fi scanning and monitoring library.

## Example

### Linux

On Linux, scans accept an explicit backend because they can be triggered through
either NetworkManager or Netlink.

```rust
use std::error::Error;
use kawaiifi::scan::Backend;

fn main() -> Result<(), Box<dyn Error>> {
    let interface = kawaiifi::default_interface().ok_or("No Wi-Fi interface found")?;
    let scan = interface.scan_blocking(Backend::NetworkManager)?;

    println!("Found {} BSS(s)", scan.bss_list().len());
    Ok(())
}
```

### Windows/macOS

On Windows and macOS, scans use the platform's Wi-Fi API directly:
Native WiFi on Windows and CoreWLAN on macOS.

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let interface = kawaiifi::default_interface().ok_or("No Wi-Fi interface found")?;
    let scan = interface.scan_blocking()?;

    println!("Found {} BSS(s)", scan.bss_list().len());
    Ok(())
}
```

See `examples/scan_sync.rs` and `examples/scan_async.rs` for fuller platform-specific examples.
