# kawaiifi C Examples

Examples demonstrating how to use kawaiifi via its C FFI.

## Requirements

- CMake 3.22+
- A C compiler
- Rust toolchain (Corrosion will use it automatically)

## Build

```sh
cmake -B build
cmake --build build
```

## Run

### scan

Performs a blocking WiFi scan and prints the results.

```sh
./build/scan
```
