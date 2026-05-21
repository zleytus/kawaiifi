use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::gio::prelude::SettingsExt;
use kawaiifi::{Band, ChannelWidths, SecurityProtocols, WifiAmendments, WifiProtocols};

use crate::objects::BssObject;

use super::KawaiiFiWindow;

pub(super) struct BssFilterState {
    show_hidden: bool,
    band_state: [bool; 3],
    show_open: bool,
    security_state: SecurityProtocols,
    width_state: ChannelWidths,
    protocol_state: WifiProtocols,
    amendment_state: WifiAmendments,
    ssid_query: String,
    bssid_query: String,
    vendor_query: String,
    band_all: bool,
    security_all: bool,
    width_all: bool,
    protocol_all: bool,
    amendment_all: bool,
}

impl BssFilterState {
    pub(super) fn from_window(window: &KawaiiFiWindow) -> Self {
        let imp = window.imp();
        let show_hidden = window.settings().boolean("show-hidden-bsss");
        let band_state = imp.bss_filter.band_state();
        let (show_open, security_state) = imp.bss_filter.security_state();
        let width_state = imp.bss_filter.width_state();
        let protocol_state = imp.bss_filter.protocol_state();
        let amendment_state = imp.bss_filter.amendment_state();
        let ssid_query = imp.bss_filter.ssid_query();
        let bssid_query = imp.bss_filter.bssid_query();
        let vendor_query = imp.bss_filter.vendor_query();

        Self {
            show_hidden,
            band_state,
            show_open,
            band_all: band_state.iter().all(|&b| b),
            security_all: show_open && security_state.is_all(),
            width_all: width_state.len() == 6,
            protocol_all: protocol_state.is_all(),
            amendment_all: amendment_state.is_all(),
            security_state,
            width_state,
            protocol_state,
            amendment_state,
            ssid_query,
            bssid_query,
            vendor_query,
        }
    }

    pub(super) fn matches(&self, bss: &BssObject) -> bool {
        if !self.show_hidden && bss.ssid().is_none() {
            return false;
        }

        if !self.ssid_query.is_empty() {
            // Hidden BSSs are searchable by the visible placeholder text used in the table.
            let ssid_match = match bss.ssid() {
                Some(ssid) => ssid.to_lowercase().contains(&self.ssid_query),
                None => "hidden".contains(&self.ssid_query),
            };
            if !ssid_match {
                return false;
            }
        }

        if !self.bssid_query.is_empty() && !bss.bssid().to_lowercase().contains(&self.bssid_query) {
            return false;
        }

        if !self.vendor_query.is_empty()
            && !bss.vendor().to_lowercase().contains(&self.vendor_query)
        {
            return false;
        }

        if !self.band_matches(bss) {
            return false;
        }

        if !self.security_matches(bss) {
            return false;
        }

        if !self.width_all && !self.width_state.contains(&bss.channel_width()) {
            return false;
        }

        if !self.protocol_all && (*bss.protocols() & *self.protocol_state).is_empty() {
            return false;
        }

        if !self.amendment_all && (*bss.amendments() & *self.amendment_state).is_empty() {
            return false;
        }

        true
    }

    fn band_matches(&self, bss: &BssObject) -> bool {
        if self.band_all {
            return true;
        }

        let allowed = [Band::TwoPointFourGhz, Band::FiveGhz, Band::SixGhz];
        allowed
            .iter()
            .enumerate()
            .any(|(i, b)| self.band_state[i] && *b == bss.band())
    }

    fn security_matches(&self, bss: &BssObject) -> bool {
        if self.security_all {
            return true;
        }

        let security = bss.security();
        // Open networks have no security flags, so they are controlled by the separate
        // "open" checkbox rather than by SecurityProtocols.
        (self.show_open && security.is_empty())
            || (!security.is_empty() && !(*security & *self.security_state).is_empty())
    }
}
