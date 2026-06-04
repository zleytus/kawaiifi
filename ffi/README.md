# kawaiifi-ffi

`kawaiifi-ffi` is a Wi-Fi scanning library for Linux, macOS, and Windows.

It exposes a C-compatible FFI for [`kawaiifi`](../lib).

## Building

```sh
cargo build --release
```

This produces a static library (`libkawaiifi.a` / `kawaiifi.lib`) and a shared library (`libkawaiifi.so` / `libkawaiifi.dylib` / `kawaiifi.dll`) under `target/release/`, and regenerates `include/kawaiifi.h`.

## Usage

Include [`kawaiifi.h`](include/kawaiifi.h) and link against either the static or shared library.

### Triggering a Wi-Fi Scan

On Linux, scans can be triggered through either [NetworkManager](https://networkmanager.dev/) or [nl80211](https://wireless.docs.kernel.org/en/latest/en/developers/documentation/nl80211.html) (Netlink), so a `Backend` must be specified.

On macOS and Windows, scans are triggered through [CoreWLAN](https://developer.apple.com/documentation/CoreWLAN) and [Native Wifi](https://learn.microsoft.com/en-us/windows/win32/nativewifi/portal) respectively.

```c
#include "kawaiifi.h"
#include <inttypes.h>
#include <stdio.h>

int main() {
    Interface *interface = kawaiifi_default_interface();
    if (!interface) {
        return -1;
    }

    #if defined(__linux__)
    Scan *scan = kawaiifi_interface_scan(interface, BACKEND_NETWORK_MANAGER);
    #else
    Scan *scan = kawaiifi_interface_scan(interface);
    #endif

    uintptr_t count = kawaiifi_scan_bss_count(scan);
    printf("Found %zu BSS(s)\n", count);

    kawaiifi_scan_free(scan);
    kawaiifi_interface_free(interface);
    return 0;
}
```

See [`scan.c`](examples/scan.c).

### Accessing BSS Data

Each `Scan` contains a list of Basic Service Sets (BSSs) that is accessed
through `kawaiifi_scan_bss_get`.

```c
uintptr_t bss_count = kawaiifi_scan_bss_count(scan);
for (uintptr_t i = 0; i < bss_count; ++i) {
    const Bss *bss = kawaiifi_scan_bss_get(scan, i);
    char *ssid = kawaiifi_bss_ssid(bss);

    if (ssid) {
        printf("SSID: %s\n", ssid);
    }
    printf("Frequency: %" PRIu32 " MHz\n", kawaiifi_bss_frequency_mhz(bss));
    printf("Channel: %" PRIu8 "\n", kawaiifi_bss_channel_number(bss));
    printf("Signal: %" PRIi32 " dBm\n", kawaiifi_bss_signal_dbm(bss));
    printf("Max Rate: %lf Mbps\n", kawaiifi_bss_max_rate_mbps(bss));
    printf("\n");

    if (ssid) {
        kawaiifi_string_free(ssid);
    }
}
```

See [`bss_data.c`](examples/bss_data.c).

### Accessing Information Elements

Each `Bss` contains a list of 802.11 Information Elements (IEs)
that is accessed through `kawaiifi_bss_ie_get`.

```c
uintptr_t bss_count = kawaiifi_scan_bss_count(scan);
for (uintptr_t i = 0; i < bss_count; ++i) {
    const Bss *bss = kawaiifi_scan_bss_get(scan, i);
    uintptr_t ie_count = kawaiifi_bss_ie_count(bss);
    for (uintptr_t j = 0; j < ie_count; ++j) {
        const Ie *ie = kawaiifi_bss_ie_get(bss, j);
        if (!ie) {
            continue;
        }

        char *ie_name = kawaiifi_ie_name(ie);
        uint8_t ie_id = kawaiifi_ie_id(ie);
        char *ie_summary = kawaiifi_ie_summary(ie);

        printf("IE: %s (%" PRIu8 ")", ie_name, ie_id);
        printf(" - %s\n", ie_summary);

        kawaiifi_string_free(ie_name);
        kawaiifi_string_free(ie_summary);
    }
    printf("\n");
}
```

See [`ies.c`](examples/ies.c).

## Memory management

Functions that return heap-allocated values document how to free them:

- Strings (`char *`) - free with `kawaiifi_string_free`
- Byte buffers (`uint8_t *` with a count) - free with `kawaiifi_bytes_free`
- `Scan *` - free with `kawaiifi_scan_free`
- `Interface *` - free with `kawaiifi_interface_free`
- `FieldList *` - free with `kawaiifi_field_list_free`

Borrowed pointers (e.g. `const Bss *` from `kawaiifi_scan_bss_get`, `const Field *` from `kawaiifi_field_subfield_get`) are valid only for the lifetime of the parent object and must not be freed.

## .NET

[`Kawaiifi.Net`](../dotnet/) is a .NET wrapper around `kawaiifi-ffi`.

It handles all P/Invoke interop, memory management, and platform
differences internally. Callers never need to write unsafe code or manage
native memory directly.

```csharp
using Kawaiifi.Net;

using var defaultInterface = Interface.Default();

if (OperatingSystem.IsLinux())
{
    using var scan = defaultInterface?.Scan(Backend.NetworkManager);
    Console.WriteLine($"Found {scan?.BssList.Count} BSS(s)");
}

if (OperatingSystem.IsMacOS() || OperatingSystem.IsWindows())
{
    using var scan = defaultInterface?.Scan();
    Console.WriteLine($"Found {scan?.BssList.Count} BSS(s)");
}
```

See the `Kawaiifi.Net` [README](../dotnet/README.md) for build instructions
and platform-specific API details.

## Troubleshooting

See the repository [troubleshooting notes](../README.md#troubleshooting) for
platform-specific permissions and location-services behavior.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
