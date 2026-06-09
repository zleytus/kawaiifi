# kawaiifi

[![CI](https://github.com/zleytus/kawaiifi/actions/workflows/ci.yml/badge.svg)](https://github.com/zleytus/kawaiifi/actions/workflows/ci.yml)
[![License: MIT or Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

`kawaiifi` is a Wi-Fi scanning library for Linux, macOS, and Windows.

It discovers local Basic Service Sets (BSSs) and reports their SSID, BSSID, signal strength, channel, channel width, security protocols, and information elements (IEs).

## Repository

This repository contains:

- [`kawaiifi`](lib/) - the Rust library crate.
- [`kawaiifi-ffi`](ffi/) - C-compatible FFI bindings for the Rust library.
- [`Kawaiifi.Net`](dotnet/) - .NET wrapper around the Rust library.

## Platform Support

`kawaiifi` uses each platform's built-in Wi-Fi API:

| Platform | API |
|----------|---------|
| Linux | [NetworkManager](https://networkmanager.dev/) or [nl80211](https://wireless.docs.kernel.org/en/latest/en/developers/documentation/nl80211.html) (Netlink) |
| macOS | [CoreWLAN](https://developer.apple.com/documentation/CoreWLAN) |
| Windows | [Native Wifi](https://learn.microsoft.com/en-us/windows/win32/nativewifi/portal) (Win32) |

## Troubleshooting

Wi-Fi scanning is often restricted by operating systems because nearby BSS data
can be used to determine physical location and other sensitive information.

### Linux

Scanning via the `Nl80211` backend requires either root privileges or the
`CAP_NET_ADMIN` capability. The `NetworkManager` backend does not have this
requirement because NetworkManager handles the scan on behalf of the application.

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

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release notes.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
