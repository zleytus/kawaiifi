<p align="center">
  <img src="data/icons/fi.kawaii.kawaiifi.svg" alt="KawaiiFi icon" width="96" height="96">
</p>

<h1 align="center">KawaiiFi</h1>

<p align="center">
  A Wi-Fi scanner and analyzer for Linux
</p>

<p align="center">
  <a href="https://github.com/zleytus/kawaiifi/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/zleytus/kawaiifi/actions/workflows/ci.yml/badge.svg"></a>
  <a href="LICENSE"><img alt="License: GPL-3.0-or-later" src="https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg"></a>
  <img alt="Platform: Linux" src="https://img.shields.io/badge/platform-Linux-blue.svg">
  <img alt="Built with GTK4/libadwaita" src="https://img.shields.io/badge/GTK4%20%2F%20libadwaita-3584e4.svg">
</p>

<p align="center">
  <img src="data/screenshots/01-main-window-bss-table.png" alt="KawaiiFi main window showing nearby Wi-Fi BSSs">
</p>

## Features

- Discover nearby Wi-Fi networks and view detailed BSS information
- Visualize channel usage across 2.4 GHz, 5 GHz, and 6 GHz bands
- Switch between Wi-Fi interfaces, including external USB adapters
- Filter scan results by SSID, BSSID, vendor, band, security, channel width, and 802.11 protocols
- Inspect 802.11 Information Elements for selected BSSs
- Save and reopen scan snapshots with `.kwifi` files

## Dependencies

### Build dependencies

- [Meson](https://mesonbuild.com/) >= 1.3
- [Rust](https://www.rust-lang.org/) (stable)
- [GTK4](https://www.gtk.org/)
- [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/)
- [blueprint-compiler](https://gitlab.gnome.org/GNOME/blueprint-compiler)

### Runtime requirements

- [NetworkManager](https://networkmanager.dev/) must be installed and running.
  KawaiiFi uses NetworkManager's D-Bus interface to initiate Wi-Fi scans.

## Building

Set up the build directory (run once, or when `meson.build` files change):

```sh
meson setup builddir --prefix=~/.local
```

Compile assets (run after changing `.blp` files or other non-Rust assets):

```sh
meson compile -C builddir
```

## Running

For Rust-only changes, you can run directly with Cargo after assets have been compiled:

```sh
APP_ID=fi.kawaii.kawaiifi RESOURCES_FILE=builddir/data/resources/resources.gresource GSETTINGS_SCHEMA_DIR=builddir/data cargo run
```

To do a full install:

```sh
meson install -C builddir
```

## License

KawaiiFi is licensed under GPL-3.0-or-later.
