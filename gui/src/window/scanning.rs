use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, MutexGuard},
    thread,
    time::Duration,
};

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    gio::prelude::{ListModelExt, ListModelExtManual},
    glib::{self, object::ObjectExt},
    prelude::WidgetExt,
};
use kawaiifi::IeData;

use super::{KawaiiFiWindow, imp};
use crate::{
    objects::{BssInternal, BssObject},
    oui,
    scan_file::ScanFile,
    vendor_cache::VendorCache,
};

enum ScanAndProcessingResult {
    Ok {
        scan: kawaiifi::Scan,
        merged_bss_list: Vec<BssInternal>,
    },
    Err {
        scan_error: kawaiifi::scan::Error,
    },
}

/// How long to keep a BSS after it was last seen (default: 5 minutes)
const BSS_RETENTION_DURATION: Duration = Duration::from_secs(5 * 60);

/// Look up vendor for a BSS via OUI database, bit-flip variants, and VendorSpecific IEs
fn lookup_vendor(bss: &kawaiifi::Bss) -> Option<String> {
    if let Some(vendor) = oui::lookup_vendor(bss.bssid()) {
        return Some(vendor);
    }

    let mut bssid = *bss.bssid();
    let original = bssid[0];
    for mask in [0x02, 0x04, 0x06, 0b1110] {
        bssid[0] = original ^ mask;
        if let Some(vendor) = oui::lookup_vendor(&bssid) {
            return Some(vendor);
        }
    }

    for oui in bss.ies().iter().filter_map(|ie| match &ie.data {
        IeData::VendorSpecific(vendor_specific) => vendor_specific.oui(),
        _ => None,
    }) {
        if let Some(vendor) = oui::lookup_vendor(oui) {
            // Ignore OUIs of common VendorSpecific IEs that don't actually indicate
            // the BSS's vendor
            if !vendor.to_lowercase().contains("microsoft")
                && !vendor.to_lowercase().contains("epigram")
                && !vendor.to_lowercase().contains("broadcom")
                && !vendor.to_lowercase().contains("wi-fi")
                && !vendor.to_lowercase().contains("qualcomm")
            {
                return Some(vendor);
            }
        }
    }

    None
}

pub fn merge_new_scan_results_with_existing(
    existing: &[BssInternal],
    scan_bss_list: &[kawaiifi::Bss],
    mut vendor_cache: MutexGuard<VendorCache>,
) -> Vec<BssInternal> {
    // Build a map of existing BSSs by BSSID for quick lookup
    let mut existing_map: HashMap<[u8; 6], BssInternal> = existing
        .iter()
        .map(|bss_internal| (bss_internal.bssid().clone(), bss_internal.clone()))
        .collect();

    for bss in scan_bss_list {
        if let Some(existing_bss) = existing_map.get_mut(bss.bssid()) {
            existing_bss.update(bss.clone());

            // Update the vendor cache if the existing BSS has a vendor
            if let Some(vendor) = existing_bss.vendor() {
                vendor_cache.insert(&existing_bss.bssid(), vendor.to_string());
                vendor_cache.insert_uptime(existing_bss.uptime(), vendor.to_string());
            }
        } else {
            let mut bss_internal = BssInternal::new(bss.clone());
            if let Some(vendor) = lookup_vendor(bss) {
                bss_internal.set_vendor(vendor.clone());
                vendor_cache.insert(bss_internal.bssid(), vendor.clone());
                vendor_cache.insert_uptime(bss_internal.uptime(), vendor);
            }
            existing_map.insert(bss.bssid().clone(), bss_internal);
        }
    }

    let mut bss_list: Vec<BssInternal> = existing_map.iter().map(|(_, bss)| bss.clone()).collect();

    fill_vendors_from_cache(&mut bss_list, &mut vendor_cache);

    bss_list
}

