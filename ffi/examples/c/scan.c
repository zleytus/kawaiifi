#include "kawaiifi.h"
#include <inttypes.h>
#include <stdint.h>
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

    uintptr_t bss_count = kawaiifi_scan_bss_count(scan);
    int64_t scan_duration_ms = kawaiifi_scan_end_time_utc_ms(scan) -
                               kawaiifi_scan_start_time_utc_ms(scan);

    printf("Found %zu BSS(s) in %" PRId64 " ms\n", bss_count, scan_duration_ms);

    kawaiifi_scan_free(scan);
    kawaiifi_interface_free(interface);
    return 0;
}
