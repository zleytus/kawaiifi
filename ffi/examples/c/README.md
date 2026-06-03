# kawaiifi C Examples

Examples demonstrating how to use kawaiifi via its C FFI.

## Requirements

- CMake 3.22+
- A C compiler
- Rust toolchain (required by [Corrosion](https://github.com/corrosion-rs/corrosion))

## Build

```sh
cmake -B build
cmake --build build
```

## Run

### `scan`

Performs a blocking Wi-Fi scan and prints the number of BSSs found.

```sh
./build/scan
```

### `bss_data`

Prints the SSID, frequency, channel, signal, and max rate of each BSS found
in a Wi-Fi scan.

```sh
./build/bss_data
```

### `ies`

Prints the name, ID, and summary of every information element for each BSS
found in a Wi-Fi scan.

```sh
./build/ies
```
