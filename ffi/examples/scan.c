#include "kawaiifi.h"
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

int main() {
    Interface *interface = kawaiifi_default_interface();
    if (!interface) {
        return -1;
    }

    Scan *scan = kawaiifi_interface_scan(interface);

    uintptr_t bss_count = kawaiifi_scan_bss_count(scan);
    int64_t scan_duration_ms = kawaiifi_scan_end_time_utc_ms(scan) -
                               kawaiifi_scan_start_time_utc_ms(scan);

    #if defined(__linux__)
    uintptr_t freqs_count = 0;
    kawaiifi_scan_freqs_mhz(scan, &freqs_count);
    char *name = kawaiifi_interface_name(interface);
    printf("Found %zu BSS(s) in %" PRId64 " ms on %zu frequencies using %s\n",
           bss_count, scan_duration_ms, freqs_count, name);
    kawaiifi_string_free(name);
    #elif defined(__APPLE__)
    char *name = kawaiifi_interface_name(interface);
    printf("Found %zu BSS(s) in %" PRId64 " ms using %s\n", bss_count,
           scan_duration_ms, name);
    kawaiifi_string_free(name);
    #elif defined(_WIN32)
    char *description = kawaiifi_interface_description(interface);
    printf("Found %zu BSS(s) in %" PRId64 " ms using %s\n", bss_count,
           scan_duration_ms, description);
    kawaiifi_string_free(description);
    #endif

    kawaiifi_scan_free(scan);
    kawaiifi_interface_free(interface);
    return 0;
}
