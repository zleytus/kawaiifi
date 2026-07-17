# Changelog

## Unreleased

### `kawaiifi`

#### Breaking

- Changed `ScanError::NetworkManager` to contain an `nmrs::ConnectionError`
  instead of a `zbus::Error`.

#### Changed

- Replaced the `zbus` connection and NetworkManager D-Bus proxies used
  for Linux scans with `nmrs`.

#### Added

- Added `Bss::ssid_lossy()` for displaying SSIDs with invalid UTF-8 bytes.

### `kawaiifi-ffi`

#### Added

- Added `kawaiifi_bss_ssid_lossy`.

### `Kawaiifi.Net`

#### Added

- Added `Bss.SsidLossy`.

## 0.2.1 - 2026-06-17

- Fixed formatting in the asynchronous scan example in the generated crate documentation.

## 0.2.0 - 2026-06-17

### `kawaiifi`

#### Breaking

- Removed ambiguous `WifiProtocols` conversions from information elements and
  supported-rate elements that did not include band context.
- Removed `Backend` on Linux. Scans are now triggered using `NetworkManager` by
  default.
- Changed `kawaiifi::interfaces()` and `kawaiifi::default_interface()` to return
  `Result<Vec<Interface>, InterfaceError>` and `Result<Option<Interface>, InterfaceError>`
  respectively.

#### Fixed

- Fixed Wi-Fi protocol inference so 5 GHz OFDM rates are reported as 802.11a
  instead of 802.11g.

#### Added

- Added `Bss::max_spatial_streams()`

### `kawaiifi-ffi`

#### Breaking

- Removed `Backend` on Linux. `kawaiifi_interface_scan` only has one argument
  on Linux now, just like on macOS and Windows.

#### Added

- Added `kawaiifi_bss_max_spatial_streams`

## 0.1.0 - 2026-06-09

### `kawaiifi`

- Added Wi-Fi interface enumeration and metadata.
- Added blocking and asynchronous Wi-Fi scanning.
- Added Linux scanning through NetworkManager and nl80211.
- Added macOS scanning through CoreWLAN.
- Added Windows scanning through Native Wifi.
- Added BSS metadata including SSID, BSSID, signal strength, channel, channel
  width, security protocols, Wi-Fi protocols, and Wi-Fi amendments.
- Added parsing for 802.11 information elements.
- Added serde serialization of scan results.

### `kawaiifi-ffi`

- Added C-compatible bindings for `kawaiifi`.

### `Kawaiifi.Net`

- Added .NET bindings for `kawaiifi`.
