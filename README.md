# kawaiifi

[![CI](https://github.com/zleytus/kawaiifi/actions/workflows/ci.yml/badge.svg)](https://github.com/zleytus/kawaiifi/actions/workflows/ci.yml)
[![License: MIT or Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

`kawaiifi` is a Wi-Fi scanning library for Linux, macOS, and Windows. It discovers nearby Wi-Fi Basic Service Sets (BSSs) and reports their SSID, BSSID, signal strength, channel, channel width, security protocols, and parsed 802.11 Information Elements.

## Repository

This repository contains:

- [`kawaiifi`](lib/) - the Rust library crate.
- [`kawaiifi-ffi`](ffi/) - C-compatible FFI bindings for the Rust library.
- [`Kawaiifi.Net`](ffi/dotnet/) - .NET wrapper around the Rust library.

## Platform Support

`kawaiifi` uses each platform's built-in Wi-Fi API:

| Platform | API |
|----------|---------|
| Linux | [NetworkManager](https://networkmanager.dev/) or [nl80211](https://wireless.docs.kernel.org/en/latest/en/developers/documentation/nl80211.html) (Netlink) |
| macOS | [CoreWLAN](https://developer.apple.com/documentation/CoreWLAN) |
| Windows | [Native Wifi](https://learn.microsoft.com/en-us/windows/win32/nativewifi/portal) (Win32) |

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
