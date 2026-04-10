# Packaging

## Flatpak

### Prerequisites

```sh
flatpak install org.gnome.Platform//50 org.gnome.Sdk//50 org.freedesktop.Sdk.Extension.rust-stable//24.08
```

### Building a Bundle

1. Regenerate `cargo-sources.json` (required after any changes to `Cargo.lock`):

```sh
python3 /path/to/flatpak-cargo-generator.py ../Cargo.lock -o flatpak/cargo-sources.json
```

> `flatpak-cargo-generator.py` is available from [flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo).

2. Build and commit to a local repo (run from the `gui/` directory):

```sh
flatpak-builder --repo=repo --force-clean builddir-flatpak flatpak/fi.kawaii.kawaiifi.yml
```

3. Package into a `.flatpak` bundle:

```sh
flatpak build-bundle repo kawaiifi.flatpak fi.kawaii.kawaiifi
```

### Installing the Bundle (recipient's machine)

```sh
flatpak install kawaiifi.flatpak
flatpak run fi.kawaii.kawaiifi
```
