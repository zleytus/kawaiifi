use std::time::Duration;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    gio::prelude::SettingsExt,
    glib::{self},
    prelude::WidgetExt,
};
use kawaiifi::Interface;

use super::KawaiiFiWindow;
use crate::{
    objects::BssInternal,
    scan::{ProcessedScan, spawn_scan_processing},
};

impl KawaiiFiWindow {
    pub(super) fn enable_scanning(&self) {
        let imp = self.imp();
        self.cancel_scheduled_scan();
        imp.scanning_enabled.replace(true);
        self.update_scan_controls();
    }

    pub fn stop_scanning(&self) {
        let imp = self.imp();
        imp.scanning_enabled.replace(false);
        self.imp().active_scan_spinner.set_visible(false);
        self.cancel_scheduled_scan();
        self.update_scan_controls();
    }

    fn update_scan_controls(&self) {
        let scanning_enabled = self.imp().scanning_enabled.get();
        self.imp()
            .start_scanning_button
            .set_sensitive(!scanning_enabled);
        self.imp()
            .stop_scanning_button
            .set_sensitive(scanning_enabled);
    }

    /// Schedule the next scan after the configured delay
    pub(super) fn schedule_active_scan(&self, interface: Interface, delay: Duration) {
        let imp = self.imp();

        // Only schedule if scanning is still enabled
        if !imp.scanning_enabled.get() {
            return;
        }

        tracing::debug!(delay_secs = %delay.as_secs(), "Scheduling next scan");

        let source_id = glib::timeout_add_local_once(
            delay,
            glib::clone!(
                #[weak(rename_to = window)]
                self,
                move || {
                    window.imp().scan_source_id.replace(None);

                    if window.imp().scanning_enabled.get() {
                        window.start_active_scan(interface);
                    }
                }
            ),
        );
        imp.scan_source_id.replace(Some(source_id));
    }

    fn cancel_scheduled_scan(&self) {
        let imp = self.imp();
        if let Some(source_id) = imp.scan_source_id.take() {
            source_id.remove();
        }
    }

    fn start_active_scan(&self, interface: Interface) {
        if !self.imp().scanning_enabled.get() {
            return;
        }

        let generation = self.scan_generation();
        let existing_bss_data = self.current_bss_data();
        let vendor_cache = self.vendor_cache_snapshot();
        let next_scan_interface = interface.clone();

        self.on_scan_started();

        glib::spawn_future_local(glib::clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                let fetch_from_scan = |interface: &Interface| {
                    let scan = interface.scan_blocking();
                    scan.map(|scan| scan.bss_list().to_vec())
                };
                let result = spawn_scan_processing(
                    fetch_from_scan,
                    interface,
                    vendor_cache,
                    existing_bss_data,
                )
                .await;

                // The user may switch interfaces while the blocking scan is still running.
                if !window.generation_is_current(generation) {
                    tracing::debug!("Discarding scan result for a previous interface");
                    return;
                }

                match result {
                    Ok(Ok(processed)) => {
                        if window.imp().scanning_enabled.get() {
                            window.apply_active_scan_result(processed);
                        }
                        window.on_scan_completed();
                    }
                    Ok(Err(scan_error)) => {
                        tracing::error!(error = %scan_error, "Scan failed");
                        window.on_scan_failed(&scan_error.to_string());
                        return;
                    }
                    Err(_) => {
                        let message = "Scan worker panicked";
                        tracing::error!(message);
                        window.on_scan_failed(&message);
                        return;
                    }
                }

                window.schedule_active_scan(
                    next_scan_interface,
                    Duration::from_secs(super::SCAN_INTERVAL_SECONDS),
                );
            }
        ));
    }

    fn apply_active_scan_result(&self, processed: ProcessedScan) {
        self.install_vendor_cache(processed.vendor_cache);

        let retention_secs =
            u64::try_from(self.settings().int("bss-retention-duration")).unwrap_or(300);
        let retention = Duration::from_secs(retention_secs);
        let merged_bss_list: Vec<BssInternal> = processed
            .bss_list
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
            .map(|bss| *bss.data().bssid());
        self.apply_merged_results(merged_bss_list);
        if let Some(bssid) = &selected_bssid {
            self.imp().bss_table.set_selected_by_bssid(bssid);
        }
    }

    pub(super) fn start_cached_scan(&self, interface: Interface) {
        if !self.imp().scanning_enabled.get() {
            return;
        }

        let generation = self.scan_generation();
        let existing_bss_data = self.current_bss_data();
        let vendor_cache = self.vendor_cache_snapshot();
        let next_scan_interface = interface.clone();

        glib::spawn_future_local(glib::clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                let fetch_from_cache =
                    |interface: &Interface| interface.cached_scan_results_blocking();
                let result = spawn_scan_processing(
                    fetch_from_cache,
                    interface,
                    vendor_cache,
                    existing_bss_data,
                )
                .await;

                if !window.generation_is_current(generation) {
                    tracing::debug!("Discarding cached scan results for a previous interface");
                    return;
                }

                match result {
                    Ok(Ok(processed)) => {
                        if window.imp().scanning_enabled.get() {
                            window.apply_cached_scan_result(processed);
                        }
                        window.on_scan_completed();
                    }
                    Ok(Err(scan_error)) => {
                        tracing::warn!(error = %scan_error, "Failed to read cached scan results");
                        window.on_scan_failed(&scan_error.to_string());
                        return;
                    }
                    Err(_) => {
                        let message = "Scan worker panicked";
                        tracing::error!(message);
                        window.on_scan_failed(message);
                        return;
                    }
                }

                window.schedule_active_scan(next_scan_interface, Duration::ZERO);
            }
        ));
    }

    fn apply_cached_scan_result(&self, processed: ProcessedScan) {
        self.install_vendor_cache(processed.vendor_cache);
        self.apply_merged_results(processed.bss_list);
    }

    fn on_scan_started(&self) {
        self.imp().active_scan_spinner.set_visible(true);
    }

    fn on_scan_completed(&self) {
        self.imp().active_scan_spinner.set_visible(false);
        self.imp().scan_failed_banner.set_revealed(false);
    }

    fn on_scan_failed(&self, error: &str) {
        self.stop_scanning();
        self.imp().scan_failed_banner.set_revealed(true);
        self.imp()
            .scan_failed_banner
            .set_title(&format!("Scan Failed: {}", error));
    }
}
