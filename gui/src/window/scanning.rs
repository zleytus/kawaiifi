use std::{path::Path, sync::Arc, thread, time::Duration};

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    gio::prelude::{ListModelExt, ListModelExtManual, SettingsExt},
    glib::{self, object::ObjectExt},
    prelude::WidgetExt,
};

use super::{
    KawaiiFiWindow, imp, scan_processing::fill_vendors_from_cache, scan_processing::lookup_vendor,
    scan_processing::merge_new_scan_results_with_existing,
};
use crate::{
    objects::{BssInternal, BssObject},
    scan_file::ScanFile,
};

enum ScanAndProcessingResult {
    Ok {
        scan: kawaiifi::Scan,
        merged_bss_list: Vec<BssInternal>,
    },
    Err {
        scan_error: kawaiifi::ScanError,
    },
}

enum CachedScanResult {
    Ok {
        bss_count: usize,
        merged_bss_list: Vec<BssInternal>,
    },
    Err {
        scan_error: kawaiifi::ScanError,
    },
}

impl KawaiiFiWindow {
    pub fn show_cached_scan_results(&self) {
        let Some(interface) = self.imp().interface_box.selected_interface() else {
            tracing::warn!("Cannot show cached scan results without a selected interface");
            return;
        };

        let existing_bss_data = self.current_bss_data();
        let vendor_cache = Arc::clone(self.imp().vendor_cache.get().unwrap());
        let (sender, receiver) = async_channel::unbounded();

        // Cached scan retrieval is blocking, so keep it off the UI thread.
        thread::spawn(move || {
            let result = interface.cached_scan_results_blocking();
            let mut vendor_cache = vendor_cache.lock().unwrap();
            match result {
                Ok(scan_results) => {
                    // Cached results are from before this app run, so keep the existing
                    // uptime cache rather than treating this as a fresh scan boundary.
                    let bss_count = scan_results.len();
                    let merged_bss_list = merge_new_scan_results_with_existing(
                        &existing_bss_data,
                        &scan_results,
                        &mut vendor_cache,
                    );
                    _ = sender.send_blocking(CachedScanResult::Ok {
                        bss_count,
                        merged_bss_list,
                    });
                }
                Err(scan_error) => {
                    _ = sender.send_blocking(CachedScanResult::Err { scan_error });
                }
            }
        });

        let window = self.clone();
        glib::spawn_future_local(async move {
            // GTK widgets must be updated from the main thread; the worker only sends
            // processed data back through the channel.
            match receiver.recv().await {
                Ok(CachedScanResult::Ok {
                    bss_count,
                    merged_bss_list,
                }) => {
                    tracing::info!(bss_count, "Received cached scan results");
                    window.apply_merged_results(merged_bss_list);
                }
                Ok(CachedScanResult::Err { scan_error }) => {
                    tracing::warn!(error = %scan_error, "Failed to read cached scan results");
                }
                Err(recv_error) => {
                    tracing::warn!(error = %recv_error, "Cached scan worker stopped before sending results");
                }
            }
        });
    }

    pub fn start_scanning(&self) {
        if self.imp().interface_box.selected_interface().is_none() {
            tracing::warn!("Cannot start scanning without a selected interface");
            self.stop_scanning();
            return;
        }

        // Stop any existing timer
        self.stop_scanning();

        let imp = self.imp();
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

        let interval_seconds = super::SCAN_INTERVAL_SECONDS;
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

        if let Some(source_id) = imp.scan_source_id.take() {
            source_id.remove();
        }

        imp.active_scan_spinner.set_visible(false);
        imp.start_scanning_button.set_sensitive(true);
        imp.stop_scanning_button.set_sensitive(false);
    }

    fn scan(&self) {
        self.imp().file_label.set_visible(false);
        self.imp().interface_box.set_visible(true);
        let Some(interface) = self.imp().interface_box.selected_interface() else {
            self.stop_scanning();
            return;
        };
        self.imp().active_scan_spinner.set_visible(true);

        self.emit_by_name::<()>(imp::SIGNAL_SCAN_STARTED, &[]);

        let (sender, receiver) = async_channel::unbounded();

        let existing_bss_data = self.current_bss_data();
        let vendor_cache = Arc::clone(self.imp().vendor_cache.get().unwrap());
        thread::spawn(move || {
            let result = interface.scan_blocking(kawaiifi::Backend::NetworkManager);
            let mut vendor_cache = vendor_cache.lock().unwrap();
            // TSF uptimes change between scans, so uptime-derived vendor matches are
            // only valid within a single fresh scan result set.
            vendor_cache.clear_uptime_map();
            match result {
                Ok(scan) => {
                    let merged_bss_list = merge_new_scan_results_with_existing(
                        &existing_bss_data,
                        scan.bss_list(),
                        &mut vendor_cache,
                    );
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
            let scan_result = receiver.recv().await;
            window.imp().active_scan_spinner.set_visible(false);
            match scan_result {
                Ok(scan_and_processing_result) => {
                    window.handle_scan_result(scan_and_processing_result);
                }
                Err(recv_error) => {
                    let message = "Scan worker stopped before sending results";
                    tracing::error!(error = %recv_error, message);
                    window.emit_by_name::<()>(imp::SIGNAL_SCAN_FAILED, &[&message]);
                    window.schedule_next_scan();
                }
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

                let retention_secs =
                    u64::try_from(self.settings().int("bss-retention-duration")).unwrap_or(300);
                let retention = Duration::from_secs(retention_secs);
                let merged_bss_list: Vec<BssInternal> = merged_bss_list
                    .into_iter()
                    .filter(|bss| {
                        bss.time_since_last_seen()
                            .is_none_or(|age| age <= retention)
                    })
                    .collect();

                // Applying merged results replaces BssObject instances, so preserve
                // the user's selection by the stable BSSID.
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

    pub fn apply_loaded_scan(&self, scan_file: ScanFile, path: &Path) {
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
        let bss_objects: Vec<BssObject> = merged_bss_list.into_iter().map(BssObject::new).collect();

        let list_store = self.bss_list_store();
        list_store.splice(0, list_store.n_items(), &bss_objects);
    }

    fn current_bss_data(&self) -> Vec<BssInternal> {
        self.bss_list_store()
            .iter::<BssObject>()
            .filter_map(|obj| obj.ok())
            .map(|obj: BssObject| obj.bss().clone())
            .collect()
    }
}
