use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::gio::{self, prelude::SettingsExtManual};

use crate::config;

use super::{BssTable, column_settings, columns::*};

impl BssTable {
    pub(super) fn setup_columns(&self) {
        let imp = self.imp();

        imp.color_column.set_factory(Some(&create_color_factory()));
        imp.ssid_column.set_factory(Some(&create_ssid_factory()));
        imp.bssid_column.set_factory(Some(&create_bssid_factory()));
        imp.vendor_column
            .set_factory(Some(&create_vendor_factory()));
        imp.signal_column
            .set_factory(Some(&create_signal_factory()));
        imp.channel_column
            .set_factory(Some(&create_channel_factory()));
        imp.channel_width_column
            .set_factory(Some(&create_channel_width_factory()));
        imp.frequency_column
            .set_factory(Some(&create_frequency_factory()));
        imp.band_column.set_factory(Some(&create_band_factory()));
        imp.protocols_column
            .set_factory(Some(&create_protocols_factory()));
        imp.amendments_column
            .set_factory(Some(&create_amendments_factory()));
        imp.security_column
            .set_factory(Some(&create_security_factory()));
        imp.channel_utilization_column
            .set_factory(Some(&create_channel_utilization_factory()));
        imp.station_count_column
            .set_factory(Some(&create_station_count_factory()));
        imp.max_rate_column
            .set_factory(Some(&create_max_rate_factory()));
        imp.uptime_column
            .set_factory(Some(&create_uptime_factory()));

        // Last Seen tracks bound labels so the refresh timer can update visible rows.
        imp.last_seen_column
            .set_factory(Some(&create_last_seen_factory(
                imp.bound_last_seen_labels.clone(),
            )));
    }

    pub(super) fn setup_column_sorters(&self) {
        let imp = self.imp();

        imp.ssid_column.set_sorter(Some(&create_ssid_sorter()));
        imp.bssid_column.set_sorter(Some(&create_bssid_sorter()));
        imp.vendor_column.set_sorter(Some(&create_vendor_sorter()));
        imp.signal_column.set_sorter(Some(&create_signal_sorter()));
        imp.channel_column
            .set_sorter(Some(&create_channel_sorter()));
        imp.frequency_column
            .set_sorter(Some(&create_frequency_sorter()));
        imp.band_column.set_sorter(Some(&create_band_sorter()));
        imp.channel_width_column
            .set_sorter(Some(&create_channel_width_sorter()));
        imp.protocols_column
            .set_sorter(Some(&create_protocols_sorter()));
        imp.amendments_column
            .set_sorter(Some(&create_amendments_sorter()));
        imp.security_column
            .set_sorter(Some(&create_security_sorter()));
        imp.channel_utilization_column
            .set_sorter(Some(&create_channel_utilization_sorter()));
        imp.station_count_column
            .set_sorter(Some(&create_station_count_sorter()));
        imp.max_rate_column
            .set_sorter(Some(&create_max_rate_sorter()));
        imp.uptime_column.set_sorter(Some(&create_uptime_sorter()));
        imp.last_seen_column
            .set_sorter(Some(&create_last_seen_sorter()));

        imp.column_view
            .sort_by_column(Some(&imp.signal_column), gtk::SortType::Descending);
    }

    pub(super) fn setup_column_visibility(&self) {
        let imp = self.imp();
        let settings = gio::Settings::new(config::app_id());

        settings
            .bind(column_settings::SHOW_BSSID, &*imp.bssid_column, "visible")
            .build();
        settings
            .bind(column_settings::SHOW_VENDOR, &*imp.vendor_column, "visible")
            .build();
        settings
            .bind(column_settings::SHOW_SIGNAL, &*imp.signal_column, "visible")
            .build();
        settings
            .bind(
                column_settings::SHOW_CHANNEL,
                &*imp.channel_column,
                "visible",
            )
            .build();
        settings
            .bind(
                column_settings::SHOW_CHANNEL_WIDTH,
                &*imp.channel_width_column,
                "visible",
            )
            .build();
        settings
            .bind(
                column_settings::SHOW_FREQUENCY,
                &*imp.frequency_column,
                "visible",
            )
            .build();
        settings
            .bind(column_settings::SHOW_BAND, &*imp.band_column, "visible")
            .build();
        settings
            .bind(
                column_settings::SHOW_PROTOCOLS,
                &*imp.protocols_column,
                "visible",
            )
            .build();
        settings
            .bind(
                column_settings::SHOW_AMENDMENTS,
                &*imp.amendments_column,
                "visible",
            )
            .build();
        settings
            .bind(
                column_settings::SHOW_SECURITY,
                &*imp.security_column,
                "visible",
            )
            .build();
        settings
            .bind(
                column_settings::SHOW_MAX_RATE,
                &*imp.max_rate_column,
                "visible",
            )
            .build();
        settings
            .bind(
                column_settings::SHOW_CHANNEL_UTILIZATION,
                &*imp.channel_utilization_column,
                "visible",
            )
            .build();
        settings
            .bind(
                column_settings::SHOW_STATIONS,
                &*imp.station_count_column,
                "visible",
            )
            .build();
        settings
            .bind(column_settings::SHOW_UPTIME, &*imp.uptime_column, "visible")
            .build();
        settings
            .bind(
                column_settings::SHOW_LAST_SEEN,
                &*imp.last_seen_column,
                "visible",
            )
            .build();
    }
}