/// Uses the vendor cache to fill in vendor names for any BSSs that don't have one.
/// Runs multiple passes since filling in one BSS may populate the cache for others
/// with the same uptime.
fn fill_vendors_from_cache(bss_list: &mut [BssInternal], vendor_cache: &mut MutexGuard<VendorCache>) {
    for _ in 0..3 {
        for bss in bss_list.iter_mut() {
            if bss.vendor().is_some() {
                continue;
            }

            if let Some(vendor) = vendor_cache.get(&bss.bssid(), bss.uptime()) {
                bss.set_vendor(vendor.clone());
                vendor_cache.insert(&bss.bssid(), vendor.clone());
                vendor_cache.insert_uptime(bss.uptime(), vendor);
            }
        }
    }
}

impl KawaiiFiWindow {
    pub fn show_cached_scan_results(&self) {
        let Some(interface) = self.imp().interface_box.selected_interface() else {
            return;
        };

        let Ok(scan_results) = interface.cached_scan_results_blocking() else {
            return;
        };

        tracing::info!(
            bss_count = scan_results.len(),
            "Received cached scan results"
        );

        let existing_bss_data: Vec<BssInternal> = self
            .bss_list_store()
            .iter::<BssObject>()
            .filter_map(|obj| obj.ok())
            .map(|obj: BssObject| obj.bss().clone())
            .collect();
        let merged_bss_list = merge_new_scan_results_with_existing(
            &existing_bss_data,
            &scan_results,
            self.imp().vendor_cache.get().unwrap().lock().unwrap(),
        );

        self.apply_merged_results(merged_bss_list);
    }

    pub fn start_scanning(&self, interval_seconds: u32) {
        // Stop any existing timer
        self.stop_scanning();

        let imp = self.imp();
        imp.scan_interval_seconds.set(interval_seconds);
        imp.scanning_enabled.set(true);

        // Do first scan immediately
        self.scan();

        imp.start_scanning_button.set_sensitive(false);
        imp.stop_scanning_button.set_sensitive(true);
    }

    /// Schedule the next scan after the configured interval
    /// This is called after a scan completes or fails
    fn schedule_next_scan(&self) {
        let imp = self.imp();

        // Only schedule if scanning is still enabled
        if !imp.scanning_enabled.get() {
            return;
        }

        let interval_seconds = imp.scan_interval_seconds.get();
        tracing::debug!(interval_seconds, "Scheduling next scan");

        let source_id = glib::timeout_add_local_once(
            std::time::Duration::from_secs(u64::from(interval_seconds)),
            glib::clone!(
                #[weak(rename_to = window)]
                self,
                move || {
                    window.imp().scan_source_id.replace(None);

                    if window.imp().scanning_enabled.get() {
                        window.scan();
                    }
                }
            ),
        );
        imp.scan_source_id.replace(Some(source_id));
    }

    pub fn stop_scanning(&self) {
        let imp = self.imp();
        imp.scanning_enabled.set(false);

        // Cancel the timer
        if let Some(source_id) = imp.scan_source_id.take() {
            source_id.remove();
        }

        imp.start_scanning_button.set_sensitive(true);
        imp.stop_scanning_button.set_sensitive(false);
    }

    pub fn set_scan_interval(&self, interval_seconds: u32) {
        let imp = self.imp();
        imp.scan_interval_seconds.set(interval_seconds);

        // Restart with new interval if currently scanning
        if imp.scanning_enabled.get() {
            self.start_scanning(interval_seconds);
        }
    }

