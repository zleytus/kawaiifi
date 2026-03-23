#include "kawaiifi.h"
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

#if defined(__linux__)
int main() {
  Interface *interface = kawaiifi_default_interface();
  if (!interface) {
    return -1;
  }
  char *interface_name = kawaiifi_interface_name(interface);

  Scan *scan = kawaiifi_interface_scan(interface, BACKEND_NETWORK_MANAGER);
  uintptr_t bss_count = kawaiifi_scan_bss_count(scan);
  uintptr_t scan_freqs_count = 0;
  const uint32_t *scan_freqs_list =
      kawaiifi_scan_freqs_mhz(scan, &scan_freqs_count);
  int64_t scan_duration_ms = kawaiifi_scan_end_time_utc_ms(scan) -
                             kawaiifi_scan_start_time_utc_ms(scan);

  printf("Found %zu BSS(s) in %" PRId64 " ms on %zu frequencies using %s\n",
         bss_count, scan_duration_ms, scan_freqs_count, interface_name);

  kawaiifi_scan_free(scan);
  kawaiifi_string_free(interface_name);
  kawaiifi_interface_free(interface);
  return 0;
}
#endif

#if defined(_WIN32)
int main() {
  Interface *interface = kawaiifi_default_interface();
  if (!interface) {
    return -1;
  }
  char *interface_description = kawaiifi_interface_description(interface);

  Scan *scan = kawaiifi_interface_scan(interface);
  uintptr_t bss_count = kawaiifi_scan_bss_count(scan);
  int64_t scan_duration_ms = kawaiifi_scan_end_time_utc_ms(scan) -
                             kawaiifi_scan_start_time_utc_ms(scan);

  printf("Found %zu BSS(s) in %" PRId64 " ms using %s\n", bss_count,
         scan_duration_ms, interface_description);

  kawaiifi_scan_free(scan);
  kawaiifi_string_free(interface_description);
  kawaiifi_interface_free(interface);
  return 0;
}
#endif
