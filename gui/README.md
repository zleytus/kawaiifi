# kawaiifi

A Wi-Fi scanner for Linux with a GTK4/libadwaita interface.

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
APP_ID=com.github.kawaiifi RESOURCES_FILE=builddir/data/resources/resources.gresource cargo run
```

To do a full install:

```sh
meson install -C builddir
```
