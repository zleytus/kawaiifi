use std::sync::OnceLock;

use adw::EntryRow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use enumflags2::BitFlags;
use gtk::glib;
use kawaiifi::{
    Band, ChannelWidth, ChannelWidths, SecurityProtocol, SecurityProtocols, WifiAmendment,
    WifiAmendments, WifiProtocol, WifiProtocols,
};

use crate::objects::BssObject;

pub(crate) const CHANNEL_WIDTH_FILTER_OPTIONS: [ChannelWidth; 6] = [
    ChannelWidth::TwentyMhz,
    ChannelWidth::FortyMhz,
    ChannelWidth::EightyMhz,
    ChannelWidth::EightyPlusEightyMhz,
    ChannelWidth::OneSixtyMhz,
    ChannelWidth::ThreeHundredTwentyMhz,
];

mod imp {
    use std::cell::Cell;

    use gtk::{Button, CheckButton};

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/bss_filter.ui")]
    pub struct BssFilter {
        #[template_child]
        pub ssid_entry: TemplateChild<EntryRow>,
        #[template_child]
        pub bssid_entry: TemplateChild<EntryRow>,
        #[template_child]
        pub vendor_entry: TemplateChild<EntryRow>,

        // Band
        #[template_child]
        pub band_2_4_check: TemplateChild<CheckButton>,
        #[template_child]
        pub band_5_check: TemplateChild<CheckButton>,
        #[template_child]
        pub band_6_check: TemplateChild<CheckButton>,

        // Security
        #[template_child]
        pub security_open_check: TemplateChild<CheckButton>,
        #[template_child]
        pub security_wep_check: TemplateChild<CheckButton>,
        #[template_child]
        pub security_wpa_check: TemplateChild<CheckButton>,
        #[template_child]
        pub security_wpa2_check: TemplateChild<CheckButton>,
        #[template_child]
        pub security_wpa3_check: TemplateChild<CheckButton>,

        // Channel Width
        #[template_child]
        pub width_20_check: TemplateChild<CheckButton>,
        #[template_child]
        pub width_40_check: TemplateChild<CheckButton>,
        #[template_child]
        pub width_80_check: TemplateChild<CheckButton>,
        #[template_child]
        pub width_80_80_check: TemplateChild<CheckButton>,
        #[template_child]
        pub width_160_check: TemplateChild<CheckButton>,
        #[template_child]
        pub width_320_check: TemplateChild<CheckButton>,

        // Protocols
        #[template_child]
        pub protocol_b_check: TemplateChild<CheckButton>,
        #[template_child]
        pub protocol_a_check: TemplateChild<CheckButton>,
        #[template_child]
        pub protocol_g_check: TemplateChild<CheckButton>,
        #[template_child]
        pub protocol_n_check: TemplateChild<CheckButton>,
        #[template_child]
        pub protocol_ac_check: TemplateChild<CheckButton>,
        #[template_child]
        pub protocol_ax_check: TemplateChild<CheckButton>,
        #[template_child]
        pub protocol_be_check: TemplateChild<CheckButton>,

        // Amendments
        #[template_child]
        pub amendment_d_check: TemplateChild<CheckButton>,
        #[template_child]
        pub amendment_e_check: TemplateChild<CheckButton>,
        #[template_child]
        pub amendment_h_check: TemplateChild<CheckButton>,
        #[template_child]
        pub amendment_i_check: TemplateChild<CheckButton>,
        #[template_child]
        pub amendment_k_check: TemplateChild<CheckButton>,
        #[template_child]
        pub amendment_r_check: TemplateChild<CheckButton>,
        #[template_child]
        pub amendment_s_check: TemplateChild<CheckButton>,
        #[template_child]
        pub amendment_v_check: TemplateChild<CheckButton>,
        #[template_child]
        pub amendment_w_check: TemplateChild<CheckButton>,

        #[template_child]
        pub reset_filter_button: TemplateChild<Button>,

        pub suppress_filter_changed: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BssFilter {
        const NAME: &'static str = "BssFilter";
        type Type = super::BssFilter;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BssFilter {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            self.reset_filter_button.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.imp().reset()
            ));

            for entry in self.text_entries() {
                entry.connect_text_notify(glib::clone!(
                    #[weak]
                    obj,
                    move |_| obj.imp().emit_filter_changed()
                ));
            }

