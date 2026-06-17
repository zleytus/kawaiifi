# Changelog

## Unreleased

### `kawaiifi`

#### Breaking

- Removed ambiguous `WifiProtocols` conversions from information elements and
  supported-rate elements that did not include band context.
- Removed `Backend` on Linux. Scans are now triggered using `NetworkManager` by
  default.

#### Fixed

- Fixed Wi-Fi protocol inference so 5 GHz OFDM rates are reported as 802.11a
  instead of 802.11g.

#### Added
- Added `Bss::max_spatial_streams()`

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
