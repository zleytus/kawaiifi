# kawaiifi-ffi

`kawaiifi-ffi` is a Wi-Fi scanning library for Linux, macOS, and Windows.

It exposes a C-compatible FFI for [`kawaiifi`](../lib).

## Building

```sh
cargo build -p kawaiifi-ffi --release
```

This produces a static library (`libkawaiifi.a` / `kawaiifi.lib`) and a shared library (`libkawaiifi.so` / `libkawaiifi.dylib` / `kawaiifi.dll`) under `target/release/`, and regenerates `include/kawaiifi.h`.

## Usage

Include [`kawaiifi.h`](include/kawaiifi.h) and link against either the static or shared library.

### Obtaining a Wi-Fi Interface

Use `kawaiifi_default_interface` to get the first available interface.

```c
Interface *interface = kawaiifi_default_interface();
if (!interface) {
    return -1;
}
```

Use `kawaiifi_interfaces` to get all available interfaces.

```c
InterfaceList *interfaces = kawaiifi_interfaces();
printf("Found %zu interface(s)\n", kawaiifi_interface_list_count(interfaces));

kawaiifi_interface_list_free(interfaces);
```

Some `Interface` properties are platform-specific.

```c
#if defined(__linux__)
printf("Index: %" PRIu32 "\n", kawaiifi_interface_index(interface));
#elif defined(__APPLE__)
printf("Noise: %" PRIi32 " dBm\n", kawaiifi_interface_noise_dbm(interface));
#elif defined(_WIN32)
char *description = kawaiifi_interface_description(interface);
printf("Description: %s\n", description);
kawaiifi_string_free(description);
#endif
```

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

`Scan` contains a list of BSSs that are accessed through `kawaiifi_scan_bss_get`.

```c
uintptr_t bss_count = kawaiifi_scan_bss_count(scan);
for (uintptr_t i = 0; i < bss_count; ++i) {
    const Bss *bss = kawaiifi_scan_bss_get(scan, i);
}
```

`Bss` exposes common properties that are available on all platforms.

```c
char *ssid = kawaiifi_bss_ssid(bss);
if (ssid) {
    printf("SSID: %s\n", ssid);
    kawaiifi_string_free(ssid);
}
printf("Frequency: %" PRIu32 " MHz\n", kawaiifi_bss_frequency_mhz(bss));
printf("Channel: %" PRIu8 "\n", kawaiifi_bss_channel_number(bss));
printf("Signal: %" PRIi32 " dBm\n", kawaiifi_bss_signal_dbm(bss));
printf("Max Rate: %lf Mbps\n", kawaiifi_bss_max_rate_mbps(bss));
printf("\n");
```

Some `Bss` properties are platform-specific.

```c
#if defined(__linux__)
printf("Status: %d\n", kawaiifi_bss_status(bss));
#elif defined(__APPLE__)
printf("Noise: %" PRIi32 " dBm\n", kawaiifi_bss_noise_dbm(bss));
#elif defined(_WIN32)
printf("Link Quality: %" PRIu8 "\n", kawaiifi_bss_link_quality(bss));
#endif
```

See [`bss_data.c`](examples/bss_data.c).

### Accessing Information Elements

`Bss` contains a list of 802.11 Information Elements (IEs)
that are accessed through `kawaiifi_bss_ie_get`.

```c
uintptr_t ie_count = kawaiifi_bss_ie_count(bss);
printf("Found %zu IE(s)\n", ie_count);
```

`Ie` exposes basic properties such as the information element's name, ID,
and a summary.

```c
const Ie *ie = kawaiifi_bss_ie_get(bss, j);
char *ie_name = kawaiifi_ie_name(ie);
uint8_t ie_id = kawaiifi_ie_id(ie);
char *ie_summary = kawaiifi_ie_summary(ie);

printf("%s (%" PRIu8 ") - %s\n", ie_name, ie_id, ie_summary);

kawaiifi_string_free(ie_name);
kawaiifi_string_free(ie_summary);
```

`Ie` also exposes its decoded fields through `kawaiifi_ie_fields`. Each `Field`
has a title, value, optional units, and nested subfields.

```c
FieldList *fields = kawaiifi_ie_fields(ie);
uintptr_t field_count = kawaiifi_field_list_count(fields);
for (uintptr_t k = 0; k < field_count; ++k) {
    const Field *field = kawaiifi_field_list_get(fields, k);
    char *title = kawaiifi_field_title(field);
    char *value = kawaiifi_field_value(field);

    printf("%s: %s\n", title, value);

    kawaiifi_string_free(title);
    kawaiifi_string_free(value);
}
kawaiifi_field_list_free(fields);
```

See [`ies.c`](examples/ies.c).

## Memory management

Functions that return heap-allocated values document how to free them:

- Strings (`char *`) - free with `kawaiifi_string_free`
- Byte buffers (`uint8_t *` with a count) - free with `kawaiifi_bytes_free`
- `Scan *` - free with `kawaiifi_scan_free`
- `Interface *` - free with `kawaiifi_interface_free`
- `InterfaceList *` - free with `kawaiifi_interface_list_free`
- `BssList *` - free with `kawaiifi_bss_list_free`
- `FieldList *` - free with `kawaiifi_field_list_free`

Borrowed pointers (e.g. `const Bss *` from `kawaiifi_scan_bss_get`, `const Field *` from `kawaiifi_field_subfield_get`) are valid only for the lifetime of the parent object and must not be freed.

## Platform-Specific APIs

`kawaiifi-ffi` exposes platform-specific APIs through preprocessor guards in
[`kawaiifi.h`](include/kawaiifi.h). Call platform-specific functions from the
matching `#if` block.

For example, on Linux and macOS, `Interface` has a name, while on Windows,
`Interface` has a description.

```c
#if defined(__linux__) || defined(__APPLE__)
char *name = kawaiifi_interface_name(interface);
printf("Interface name: %s\n", name);
kawaiifi_string_free(name);
#elif defined(_WIN32)
char *description = kawaiifi_interface_description(interface);
printf("Interface description: %s\n", description);
kawaiifi_string_free(description);
#endif
```

## Troubleshooting

See the repository [troubleshooting notes](../README.md#troubleshooting) for
platform-specific permissions and location-services behavior.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