    fn scan(&self) {
        self.imp().file_label.set_visible(false);
        self.imp().interface_box.set_visible(true);
        let Some(interface) = self.imp().interface_box.selected_interface() else {
            return;
        };

        self.emit_by_name::<()>(imp::SIGNAL_SCAN_STARTED, &[]);

        let (sender, receiver) = async_channel::unbounded();

        let existing_bss_data: Vec<BssInternal> = self
            .bss_list_store()
            .iter::<BssObject>()
            .filter_map(|obj| obj.ok())
            .map(|obj: BssObject| obj.bss().clone())
            .collect();
        let vendor_cache = Arc::clone(self.imp().vendor_cache.get().unwrap());
        thread::spawn(move || {
            let result = interface.scan_blocking(kawaiifi::scan::Backend::NetworkManager);
            let mut vendor_cache = vendor_cache.lock().unwrap();
            vendor_cache.clear_uptime_map();
            match result {
                Ok(scan) => {
                    let merged_bss_list: Vec<BssInternal> = merge_new_scan_results_with_existing(
                        &existing_bss_data,
                        scan.bss_list(),
                        vendor_cache,
                    )
                    .iter()
                    .cloned()
                    .filter(|bss| {
                        let Some(age) = bss.time_since_last_seen() else {
                            return true;
                        };

                        age <= BSS_RETENTION_DURATION
                    })
                    .collect();
                    _ = sender.send_blocking(ScanAndProcessingResult::Ok {
                        scan,
                        merged_bss_list,
                    });
                }
                Err(e) => _ = sender.send_blocking(ScanAndProcessingResult::Err { scan_error: e }),
            }
        });

        let window = self.clone();
        glib::spawn_future_local(async move {
            if let Ok(scan_and_processing_result) = receiver.recv().await {
                window.handle_scan_result(scan_and_processing_result);
            }
        });
    }

    fn handle_scan_result(&self, result: ScanAndProcessingResult) {
        match result {
            ScanAndProcessingResult::Ok {
                scan,
                merged_bss_list,
            } if self.imp().scanning_enabled.get() => {
                self.imp().scan_info_popover.set_scan_info(&scan);
                tracing::info!(bss_count = scan.bss_list().len(), "Received scan results");

                let selected_bssid = self
                    .imp()
                    .bss_table
                    .selected_bss()
                    .map(|bss| bss.bssid_bytes());
                self.apply_merged_results(merged_bss_list);
                if let Some(bssid) = &selected_bssid {
                    self.imp().bss_table.set_selected_by_bssid(bssid);
                }

                self.emit_by_name::<()>(imp::SIGNAL_SCAN_COMPLETED, &[]);

                // Schedule the next scan after this one completed
                self.schedule_next_scan();
            }
            ScanAndProcessingResult::Ok {
                scan,
                merged_bss_list: _,
            } => {
                // Scanning was paused while this scan was in flight. Discard the results
                // rather than updating the UI unexpectedly after the user paused.
                tracing::info!(bss_count = scan.bss_list().len(), "Received scan results");

                self.emit_by_name::<()>(imp::SIGNAL_SCAN_COMPLETED, &[]);
            }
            ScanAndProcessingResult::Err { scan_error } => {
                tracing::error!(error = %scan_error, "Scan failed");
                self.emit_by_name::<()>(imp::SIGNAL_SCAN_FAILED, &[&scan_error.to_string()]);

                // Schedule the next scan even if this one failed
                self.schedule_next_scan();
            }
        }
    }

    pub fn apply_loaded_scan(&self, scan_file: ScanFile, path: &PathBuf) {
        self.imp().file_label.set_label(&path.to_string_lossy());
        self.imp().file_label.set_visible(true);
        self.imp().interface_box.set_visible(false);
        self.imp().scan_info_popover.clear();

        let mut vendor_cache = self.imp().vendor_cache.get().unwrap().lock().unwrap();
        let mut bss_list: Vec<BssInternal> = scan_file
            .bss_list()
            .iter()
            .map(|bss| {
                let mut bss_internal = BssInternal::new(bss.clone());
                if let Some(vendor) = lookup_vendor(bss) {
                    bss_internal.set_vendor(vendor.clone());
                    vendor_cache.insert(bss_internal.bssid(), vendor.clone());
                    vendor_cache.insert_uptime(bss_internal.uptime(), vendor);
                }
                bss_internal
            })
            .collect();

        // Use the cache to resolve vendors for any remaining BSSs
        fill_vendors_from_cache(&mut bss_list, &mut vendor_cache);
        drop(vendor_cache);

        self.apply_merged_results(bss_list);
    }

    fn apply_merged_results(&self, merged_bss_list: Vec<BssInternal>) {
        let bss_objects: Vec<BssObject> = merged_bss_list
            .into_iter()
            .map(|bss_internal| BssObject::new(bss_internal))
            .collect();

        let list_store = self.bss_list_store();
        list_store.splice(0, list_store.n_items(), &bss_objects);
    }
}
