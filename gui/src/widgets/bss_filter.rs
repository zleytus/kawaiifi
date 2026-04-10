use std::sync::OnceLock;

use adw::EntryRow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use enumflags2::BitFlags;
use gtk::glib;
use kawaiifi::{
    ChannelWidth, ChannelWidths, SecurityProtocol, SecurityProtocols, WifiAmendment,
    WifiAmendments, WifiProtocol, WifiProtocols,
};

mod imp {
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
                    move |_| obj.emit_by_name::<()>("filter-changed", &[])
                ));
            }

            for check in self.check_buttons() {
                check.connect_active_notify(glib::clone!(
                    #[weak(rename_to = filter)]
                    obj,
                    move |_| {
                        filter.emit_by_name::<()>("filter-changed", &[]);
                    }
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

        pub fn reset(&self) {
            for check in self.check_buttons() {
                check.set_active(true);
            }
            for entry in self.text_entries() {
                entry.set_text("");
            }
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
        if imp.width_20_check.is_active() {
            widths.insert(ChannelWidth::TwentyMhz);
        }
        if imp.width_40_check.is_active() {
            widths.insert(ChannelWidth::FortyMhz);
        }
        if imp.width_80_check.is_active() {
            widths.insert(ChannelWidth::EightyMhz);
        }
        if imp.width_80_80_check.is_active() {
            widths.insert(ChannelWidth::EightyPlusEightyMhz);
        }
        if imp.width_160_check.is_active() {
            widths.insert(ChannelWidth::OneSixtyMhz);
        }
        if imp.width_320_check.is_active() {
            widths.insert(ChannelWidth::ThreeHundredTwentyMhz);
        }
        ChannelWidths::from(widths)
    }

    pub fn protocol_state(&self) -> WifiProtocols {
        let imp = self.imp();
        let mut flags: BitFlags<WifiProtocol> = BitFlags::empty();
        if imp.protocol_b_check.is_active() {
            flags |= WifiProtocol::B;
        }
        if imp.protocol_a_check.is_active() {
            flags |= WifiProtocol::A;
        }
        if imp.protocol_g_check.is_active() {
            flags |= WifiProtocol::G;
        }
        if imp.protocol_n_check.is_active() {
            flags |= WifiProtocol::N;
        }
        if imp.protocol_ac_check.is_active() {
            flags |= WifiProtocol::AC;
        }
        if imp.protocol_ax_check.is_active() {
            flags |= WifiProtocol::AX;
        }
        if imp.protocol_be_check.is_active() {
            flags |= WifiProtocol::BE;
        }
        WifiProtocols::from(flags)
    }

    pub fn amendment_state(&self) -> WifiAmendments {
        let imp = self.imp();
        let mut flags: BitFlags<WifiAmendment> = BitFlags::empty();
        if imp.amendment_d_check.is_active() {
            flags |= WifiAmendment::D;
        }
        if imp.amendment_e_check.is_active() {
            flags |= WifiAmendment::E;
        }
        if imp.amendment_h_check.is_active() {
            flags |= WifiAmendment::H;
        }
        if imp.amendment_i_check.is_active() {
            flags |= WifiAmendment::I;
        }
        if imp.amendment_k_check.is_active() {
            flags |= WifiAmendment::K;
        }
        if imp.amendment_r_check.is_active() {
            flags |= WifiAmendment::R;
        }
        if imp.amendment_s_check.is_active() {
            flags |= WifiAmendment::S;
        }
        if imp.amendment_v_check.is_active() {
            flags |= WifiAmendment::V;
        }
        if imp.amendment_w_check.is_active() {
            flags |= WifiAmendment::W;
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

impl Default for BssFilter {
    fn default() -> Self {
        Self::new()
    }
}