            for check in self.check_buttons() {
                check.connect_active_notify(glib::clone!(
                    #[weak(rename_to = filter)]
                    obj,
                    move |_| filter.imp().emit_filter_changed()
                ));
            }
        }

        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![glib::subclass::Signal::builder("filter-changed").build()])
        }
    }

    impl WidgetImpl for BssFilter {}
    impl BoxImpl for BssFilter {}

    impl BssFilter {
        fn text_entries(&self) -> [&EntryRow; 3] {
            [&self.ssid_entry, &self.bssid_entry, &self.vendor_entry]
        }

        fn check_buttons(&self) -> [&CheckButton; 30] {
            [
                &self.band_2_4_check,
                &self.band_5_check,
                &self.band_6_check,
                &self.security_open_check,
                &self.security_wep_check,
                &self.security_wpa_check,
                &self.security_wpa2_check,
                &self.security_wpa3_check,
                &self.width_20_check,
                &self.width_40_check,
                &self.width_80_check,
                &self.width_80_80_check,
                &self.width_160_check,
                &self.width_320_check,
                &self.protocol_b_check,
                &self.protocol_a_check,
                &self.protocol_g_check,
                &self.protocol_n_check,
                &self.protocol_ac_check,
                &self.protocol_ax_check,
                &self.protocol_be_check,
                &self.amendment_d_check,
                &self.amendment_e_check,
                &self.amendment_h_check,
                &self.amendment_i_check,
                &self.amendment_k_check,
                &self.amendment_r_check,
                &self.amendment_s_check,
                &self.amendment_v_check,
                &self.amendment_w_check,
            ]
        }

        fn emit_filter_changed(&self) {
            if !self.suppress_filter_changed.get() {
                self.obj().emit_by_name::<()>("filter-changed", &[]);
            }
        }

        pub fn reset(&self) {
            self.suppress_filter_changed.set(true);
            for check in self.check_buttons() {
                if !check.is_active() {
                    check.set_active(true);
                }
            }
            for entry in self.text_entries() {
                if !entry.text().is_empty() {
                    entry.set_text("");
                }
            }
            self.suppress_filter_changed.set(false);
            self.emit_filter_changed();
        }
    }
}

