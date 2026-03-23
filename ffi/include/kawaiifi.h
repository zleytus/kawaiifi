#ifndef kawaiifi_h
#define kawaiifi_h

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#ifdef _WIN32
#include <guiddef.h>
#endif
typedef struct Interface Interface;
typedef struct Bss Bss;
typedef struct Scan Scan;
typedef struct Ie Ie;
typedef struct FieldList FieldList;
typedef struct InterfaceList InterfaceList;

/**
 * FFI-safe equivalent of kawaiifi::Band.
 */
typedef enum Band {
  BAND_TWO_POINT_FOUR_GHZ,
  BAND_FIVE_GHZ,
  BAND_SIX_GHZ,
  BAND_UNKNOWN,
} Band;

/**
 * FFI-safe equivalent of kawaiifi::ChannelWidth.
 */
typedef enum ChannelWidth {
  CHANNEL_WIDTH_TWENTY_MHZ,
  CHANNEL_WIDTH_FORTY_MHZ,
  CHANNEL_WIDTH_EIGHTY_MHZ,
  CHANNEL_WIDTH_EIGHTY_PLUS_EIGHTY_MHZ,
  CHANNEL_WIDTH_ONE_SIXTY_MHZ,
  CHANNEL_WIDTH_THREE_HUNDRED_TWENTY_MHZ,
  CHANNEL_WIDTH_UNKNOWN,
} ChannelWidth;

#if defined(__linux__)
/**
 * FFI-safe equivalent of kawaiifi::BusType.
 */
typedef enum BusType {
#if defined(__linux__)
  BUS_TYPE_PCI,
#endif
#if defined(__linux__)
  BUS_TYPE_USB,
#endif
#if defined(__linux__)
  BUS_TYPE_SDIO,
#endif
#if defined(__linux__)
  BUS_TYPE_UNKNOWN,
#endif
} BusType;
#endif

#if defined(__linux__)
/**
 * FFI-safe equivalent of kawaiifi::scan::Backend.
 */
typedef enum Backend {
#if defined(__linux__)
  BACKEND_NL80211,
#endif
#if defined(__linux__)
  BACKEND_NETWORK_MANAGER,
#endif
} Backend;
#endif

/**
 * FFI-safe equivalent of kawaiifi::CapabilityInfo.
 */
typedef struct CapabilityInfo {
  bool ess;
  bool ibss;
  bool privacy;
  bool short_preamble;
  bool critical_update_flag;
  bool nontransmitted_bssids_critical_update_flag;
  bool spectrum_management;
  bool qos;
  bool short_slot_time;
  bool apsd;
  bool radio_measurement;
  bool epd;
} CapabilityInfo;
#define CapabilityInfo_LENGTH 2

#if defined(__linux__)
typedef struct Flags {
  /**
   * The scan can be delayed or paused to allow normal data transmission
   * or other higher priority operations to proceed.
   */
  bool low_priority;
  /**
   * Flush cached scan results before starting a new scan.
   *
   * When set, the driver will discard previously cached BSS entries
   * before reporting new scan results.
   */
  bool flush;
  /**
   * Force a scan even if the interface is an AP.
   *
   * Indicates this scan was initiated by an AP, which may have
   * different scanning behavior than client devices.
   */
  bool ap;
  /**
   * Use a random MAC address for probe requests.
   *
   * Privacy feature that randomizes the device's MAC address during
   * active scanning to prevent tracking across networks.
   */
  bool random_addr;
  /**
   * Fill the dwell time in the FILS request parameters IE in the probe request
   */
  bool fils_max_channel_time;
  /**
   * Accept broadcast probe responses.
   */
  bool accept_bcast_probe_resp;
  /**
   * Send probe request frames at rate of at least 5.5M.
   */
  bool oce_probe_req_high_tx_rate;
  /**
   * Allow probe request tx deferral and suppression.
   */
  bool oce_probe_req_deferral_suppression;
  /**
   * Perform the scan with minimal time on each channel.
   */
  bool low_span;
  /**
   * Perform the scan with lower power.
   */
  bool low_power;
  /**
   * Perform the scan with highest accuracy to find all available networks.
   */
  bool high_accuracy;
  /**
   * Use random sequence numbers in probe requests.
   */
  bool random_sn;
  /**
   * Use minimal content in probe requests.
   */
  bool min_preq_content;
  /**
   * Frequencies specified in kHz (not MHz).
   */
  bool freq_khz;
  /**
   * Discover colocated 6 GHz APs through RNR.
   */
  bool colocated_6ghz;
} Flags;
#endif

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Returns a borrowed pointer to the BSS's 6-byte BSSID (MAC address), or null if `bss` is null.
 * The pointer is valid for the lifetime of the BSS. Do NOT free it.
 */
