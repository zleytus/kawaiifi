use std::time::Duration;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    gio::prelude::SettingsExt,
    glib::{self},
};
use kawaiifi::{Bss, Interface, ScanError};

use super::KawaiiFiWindow;
use crate::{
    objects::BssInternal,
    scan::{ProcessedScan, spawn_scan_processing},
};

/// Interval between automatic Wi-Fi scans, in seconds.
const SCAN_INTERVAL_SECONDS: u64 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScanKind {
    Cached,
    Active,
}

impl ScanKind {
    fn fetch(self, interface: &Interface) -> Result<Vec<Bss>, ScanError> {
        match self {
            Self::Cached => interface.cached_scan_results_blocking(),
            Self::Active => interface
                .scan_blocking()
                .map(|scan| scan.bss_list().to_vec()),
        }
    }

    fn next_delay(self) -> Duration {
        match self {
            Self::Cached => Duration::ZERO,
            Self::Active => Duration::from_secs(SCAN_INTERVAL_SECONDS),
        }
    }
}

impl KawaiiFiWindow {
    pub(super) fn start_scanning(&self, interface: Interface) {
        self.cancel_scheduled_scan();
        self.set_scanning_enabled(true);
        self.start_scan(interface, ScanKind::Cached);
    }

    pub(super) fn stop_scanning(&self) {
        self.set_scan_active(false);
        self.set_scanning_enabled(false);
        self.cancel_scheduled_scan();
    }

    /// Schedule the next active scan after the configured delay
    fn schedule_active_scan(&self, interface: Interface, delay: Duration) {
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
                        window.start_scan(interface, ScanKind::Active);
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

    fn start_scan(&self, interface: Interface, scan_kind: ScanKind) {
        if !self.imp().scanning_enabled.get() {
            return;
        }

        if !interface_is_available(&interface) {
            self.on_scan_failed(&format!("{} is no longer available", interface.name()));
            return;
        }

        let generation = self.scan_generation();
        let existing_bss_data = self.current_bss_data();
        let vendor_cache = self.vendor_cache_snapshot();
        let next_scan_interface = interface.clone();

        if scan_kind == ScanKind::Active {
            self.on_scan_started();
        }

        glib::spawn_future_local(glib::clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                let interface_name = interface.name().to_string();
                let fetch_from_scan = move || scan_kind.fetch(&interface);
                let result =
                    spawn_scan_processing(fetch_from_scan, vendor_cache, existing_bss_data).await;

                // The user may switch interfaces while the blocking scan is still running.
                if !window.generation_is_current(generation) {
                    tracing::debug!(
                        ?scan_kind,
                        "Discarding scan result for a previous interface"
                    );
                    return;
                }

                match result {
                    Ok(Ok(processed)) => {
                        tracing::info!(
                            ?scan_kind,
                            bss_count = processed.bss_list.len(),
                            interface_name,
                            "Received scan results"
                        );
                        if window.imp().scanning_enabled.get() {
                            match scan_kind {
                                ScanKind::Active => window.apply_active_scan_result(processed),
                                ScanKind::Cached => window.apply_cached_scan_result(processed),
                            }
                        }
                        window.on_scan_completed();
                    }
                    Ok(Err(scan_error)) => {
                        tracing::error!(?scan_kind, error = %scan_error, "Scan failed");
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

                window.schedule_active_scan(next_scan_interface, scan_kind.next_delay());
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

    fn apply_cached_scan_result(&self, processed: ProcessedScan) {
        self.install_vendor_cache(processed.vendor_cache);
        self.apply_merged_results(processed.bss_list);
    }

    fn on_scan_started(&self) {
        self.set_scan_active(true);
    }

    fn on_scan_completed(&self) {
        self.set_scan_active(false);
        self.imp().status_banner.set_revealed(false);
    }

    fn on_scan_failed(&self, error: &str) {
        self.stop_scanning();
        self.imp().status_banner.set_revealed(true);
        self.imp()
            .status_banner
            .set_title(&format!("Scan Failed: {}", error));
    }
}

fn interface_is_available(interface: &Interface) -> bool {
    kawaiifi::interfaces().is_ok_and(|interfaces| {
        interfaces.iter().any(|current| {
            current.index() == interface.index() && current.mac_address() == interface.mac_address()
        })
    })
}
