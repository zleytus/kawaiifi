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

    kawaiifi_scan_free(scan);
    kawaiifi_interface_free(interface);
    return 0;
}