const uint8_t *kawaiifi_bss_bssid(const struct Bss *bss);

/**
 * Returns the SSID as a C string, or null if not present.
 * The caller must free the returned string with `kawaiifi_string_free`.
 */
char *kawaiifi_bss_ssid(const struct Bss *bss);

/**
 * Returns the operating frequency of the BSS in MHz, or 0 if `bss` is null.
 */
uint32_t kawaiifi_bss_frequency_mhz(const struct Bss *bss);

/**
 * Returns the band the BSS operates on (2.4 GHz, 5 GHz, or 6 GHz), or `BAND_UNKNOWN` if `bss` is null.
 */
enum Band kawaiifi_bss_band(const struct Bss *bss);

/**
 * Returns the channel width of the BSS, or `CHANNEL_WIDTH_UNKNOWN` if `bss` is null.
 */
enum ChannelWidth kawaiifi_bss_channel_width(const struct Bss *bss);

/**
 * Returns the center frequency of the BSS in MHz, or 0 if `bss` is null.
 */
uint32_t kawaiifi_bss_center_frequency_mhz(const struct Bss *bss);

/**
 * Returns the channel number of the BSS, or 0 if `bss` is null.
 */
uint8_t kawaiifi_bss_channel_number(const struct Bss *bss);

/**
 * Returns the signal strength of the BSS in dBm, or 0 if `bss` is null.
 */
int32_t kawaiifi_bss_signal_dbm(const struct Bss *bss);

/**
 * Returns the beacon interval of the BSS in time units (TUs, 1 TU = 1024 µs), or 0 if `bss` is null.
 */
uint16_t kawaiifi_bss_beacon_interval_tu(const struct Bss *bss);

/**
 * Returns the beacon interval of the BSS in milliseconds, or 0.0 if `bss` is null.
 */
double kawaiifi_bss_beacon_interval_ms(const struct Bss *bss);

/**
 * Returns the 802.11 capability information flags for the BSS.
 */
struct CapabilityInfo kawaiifi_bss_capability_info(const struct Bss *bss);

/**
 * Returns the timing synchronization function (TSF) timer value of the BSS, or 0 if `bss` is null.
 */
uint64_t kawaiifi_bss_tsf(const struct Bss *bss);

/**
 * Writes the Unix timestamp (milliseconds) of when the BSS was last seen into `out`.
 * Returns false if the timestamp is unavailable or `bss` is null.
 */
bool kawaiifi_bss_last_seen_utc_ms(const struct Bss *bss, int64_t *out);

/**
 * Returns the security protocols as a bitmask (WEP=1, WPA=2, WPA2=4, WPA3=8).
 */
uint8_t kawaiifi_bss_security_protocols(const struct Bss *bss);

/**
 * Returns the Wi-Fi protocols as a bitmask (a=1, b=2, g=4, n=8, ac=16, ax=32, be=64).
 */
uint16_t kawaiifi_bss_wifi_protocols(const struct Bss *bss);

/**
 * Returns the maximum supported data rate of the BSS in Mbps, or 0.0 if `bss` is null.
 */
double kawaiifi_bss_max_rate_mbps(const struct Bss *bss);

