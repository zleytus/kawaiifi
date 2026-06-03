# Kawaiifi.Net

A cross-platform Wi-Fi scanning library for .NET.

Key types:

- `Interface` — a wireless network interface; use `Interface.Default()` or `Interface.All()` to access one or multiple interfaces
- `Scan` — the results of a Wi-Fi scan, obtained by calling `Interface.Scan()`
- `Bss` — a single BSS (network) discovered during a scan
- `Ie` — an 802.11 information element within a BSS
- `Field` / `FieldList` — decoded fields within an information element

## Implementation

`Kawaiifi.Net` is a managed wrapper around [kawaiifi](https://github.com/zleytus/kawaiifi), a
Wi-Fi scanning library written in Rust. It communicates with the native library via P/Invoke
and handles all interop, memory management, and platform differences internally, so callers never
need to write unsafe code or manage native memory directly.

## Requirements

- [.NET 10 SDK](https://dotnet.microsoft.com/download)
- [Rust toolchain](https://rustup.rs) (to build the native library)

## Build

First, build the native kawaiifi library from the workspace root:

```sh
cargo build --release
```

This produces `libkawaiifi.so` (Linux) or `kawaiifi.dll` (Windows) in `target/release/`.

Then build the .NET solution:

```sh
dotnet build
```

## Platform Notes

`Kawaiifi.Net` exposes platform-specific APIs via [`[SupportedOSPlatform]`](https://learn.microsoft.com/en-us/dotnet/api/system.runtime.versioning.supportedosplatformattribute) attributes.
The Roslyn analyzer will warn if platform-specific APIs are called without an OS check.

- **Linux** — nl80211 and NetworkManager scan backends, additional interface properties
  (wiphy, wdev, driver, bus type, etc.), and Linux-specific scan metadata
- **Windows** — WLAN API scan backend and Windows-specific interface properties
  (GUID, description)
