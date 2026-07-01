use kawaiifi::{Bss, IeData};

use super::oui;

// Some APs derive BSSIDs by toggling local/admin or virtual-interface bits in
// the first octet, so try common variants before falling back to IE OUIs.
const BSSID_VENDOR_MASKS: [u8; 4] = [0x02, 0x04, 0x06, 0b1110];
// These vendors commonly appear in VendorSpecific IEs for protocol extensions,
// but their OUI usually identifies the IE owner rather than the AP manufacturer.
const NON_BSS_VENDOR_IE_NAMES: [&str; 5] =
    ["microsoft", "epigram", "broadcom", "wi-fi", "qualcomm"];

fn bssid_vendor_candidates(bssid: &[u8; 6]) -> Vec<[u8; 6]> {
    let mut candidates = Vec::with_capacity(BSSID_VENDOR_MASKS.len() + 1);
    candidates.push(*bssid);

    let original_first_octet = bssid[0];
    for mask in BSSID_VENDOR_MASKS {
        let mut candidate = *bssid;
        candidate[0] = original_first_octet ^ mask;
        candidates.push(candidate);
    }

    candidates
}

fn bss_vendor_candidates(bss: &Bss) -> Vec<[u8; 6]> {
    let mut candidates = bssid_vendor_candidates(bss.bssid());
    if let Some(parent_bssid) = bss.parent_bssid() {
        for candidate in bssid_vendor_candidates(&parent_bssid) {
            if !candidates.contains(&candidate) {
                candidates.push(candidate);
            }
        }
    }

    candidates
}

fn is_likely_bss_vendor(vendor: &str) -> bool {
    let vendor = vendor.to_lowercase();
    !NON_BSS_VENDOR_IE_NAMES
        .iter()
        .any(|ignored_vendor| vendor.contains(ignored_vendor))
}

/// Look up vendor for a BSS via OUI database, bit-flip variants, and VendorSpecific IEs.
pub fn lookup_vendor(bss: &Bss) -> Option<String> {
    for candidate in bss_vendor_candidates(bss) {
        if let Some(vendor) = oui::lookup_vendor(&candidate) {
            return Some(vendor);
        }
    }

    for oui in bss.ies().iter().filter_map(|ie| match &ie.data {
        IeData::VendorSpecific(vendor_specific) => vendor_specific.oui(),
        _ => None,
    }) {
        if let Some(vendor) = oui::lookup_vendor(oui) {
            // Ignore OUIs of common VendorSpecific IEs that don't actually indicate
            // the BSS's vendor.
            if is_likely_bss_vendor(&vendor) {
                return Some(vendor);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bssid_vendor_candidates_include_original_and_masked_variants() {
        let candidates = bssid_vendor_candidates(&[0x10, 0x22, 0x33, 0x44, 0x55, 0x66]);

        assert_eq!(
            candidates,
            vec![
                [0x10, 0x22, 0x33, 0x44, 0x55, 0x66],
                [0x12, 0x22, 0x33, 0x44, 0x55, 0x66],
                [0x14, 0x22, 0x33, 0x44, 0x55, 0x66],
                [0x16, 0x22, 0x33, 0x44, 0x55, 0x66],
                [0x1e, 0x22, 0x33, 0x44, 0x55, 0x66],
            ]
        );
    }

    #[test]
    fn vendor_specific_filter_rejects_common_ie_vendors() {
        assert!(!is_likely_bss_vendor("Microsoft Corp."));
        assert!(!is_likely_bss_vendor("Broadcom Limited"));
        assert!(!is_likely_bss_vendor("Wi-Fi Alliance"));
        assert!(!is_likely_bss_vendor("QUALCOMM Inc."));
    }

    #[test]
    fn vendor_specific_filter_accepts_specific_device_vendors() {
        assert!(is_likely_bss_vendor("Aruba Networks"));
        assert!(is_likely_bss_vendor("Ubiquiti Inc."));
    }
}
