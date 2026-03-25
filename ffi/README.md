# kawaiifi-ffi

C-compatible FFI bindings for [kawaiifi](../lib), a cross-platform Wi-Fi scanning library. Exports a C header via [cbindgen](https://github.com/mozilla/cbindgen) for use from C, C++, or any language with a C FFI.

## Building

```sh
cargo build --release
```

This produces a static library (`libkawaiifi.a` / `kawaiifi.lib`) and a shared library (`libkawaiifi.so` / `kawaiifi.dll`) under `target/release/`, and regenerates `include/kawaiifi.h`.

## Usage

Include `include/kawaiifi.h` and link against the built library.

```c
#include "kawaiifi.h"
#include <inttypes.h>
#include <stdio.h>

int main() {
    Interface *interface = kawaiifi_default_interface();
    if (!interface) return -1;

    Scan *scan = kawaiifi_interface_scan(interface);
    uintptr_t count = kawaiifi_scan_bss_count(scan);
    int64_t duration_ms = kawaiifi_scan_end_time_utc_ms(scan) -
                          kawaiifi_scan_start_time_utc_ms(scan);
    printf("Found %zu BSS(s) in %" PRId64 " ms\n", count, duration_ms);

    kawaiifi_scan_free(scan);
    kawaiifi_interface_free(interface);
    return 0;
}
```

See [`examples/c/`](examples/c/) for a complete CMake-based C example using [Corrosion](https://github.com/corrosion-rs/corrosion), or [`examples/dotnet/`](examples/dotnet/) for a C# example using the idiomatic `Kawaiifi.Net` wrapper library.

## Memory management

Functions that return heap-allocated values document how to free them:

- Strings (`char *`) — free with `kawaiifi_string_free`
- Byte buffers (`uint8_t *` with a count) — free with `kawaiifi_bytes_free`
- `Scan *` — free with `kawaiifi_scan_free`
- `Interface *` — free with `kawaiifi_interface_free`
- `FieldList *` — free with `kawaiifi_field_list_free`

Borrowed pointers (e.g. `const Bss *` from `kawaiifi_scan_bss_get`, `const Field *` from `kawaiifi_field_subfield_get`) are valid only for the lifetime of the parent object and must not be freed.
