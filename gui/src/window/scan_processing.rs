use std::collections::HashMap;

use kawaiifi::IeData;

use crate::{objects::BssInternal, oui, vendor_cache::VendorCache};

const BSSID_VENDOR_MASKS: [u8; 4] = [0x02, 0x04, 0x06, 0b1110];
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

fn is_likely_bss_vendor(vendor: &str) -> bool {
    let vendor = vendor.to_lowercase();
    !NON_BSS_VENDOR_IE_NAMES
        .iter()
        .any(|ignored_vendor| vendor.contains(ignored_vendor))
}

/// Look up vendor for a BSS via OUI database, bit-flip variants, and VendorSpecific IEs.
pub(super) fn lookup_vendor(bss: &kawaiifi::Bss) -> Option<String> {
    for candidate in bssid_vendor_candidates(bss.bssid()) {
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

pub(super) fn merge_new_scan_results_with_existing(
    existing: &[BssInternal],
    scan_bss_list: &[kawaiifi::Bss],
    vendor_cache: &mut VendorCache,
) -> Vec<BssInternal> {
    let mut existing_map: HashMap<[u8; 6], BssInternal> = existing
        .iter()
        .map(|bss_internal| (*bss_internal.bssid(), bss_internal.clone()))
        .collect();

    for bss in scan_bss_list {
        if let Some(existing_bss) = existing_map.get_mut(bss.bssid()) {
            existing_bss.update(bss.clone());

            if let Some(vendor) = existing_bss.vendor() {
                vendor_cache.insert(existing_bss.bssid(), vendor.to_string());
                vendor_cache.insert_uptime(existing_bss.uptime(), vendor.to_string());
            }
        } else {
            let mut bss_internal = BssInternal::new(bss.clone());
            if let Some(vendor) = lookup_vendor(bss) {
                bss_internal.set_vendor(vendor.clone());
                vendor_cache.insert(bss_internal.bssid(), vendor.clone());
                vendor_cache.insert_uptime(bss_internal.uptime(), vendor);
            }
            existing_map.insert(*bss.bssid(), bss_internal);
        }
    }

    let mut bss_list: Vec<BssInternal> = existing_map.values().cloned().collect();

    fill_vendors_from_cache(&mut bss_list, vendor_cache);

    bss_list
}

/// Uses the vendor cache to fill in vendor names for any BSSs that don't have one.
/// Runs until no more BSSs can be filled, since each resolved BSS may add cache keys
/// that allow another BSS to resolve.
pub(super) fn fill_vendors_from_cache(
    bss_list: &mut [BssInternal],
    vendor_cache: &mut VendorCache,
) {
    loop {
        let mut filled_any = false;

        for bss in bss_list.iter_mut() {
            if bss.vendor().is_some() {
                continue;
            }

            if let Some(vendor) = vendor_cache.get(bss.bssid(), bss.uptime()) {
                bss.set_vendor(vendor.clone());
                vendor_cache.insert(bss.bssid(), vendor.clone());
                vendor_cache.insert_uptime(bss.uptime(), vendor);
                filled_any = true;
            }
        }

        if !filled_any {
            break;
        }
    }
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