/**
 * Returns the number of information elements in the BSS, or 0 if `bss` is null.
 */
uintptr_t kawaiifi_bss_ie_count(const struct Bss *bss);

/**
 * Returns a borrowed pointer to the information element at `index`, or null if out of bounds or `bss` is null.
 * The pointer is valid for the lifetime of the BSS. Do NOT free it.
 */
const struct Ie *kawaiifi_bss_ie_get(const struct Bss *bss,
                                     uintptr_t index);

/**
 * Frees a string returned by any kawaiifi function.
 */
void kawaiifi_string_free(char *s);

/**
 * Frees a byte buffer returned by any kawaiifi function.
 */
void kawaiifi_bytes_free(uint8_t *ptr, uintptr_t count);

/**
 * Returns the number of fields in the list, or 0 if `list` is null.
 */
uintptr_t kawaiifi_field_list_count(const struct FieldList *list);

/**
 * Returns a borrowed pointer to the field at `index`, or null if out of bounds or `list` is null.
 * The pointer is valid for the lifetime of the list. Do NOT free it.
 */
const struct Field *kawaiifi_field_list_get(const struct FieldList *list, uintptr_t index);

/**
 * Frees a field list returned by `kawaiifi_ie_fields`. Does nothing if `list` is null.
 */
void kawaiifi_field_list_free(struct FieldList *list);

/**
 * Returns the field's title as a C string. The caller must free with `kawaiifi_string_free`.
 */
char *kawaiifi_field_title(const struct Field *field);

/**
 * Returns the field's value as a C string. The caller must free with `kawaiifi_string_free`.
 */
char *kawaiifi_field_value(const struct Field *field);

/**
 * Writes the field's byte into `out`. Returns false if unavailable or `field` is null.
 */
bool kawaiifi_field_byte(const struct Field *field, uint8_t *out);

/**
 * Returns the field's raw bytes as a borrowed pointer. Valid for the lifetime of the field.
 * Do NOT free this pointer.
 */
const uint8_t *kawaiifi_field_bytes(const struct Field *field, uintptr_t *out_count);

/**
 * Returns a formatted bit range string. The caller must free with `kawaiifi_string_free`.
 */
char *kawaiifi_field_bits(const struct Field *field);

/**
 * Returns the field's units as a C string. The caller must free with `kawaiifi_string_free`.
 */
char *kawaiifi_field_units(const struct Field *field);

/**
 * Returns the number of subfields in the field, or 0 if `field` is null.
 */
uintptr_t kawaiifi_field_subfield_count(const struct Field *field);

/**
 * Returns a borrowed pointer to the subfield at `index`, or null if out of bounds or `field` is null.
 * The pointer is valid for the lifetime of the field. Do NOT free it.
 */
const struct Field *kawaiifi_field_subfield_get(const struct Field *field,
                                                uintptr_t index);

/**
 * Returns the element ID of the information element, or 0 if `ie` is null.
 */
uint8_t kawaiifi_ie_id(const struct Ie *ie);

/**
 * Returns the length in bytes of the information element's data, or 0 if `ie` is null.
 */
uint8_t kawaiifi_ie_len(const struct Ie *ie);

/**
 * Writes the extended element ID into `out`. Returns false if the IE has no extension ID or `ie` is null.
 */
bool kawaiifi_ie_id_ext(const struct Ie *ie,
                        uint8_t *out);

/**
 * Returns the IE's name as a null-terminated C string, or null if `ie` is null.
 * The caller must free the returned string with `kawaiifi_string_free`.
 */
char *kawaiifi_ie_name(const struct Ie *ie);

/**
 * Returns the raw bytes of the IE. The caller must free with `kawaiifi_bytes_free`.
 */
uint8_t *kawaiifi_ie_bytes(const struct Ie *ie, uintptr_t *out_count);

/**
 * Returns the IE's summary as a C string. The caller must free with `kawaiifi_string_free`.
 */
