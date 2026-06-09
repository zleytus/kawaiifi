#include "kawaiifi.h"
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

static void print_field(const Field *field, int indent) {
    char *title = kawaiifi_field_title(field);
    char *value = kawaiifi_field_value(field);

    printf("%*s%s: %s\n", indent, "", title, value);

    kawaiifi_string_free(title);
    kawaiifi_string_free(value);

    uintptr_t subfield_count = kawaiifi_field_subfield_count(field);
    for (uintptr_t i = 0; i < subfield_count; ++i) {
        print_field(kawaiifi_field_subfield_get(field, i), indent + 2);
    }
}

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

            printf("%s (%" PRIu8 ")", ie_name, ie_id);
            printf(" - %s\n", ie_summary);

            kawaiifi_string_free(ie_name);
            kawaiifi_string_free(ie_summary);

            FieldList *fields = kawaiifi_ie_fields(ie);
            uintptr_t field_count = kawaiifi_field_list_count(fields);
            for (uintptr_t k = 0; k < field_count; ++k) {
                print_field(kawaiifi_field_list_get(fields, k), 2);
            }
            kawaiifi_field_list_free(fields);
        }
        printf("\n");
    }

    kawaiifi_scan_free(scan);
    kawaiifi_interface_free(interface);
    return 0;
}
