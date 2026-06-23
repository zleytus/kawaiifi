use std::{any::Any, collections::HashMap};

use kawaiifi::ScanError;

use crate::{objects::BssInternal, vendor::VendorCache};

pub struct ProcessedScan {
    pub bss_list: Vec<BssInternal>,
    pub vendor_cache: VendorCache,
}

pub async fn spawn_scan_processing(
    fetch_bss_list: impl FnOnce() -> Result<Vec<kawaiifi::Bss>, ScanError> + Send + 'static,
    vendor_cache: VendorCache,
    existing_bss_data: Vec<BssInternal>,
) -> Result<Result<ProcessedScan, ScanError>, Box<dyn Any + Send>> {
    gtk::gio::spawn_blocking(move || {
        let bss_list = fetch_bss_list()?;
        tracing::info!(bss_count = bss_list.len(), "Received scan results");
        Ok(process_scan_results(
            bss_list,
            existing_bss_data,
            vendor_cache,
        ))
    })
    .await
}

fn process_scan_results(
    bss_list: Vec<kawaiifi::Bss>,
    existing_bss_list: Vec<BssInternal>,
    mut vendor_cache: VendorCache,
) -> ProcessedScan {
    // Rebuild uptime-derived vendor matches from this scan, then use known vendors
    // to infer vendors for related BSSIDs.
    vendor_cache.clear_uptime_map();
    let mut merged_bss_list = merge_bss_lists(existing_bss_list, bss_list);
    update_vendor_cache(&mut vendor_cache, &merged_bss_list);
    fill_vendors_from_cache(&mut merged_bss_list, &mut vendor_cache);

    ProcessedScan {
        bss_list: merged_bss_list,
        vendor_cache,
    }
}

// Keep previously seen BSSs so retention filtering can happen after processing,
// while replacing matching BSSIDs with fresher scan data.
fn merge_bss_lists(
    existing_list: Vec<BssInternal>,
    new_list: Vec<kawaiifi::Bss>,
) -> Vec<BssInternal> {
    let mut existing_map: HashMap<[u8; 6], BssInternal> = existing_list
        .into_iter()
        .map(|bss| (*bss.bssid(), bss))
        .collect();

    for bss in new_list {
        if let Some(existing_bss) = existing_map.get_mut(bss.bssid()) {
            existing_bss.update(bss);
        } else {
            existing_map.insert(*bss.bssid(), BssInternal::new(bss.clone()));
        }
    }

    existing_map.into_values().collect()
}

fn update_vendor_cache(vendor_cache: &mut VendorCache, bss_list: &[BssInternal]) {
    for bss in bss_list.iter() {
        if let Some(vendor) = bss.vendor() {
            vendor_cache.insert(bss.bssid(), vendor.to_string());
            vendor_cache.insert_uptime(bss.uptime(), vendor.to_string());
        }
    }
}

/// Uses the vendor cache to fill in vendor names for any BSSs that don't have one.
/// Runs until no more BSSs can be filled, since each resolved BSS may add cache keys
/// that allow another BSS to resolve.
fn fill_vendors_from_cache(bss_list: &mut [BssInternal], vendor_cache: &mut VendorCache) {
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