char *kawaiifi_ie_summary(const struct Ie *ie);

/**
 * Returns the IE's fields as an opaque FieldList. The caller must free with `kawaiifi_field_list_free`.
 * Returns null if the IE is null or has no fields.
 */
struct FieldList *kawaiifi_ie_fields(const struct Ie *ie);

/**
 * Returns all available wireless interfaces as an opaque list.
 * The caller must free the returned list with `kawaiifi_interface_list_free`.
 */
struct InterfaceList *kawaiifi_interfaces(void);

/**
 * Returns the number of interfaces in the list, or 0 if `list` is null.
 */
uintptr_t kawaiifi_interface_list_count(const struct InterfaceList *list);

/**
 * Returns a borrowed pointer to the interface at `index`, or null if out of bounds or `list` is null.
 * The pointer is valid for the lifetime of the list. Do NOT free it with `kawaiifi_interface_free`.
 */
const struct Interface *kawaiifi_interface_list_get(const struct InterfaceList *list,
                                                    uintptr_t index);

/**
 * Frees an interface list returned by `kawaiifi_interfaces`. Does nothing if `list` is null.
 */
void kawaiifi_interface_list_free(struct InterfaceList *list);

/**
 * Returns the default wireless interface, or null if none is found.
 * The caller must free the returned interface with `kawaiifi_interface_free`.
 */
struct Interface *kawaiifi_default_interface(void);

/**
 * Frees an interface returned by `kawaiifi_default_interface`. Does nothing if `interface` is null.
 */
void kawaiifi_interface_free(struct Interface *interface);

/**
 * Returns the number of BSSes in the scan, or 0 if `scan` is null.
 */
uintptr_t kawaiifi_scan_bss_count(const struct Scan *scan);

/**
 * Returns a borrowed pointer to the BSS at `index`, or null if out of bounds or `scan` is null.
 * The pointer is valid for the lifetime of the scan. Do NOT free it.
 */
const struct Bss *kawaiifi_scan_bss_get(const struct Scan *scan, uintptr_t index);

/**
 * Returns the start time of the scan as a Unix timestamp in milliseconds, or 0 if `scan` is null.
 */
int64_t kawaiifi_scan_start_time_utc_ms(const struct Scan *scan);

/**
 * Returns the end time of the scan as a Unix timestamp in milliseconds, or 0 if `scan` is null.
 */
int64_t kawaiifi_scan_end_time_utc_ms(const struct Scan *scan);

/**
 * Frees a scan returned by `kawaiifi_interface_scan`. Does nothing if `scan` is null.
 */
void kawaiifi_scan_free(struct Scan *scan);

#if defined(__linux__)
/**
 * Returns true if the BSS information came from a probe response, or false if from a beacon or if `bss` is null.
 */
bool kawaiifi_bss_is_from_probe_response(const struct Bss *bss);
#endif

#if defined(__linux__)
/**
 * Returns a borrowed pointer to the 6-byte parent BSSID, or null if unavailable or `bss` is null.
 * The pointer is valid for the lifetime of the BSS. Do NOT free it.
 */
const uint8_t *kawaiifi_bss_parent_bssid(const struct Bss *bss);
#endif

#if defined(__linux__)
/**
 * Writes the TSF timer value of the transmitting BSS into `out`. Returns false if unavailable or `bss` is null.
 */