glib::wrapper! {
    pub struct BssFilter(ObjectSubclass<imp::BssFilter>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl BssFilter {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub(crate) fn state(&self, show_hidden: bool) -> BssFilterState {
        let band_state = self.band_state();
        let (show_open, security_state) = self.security_state();
        let width_state = self.width_state();
        let protocol_state = self.protocol_state();
        let amendment_state = self.amendment_state();

        BssFilterState {
            show_hidden,
            band_state,
            show_open,
            band_all: band_state.iter().all(|&b| b),
            security_all: show_open && security_state.is_all(),
            width_all: all_channel_widths_selected(&width_state),
            protocol_all: protocol_state.is_all(),
            amendment_all: amendment_state.is_all(),
            security_state,
            width_state,
            protocol_state,
            amendment_state,
            ssid_query: self.ssid_query(),
            bssid_query: self.bssid_query(),
            vendor_query: self.vendor_query(),
        }
    }

    pub fn band_state(&self) -> [bool; 3] {
        let imp = self.imp();
        [
            imp.band_2_4_check.is_active(),
            imp.band_5_check.is_active(),
            imp.band_6_check.is_active(),
        ]
    }

    /// Returns `(show_open, protocols)`. `show_open` covers networks with no security;
    /// `protocols` covers WEP/WPA/WPA2/WPA3 since `SecurityProtocol` has no Open variant.
    pub fn security_state(&self) -> (bool, SecurityProtocols) {
        let imp = self.imp();
        let show_open = imp.security_open_check.is_active();
        let mut flags: BitFlags<SecurityProtocol> = BitFlags::empty();
        if imp.security_wep_check.is_active() {
            flags |= SecurityProtocol::WEP;
        }
        if imp.security_wpa_check.is_active() {
            flags |= SecurityProtocol::WPA;
        }
        if imp.security_wpa2_check.is_active() {
            flags |= SecurityProtocol::WPA2;
        }
        if imp.security_wpa3_check.is_active() {
            flags |= SecurityProtocol::WPA3;
        }
        (show_open, SecurityProtocols::from(flags))
    }

    pub fn width_state(&self) -> ChannelWidths {
        let imp = self.imp();
        let mut widths = std::collections::HashSet::new();

        for (check, width) in [
            (&imp.width_20_check, ChannelWidth::TwentyMhz),
            (&imp.width_40_check, ChannelWidth::FortyMhz),
            (&imp.width_80_check, ChannelWidth::EightyMhz),
            (&imp.width_80_80_check, ChannelWidth::EightyPlusEightyMhz),
            (&imp.width_160_check, ChannelWidth::OneSixtyMhz),
            (&imp.width_320_check, ChannelWidth::ThreeHundredTwentyMhz),
        ] {
            if check.is_active() {
                widths.insert(width);
            }
        }

        ChannelWidths::from(widths)
    }

    pub fn protocol_state(&self) -> WifiProtocols {
        let imp = self.imp();
        let mut flags: BitFlags<WifiProtocol> = BitFlags::empty();

        for (check, protocol) in [
            (&imp.protocol_b_check, WifiProtocol::B),
            (&imp.protocol_a_check, WifiProtocol::A),
            (&imp.protocol_g_check, WifiProtocol::G),
            (&imp.protocol_n_check, WifiProtocol::N),
            (&imp.protocol_ac_check, WifiProtocol::AC),
            (&imp.protocol_ax_check, WifiProtocol::AX),
            (&imp.protocol_be_check, WifiProtocol::BE),
        ] {
            if check.is_active() {
                flags |= protocol;
            }
        }

        WifiProtocols::from(flags)
    }

    pub fn amendment_state(&self) -> WifiAmendments {
        let imp = self.imp();
        let mut flags: BitFlags<WifiAmendment> = BitFlags::empty();

        for (check, amendment) in [
            (&imp.amendment_d_check, WifiAmendment::D),
            (&imp.amendment_e_check, WifiAmendment::E),
            (&imp.amendment_h_check, WifiAmendment::H),
            (&imp.amendment_i_check, WifiAmendment::I),
            (&imp.amendment_k_check, WifiAmendment::K),
            (&imp.amendment_r_check, WifiAmendment::R),
            (&imp.amendment_s_check, WifiAmendment::S),
            (&imp.amendment_v_check, WifiAmendment::V),
            (&imp.amendment_w_check, WifiAmendment::W),
        ] {
            if check.is_active() {
                flags |= amendment;
            }
        }

        WifiAmendments::from(flags)
    }

    pub fn ssid_query(&self) -> String {
        self.imp().ssid_entry.text().to_lowercase()
    }

    pub fn bssid_query(&self) -> String {
        self.imp().bssid_entry.text().to_lowercase()
    }

    pub fn vendor_query(&self) -> String {
        self.imp().vendor_entry.text().to_lowercase()
    }

    pub fn connect_filter_changed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("filter-changed", false, move |args| {
            let obj = args[0].get::<Self>().unwrap();
            f(&obj);
            None
        })
    }
}

pub(crate) struct BssFilterState {
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
    pub(crate) fn matches(&self, bss: &BssObject) -> bool {
        if !self.show_hidden && bss.data().formatted_ssid().is_none() {
            return false;
        }

        if !self.ssid_query.is_empty() {
            // Hidden BSSs are searchable by the visible placeholder text used in the table.
            let ssid_match = match bss.data().formatted_ssid() {
                Some(ssid) => ssid.to_lowercase().contains(&self.ssid_query),
                None => "hidden".contains(&self.ssid_query),
            };
            if !ssid_match {
                return false;
            }
        }

        if !self.bssid_query.is_empty()
            && !bss
                .data()
                .formatted_bssid()
                .to_lowercase()
                .contains(&self.bssid_query)
        {
            return false;
        }

        if !self.vendor_query.is_empty()
            && !bss
                .data()
                .formatted_vendor()
                .to_lowercase()
                .contains(&self.vendor_query)
        {
            return false;
        }

        if !self.band_matches(bss) {
            return false;
        }

        if !self.security_matches(bss) {
            return false;
        }

        if !self.width_all && !self.width_state.contains(&bss.data().channel_width()) {
            return false;
        }

        if !self.protocol_all && (*bss.data().wifi_protocols() & *self.protocol_state).is_empty() {
            return false;
        }

        if !self.amendment_all && (*bss.data().wifi_amendments() & *self.amendment_state).is_empty()
        {
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
            .any(|(i, b)| self.band_state[i] && *b == bss.data().band())
    }

    fn security_matches(&self, bss: &BssObject) -> bool {
        if self.security_all {
            return true;
        }

        let security = bss.data().security_protocols();
        // Open networks have no security flags, so they are controlled by the separate
        // "open" checkbox rather than by SecurityProtocols.
        (self.show_open && security.is_empty())
            || (!security.is_empty() && !(*security & *self.security_state).is_empty())
    }
}

fn all_channel_widths_selected(widths: &ChannelWidths) -> bool {
    CHANNEL_WIDTH_FILTER_OPTIONS
        .iter()
        .all(|width| widths.contains(width))
}

impl Default for BssFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn selected_widths(widths: &[ChannelWidth]) -> ChannelWidths {
        ChannelWidths::from(widths.iter().copied().collect::<HashSet<_>>())
    }

    #[test]
    fn all_channel_widths_selected_uses_filter_option_list() {
        assert!(all_channel_widths_selected(&selected_widths(
            &CHANNEL_WIDTH_FILTER_OPTIONS
        )));
    }

    #[test]
    fn all_channel_widths_selected_rejects_partial_selection() {
        assert!(!all_channel_widths_selected(&selected_widths(&[
            ChannelWidth::TwentyMhz,
            ChannelWidth::FortyMhz,
        ])));
    }
}
