<p align="center">
  <img src="data/icons/fi.kawaii.kawaiifi.svg" alt="KawaiiFi icon" width="96" height="96">
</p>

<h1 align="center">KawaiiFi</h1>

<p align="center">
  A Wi-Fi scanner for Linux built with GTK4/libadwaita
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

## Dependencies

- [Meson](https://mesonbuild.com/) >= 1.4
- [Rust](https://www.rust-lang.org/) (stable)
- GTK4
- libadwaita
- [blueprint-compiler](https://gitlab.gnome.org/GNOME/blueprint-compiler)
- NetworkManager

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