bool kawaiifi_bss_parent_tsf(const struct Bss *bss,
                             uint64_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the TSF timer value from the last beacon into `out`. Returns false if unavailable or `bss` is null.
 */
bool kawaiifi_bss_beacon_tsf(const struct Bss *bss,
                             uint64_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the frequency offset of the BSS in kHz into `out`. Returns false if unavailable or `bss` is null.
 */
bool kawaiifi_bss_frequency_offset_khz(const struct Bss *bss,
                                       uint32_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the signal strength as a percentage (0–100) into `out`. Returns false if unavailable or `bss` is null.
 */
bool kawaiifi_bss_signal_percent(const struct Bss *bss,
                                 uint8_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the time the BSS was last seen as nanoseconds since boot into `out`.
 * Returns false if unavailable or `bss` is null.
 */
bool kawaiifi_bss_last_seen_boottime_ns(const struct Bss *bss, uint64_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the number of milliseconds since the BSS was last seen into `out`.
 * Returns false if unavailable or `bss` is null.
 */
bool kawaiifi_bss_seen_ms_ago(const struct Bss *bss, uint32_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the Multi-Link Operation (MLO) link ID into `out`. Returns false if unavailable or `bss` is null.
 */
bool kawaiifi_bss_mlo_link_id(const struct Bss *bss,
                              uint8_t *out);
#endif

#if defined(__linux__)
/**
 * Returns a borrowed pointer to the 6-byte MLD address, or null if unavailable or `bss` is null.
 * The pointer is valid for the lifetime of the BSS. Do NOT free it.
 */
const uint8_t *kawaiifi_bss_mld_address(const struct Bss *bss);
#endif

#if defined(__linux__)
/**
 * Returns the interface name as a C string, or null if `interface` is null.
 * The caller must free the returned string with `kawaiifi_string_free`.
 */
char *kawaiifi_interface_name(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Returns the kernel interface index (ifindex), or 0 if `interface` is null.
 */
uint32_t kawaiifi_interface_index(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Returns the wiphy index of the physical radio this interface belongs to, or 0 if `interface` is null.
 */
uint32_t kawaiifi_interface_wiphy(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Returns the wireless device identifier (wdev), or 0 if `interface` is null.
 */
uint64_t kawaiifi_interface_wdev(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Writes the 6-byte MAC address into `out`. Does nothing if either argument is null.
 */
void kawaiifi_interface_mac_address(const struct Interface *interface, uint8_t *out);
#endif

#if defined(__linux__)
/**
 * Returns the netlink generation counter for this interface, or 0 if `interface` is null.
 */
uint32_t kawaiifi_interface_generation(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Returns true if the interface is operating in 4-address (WDS) mode, or false if `interface` is null.
 */
bool kawaiifi_interface_four_address(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Returns the SSID as a C string, or null if not associated or `interface` is null.
 * The caller must free the returned string with `kawaiifi_string_free`.
 */
char *kawaiifi_interface_ssid(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Writes the current operating frequency of the radio in MHz into `out`.
 * Returns false if unavailable or `interface` is null.
 */
bool kawaiifi_interface_wiphy_freq_mhz(const struct Interface *interface, uint32_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the frequency offset of the radio in kHz into `out`.
 * Returns false if unavailable or `interface` is null.
 */
bool kawaiifi_interface_wiphy_freq_offset_khz(const struct Interface *interface, uint32_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the transmit power level in mBm (100 * dBm) into `out`.
 * Returns false if unavailable or `interface` is null.
 */
bool kawaiifi_interface_wiphy_tx_power_level_mbm(const struct Interface *interface, uint32_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the primary center frequency in MHz into `out`.
 * Returns false if unavailable or `interface` is null.
 */
bool kawaiifi_interface_center_freq_1_mhz(const struct Interface *interface, uint32_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the secondary center frequency in MHz into `out` (used for 80+80 MHz channels).
 * Returns false if unavailable or `interface` is null.
 */
bool kawaiifi_interface_center_freq_2_mhz(const struct Interface *interface, uint32_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the channel width into `out`. Returns false if unavailable or `interface` is null.
 */
bool kawaiifi_interface_channel_width(const struct Interface *interface, enum ChannelWidth *out);
#endif

#if defined(__linux__)
/**
 * Writes the virtual interface radio mask into `out`. Returns false if unavailable or `interface` is null.
 */
bool kawaiifi_interface_vif_radio_mask(const struct Interface *interface,
                                       uint32_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the PCI/USB vendor ID into `out`. Returns false if unavailable or `interface` is null.
 */
bool kawaiifi_interface_vendor_id(const struct Interface *interface, uint16_t *out);
#endif

#if defined(__linux__)
/**
 * Writes the PCI/USB device ID into `out`. Returns false if unavailable or `interface` is null.
 */
bool kawaiifi_interface_device_id(const struct Interface *interface, uint16_t *out);
#endif

#if defined(__linux__)
/**
 * Returns the vendor name as a C string, or null if unavailable.
 * The caller must free the returned string with `kawaiifi_string_free`.
 */
char *kawaiifi_interface_vendor_name(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Returns the device name as a C string, or null if unavailable.
 * The caller must free the returned string with `kawaiifi_string_free`.
 */
char *kawaiifi_interface_device_name(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Returns the driver name as a C string, or null if unavailable.
 * The caller must free the returned string with `kawaiifi_string_free`.
 */
char *kawaiifi_interface_driver(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Returns the bus type (PCI, USB, SDIO) the wireless adapter is connected via.
 */
enum BusType kawaiifi_interface_bus_type(const struct Interface *interface);
#endif

#if defined(__linux__)
/**
 * Performs a blocking scan and returns the result, or null on error.
 * The caller must free the returned scan with `kawaiifi_scan_free`.
 */
struct Scan *kawaiifi_interface_scan(const struct Interface *interface, enum Backend backend);
#endif

#if defined(__linux__)
/**
 * Returns the wiphy index of the radio that performed the scan, or 0 if `scan` is null.
 */
uint32_t kawaiifi_scan_wiphy(const struct Scan *scan);
#endif

#if defined(__linux__)
/**
 * Returns the interface index (ifindex) that performed the scan, or 0 if `scan` is null.
 */
uint32_t kawaiifi_scan_ifindex(const struct Scan *scan);
#endif

#if defined(__linux__)
/**
 * Returns a borrowed pointer to the frequencies (in MHz) that were scanned and writes the count into `out_count`.
 * The pointer is valid for the lifetime of the scan. Do NOT free it.
 */
const uint32_t *kawaiifi_scan_freqs_mhz(const struct Scan *scan,
                                        uintptr_t *out_count);
#endif

#if defined(__linux__)
/**
 * Returns the number of information elements requested in the scan probe, or 0 if `scan` is null.
 */
uintptr_t kawaiifi_scan_ie_count(const struct Scan *scan);
#endif

#if defined(__linux__)
/**
 * Returns a borrowed pointer to the information element at `index`, or null if out of bounds or `scan` is null.
 * The pointer is valid for the lifetime of the scan. Do NOT free it.
 */
const struct Ie *kawaiifi_scan_ie_get(const struct Scan *scan,
                                      uintptr_t index);
#endif

#if defined(__linux__)
/**
 * Writes the scan flags into `out`. Returns false if unavailable or `scan` is null.
 */
bool kawaiifi_scan_flags(const struct Scan *scan, struct Flags *out);
#endif

#if defined(_WIN32)
/**
 * Returns the link quality of the BSS as a value from 0 to 100, or 0 if `bss` is null.
 */
uint8_t kawaiifi_bss_link_quality(const struct Bss *bss);
#endif

#if defined(_WIN32)
/**
 * Returns the GUID of the network interface, or a zeroed GUID if `interface` is null.
 */
GUID kawaiifi_interface_guid(const struct Interface *interface);
#endif

#if defined(_WIN32)
/**
 * Returns the human-readable description of the network interface as a C string, or null if `interface` is null.
 * The caller must free the returned string with `kawaiifi_string_free`.
 */
char *kawaiifi_interface_description(const struct Interface *interface);
#endif

#if defined(_WIN32)
/**
 * Performs a blocking scan and returns the result, or null on error.
 * The caller must free the returned scan with `kawaiifi_scan_free`.
 */
struct Scan *kawaiifi_interface_scan(const struct Interface *interface);
#endif

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* kawaiifi_h */
