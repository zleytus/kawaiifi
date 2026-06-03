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

    kawaiifi_scan_free(scan);
    kawaiifi_interface_free(interface);
    return 0;
}
