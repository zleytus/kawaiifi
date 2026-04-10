# Packaging

## Flatpak

### Prerequisites

```sh
flatpak install org.gnome.Platform//49 org.gnome.Sdk//49 org.freedesktop.Sdk.Extension.rust-stable//24.08
```

### Building a Bundle

1. Regenerate `cargo-sources.json` (required after any changes to `Cargo.lock`):

```sh
python3 /path/to/flatpak-cargo-generator.py ../Cargo.lock -o flatpak/cargo-sources.json
```

> `flatpak-cargo-generator.py` is available from [flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo).

2. Build and commit to a local repo (run from the `gui/` directory):

```sh
flatpak-builder --repo=repo --force-clean builddir-flatpak flatpak/com.github.kawaiifi.yml
```

3. Package into a `.flatpak` bundle:

```sh
flatpak build-bundle repo kawaiifi.flatpak com.github.kawaiifi
```

### Installing the Bundle (recipient's machine)

```sh
flatpak install kawaiifi.flatpak
flatpak run com.github.kawaiifi
```
