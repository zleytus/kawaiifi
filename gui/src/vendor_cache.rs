use std::{collections::HashMap, time::Duration};

/// Caches vendor names for BSSs with locally-administered (randomized) BSSIDs that can't
/// be resolved via a direct OUI lookup. When a vendor is resolved for one BSS, it's stored
/// under several partial-BSSID keys so that related BSSs from the same AP can inherit it.
///
/// The partial-BSSID maps persist across scans. The uptime map is cleared each scan and
/// repopulated from the current scan's results, since TSF uptime values change over time.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VendorCache {
    /// Keyed on bytes 3-5 of the BSSID. APs often increment only the last byte(s) across
    /// their virtual interfaces, so the tail bytes are shared.
    by_last3: HashMap<[u8; 3], String>,
    /// Keyed on bytes 0-4 of the BSSID.
    by_first5: HashMap<[u8; 5], String>,
    /// Keyed on bytes 0-2 and 4-5 (skipping byte 3). Covers cases where byte 3 varies
    /// across virtual interfaces but the rest of the address is stable.
    by_first_3_last_2: HashMap<[u8; 5], String>,
    /// Keyed on TSF uptime in whole seconds. Co-located virtual APs on the same physical
    /// radio share a TSF clock, so their uptimes match exactly.
    by_uptime: HashMap<u64, String>,
}

impl VendorCache {
    /// Inserts a vendor under all partial-BSSID keys for the given BSSID.
    pub fn insert(&mut self, bssid: &[u8; 6], vendor: String) {
        let last_3 = [bssid[3], bssid[4], bssid[5]];
        self.by_last3.insert(last_3, vendor.clone());

        let first_5 = [bssid[0], bssid[1], bssid[2], bssid[3], bssid[4]];
        self.by_first5.insert(first_5, vendor.clone());

        let first_3_last_2 = [bssid[0], bssid[1], bssid[2], bssid[4], bssid[5]];
        self.by_first_3_last_2
            .insert(first_3_last_2, vendor.clone());
    }

    /// Inserts a vendor under the TSF uptime key (truncated to whole seconds).
    pub fn insert_uptime(&mut self, uptime: Duration, vendor: String) {
        self.by_uptime.insert(uptime.as_secs(), vendor);
    }

    /// Looks up a vendor for the given BSSID and uptime, trying each strategy in order.
    pub fn get(&self, bssid: &[u8; 6], uptime: Duration) -> Option<String> {
        if let Some(vendor) = self.by_uptime.get(&uptime.as_secs()) {
            return Some(vendor.to_string());
        }

        let last_3 = [bssid[3], bssid[4], bssid[5]];
        if let Some(vendor) = self.by_last3.get(&last_3) {
            return Some(vendor.to_string());
        }

        let first_5 = [bssid[0], bssid[1], bssid[2], bssid[3], bssid[4]];
        if let Some(vendor) = self.by_first5.get(&first_5) {
            return Some(vendor.to_string());
        }

        let first_3_last_2 = [bssid[0], bssid[1], bssid[2], bssid[4], bssid[5]];
        if let Some(vendor) = self.by_first_3_last_2.get(&first_3_last_2) {
            return Some(vendor.to_string());
        }

        None
    }

    /// Clears the uptime map. Should be called at the start of each scan since TSF values
    /// change and stale entries would cause incorrect matches.
    pub fn clear_uptime_map(&mut self) {
        self.by_uptime.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BSSID: [u8; 6] = [0x10, 0x22, 0x33, 0x44, 0x55, 0x66];

    #[test]
    fn uptime_lookup_takes_priority_over_partial_bssid_matches() {
        let mut cache = VendorCache::default();
        cache.insert(&BSSID, "Partial Vendor".to_string());
        cache.insert_uptime(Duration::from_secs(123), "Uptime Vendor".to_string());

        let vendor = cache.get(&BSSID, Duration::from_secs(123));

        assert_eq!(vendor.as_deref(), Some("Uptime Vendor"));
    }

    #[test]
    fn lookup_matches_last_three_bssid_bytes() {
        let mut cache = VendorCache::default();
        cache.insert(&BSSID, "Tail Vendor".to_string());

        let vendor = cache.get(
            &[0xaa, 0xbb, 0xcc, 0x44, 0x55, 0x66],
            Duration::from_secs(999),
        );

        assert_eq!(vendor.as_deref(), Some("Tail Vendor"));
    }

    #[test]
    fn lookup_matches_first_five_bssid_bytes() {
        let mut cache = VendorCache::default();
        cache.insert(&BSSID, "First Five Vendor".to_string());

        let vendor = cache.get(
            &[0x10, 0x22, 0x33, 0x44, 0x55, 0xff],
            Duration::from_secs(999),
        );

        assert_eq!(vendor.as_deref(), Some("First Five Vendor"));
    }

    #[test]
    fn lookup_matches_first_three_and_last_two_bssid_bytes() {
        let mut cache = VendorCache::default();
        cache.insert(&BSSID, "Skipped Byte Vendor".to_string());

        let vendor = cache.get(
            &[0x10, 0x22, 0x33, 0xff, 0x55, 0x66],
            Duration::from_secs(999),
        );

        assert_eq!(vendor.as_deref(), Some("Skipped Byte Vendor"));
    }

    #[test]
    fn clear_uptime_map_keeps_partial_bssid_matches() {
        let mut cache = VendorCache::default();
        cache.insert(&BSSID, "Partial Vendor".to_string());
        cache.insert_uptime(Duration::from_secs(123), "Uptime Vendor".to_string());

        cache.clear_uptime_map();

        let vendor = cache.get(&BSSID, Duration::from_secs(123));
        assert_eq!(vendor.as_deref(), Some("Partial Vendor"));
    }
}
