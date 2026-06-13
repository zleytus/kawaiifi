use gtk::{glib, subclass::prelude::*};

mod imp {
    use adw::{
        prelude::ComboRowExt,
        subclass::{dialog::AdwDialogImpl, preferences_dialog::PreferencesDialogImpl},
    };
    use gtk::gio::{
        self,
        prelude::{SettingsExt, SettingsExtManual},
    };

    use super::super::bss_table::column_settings;
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/preferences_dialog.ui")]
    pub struct PreferencesDialog {
        #[template_child]
        pub show_hidden_bsss_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub bss_retention_duration_combo_row: TemplateChild<adw::ComboRow>,
        #[template_child]
        pub show_bssid_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_vendor_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_signal_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_channel_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_channel_width_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_frequency_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_band_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_protocols_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_amendments_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_security_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_max_rate_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_channel_utilization_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_stations_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_uptime_column_switch_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub show_last_seen_column_switch_row: TemplateChild<adw::SwitchRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesDialog {
        const NAME: &'static str = "PreferencesDialog";
        type Type = super::PreferencesDialog;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template()
        }
    }

    impl ObjectImpl for PreferencesDialog {
        fn constructed(&self) {
            self.parent_constructed();
            self.setup_settings();
        }
    }

    impl PreferencesDialog {
        fn setup_settings(&self) {
            let settings = gio::Settings::new(crate::config::app_id());

            settings
                .bind(
                    "show-hidden-bsss",
                    &*self.show_hidden_bsss_switch_row,
                    "active",
                )
                .build();

            self.setup_bss_retention_duration(&settings);
            self.setup_column_bindings(&settings);
        }

        fn setup_bss_retention_duration(&self, settings: &gio::Settings) {
            self.bss_retention_duration_combo_row.set_selected(
                match settings.int("bss-retention-duration") {
                    60 => 0,
                    120 => 1,
                    300 => 2,
                    600 => 3,
                    1800 => 4,
                    3600 => 5,
                    _ => 2,
                },
            );

            self.bss_retention_duration_combo_row
                .connect_selected_notify(glib::clone!(
                    #[strong]
                    settings,
                    move |row| {
                        let seconds = match row.selected() {
                            0 => 60,
                            1 => 120,
                            2 => 300,
                            3 => 600,
                            4 => 1800,
                            5 => 3600,
                            _ => 300,
                        };
                        if let Err(e) = settings.set_int("bss-retention-duration", seconds) {
                            tracing::warn!("Unable to set BSS retention duration: {:?}", e);
                        }
                    }
                ));
        }

        fn setup_column_bindings(&self, settings: &gio::Settings) {
            for (key, row) in [
                (
                    column_settings::SHOW_BSSID,
                    &*self.show_bssid_column_switch_row,
                ),
                (
                    column_settings::SHOW_VENDOR,
                    &*self.show_vendor_column_switch_row,
                ),
                (
                    column_settings::SHOW_SIGNAL,
                    &*self.show_signal_column_switch_row,
                ),
                (
                    column_settings::SHOW_CHANNEL,
                    &*self.show_channel_column_switch_row,
                ),
                (
                    column_settings::SHOW_CHANNEL_WIDTH,
                    &*self.show_channel_width_column_switch_row,
                ),
                (
                    column_settings::SHOW_FREQUENCY,
                    &*self.show_frequency_column_switch_row,
                ),
                (
                    column_settings::SHOW_BAND,
                    &*self.show_band_column_switch_row,
                ),
                (
                    column_settings::SHOW_PROTOCOLS,
                    &*self.show_protocols_column_switch_row,
                ),
                (
                    column_settings::SHOW_AMENDMENTS,
                    &*self.show_amendments_column_switch_row,
                ),
                (
                    column_settings::SHOW_SECURITY,
                    &*self.show_security_column_switch_row,
                ),
                (
                    column_settings::SHOW_MAX_RATE,
                    &*self.show_max_rate_column_switch_row,
                ),
                (
                    column_settings::SHOW_CHANNEL_UTILIZATION,
                    &*self.show_channel_utilization_column_switch_row,
                ),
                (
                    column_settings::SHOW_STATIONS,
                    &*self.show_stations_column_switch_row,
                ),
                (
                    column_settings::SHOW_UPTIME,
                    &*self.show_uptime_column_switch_row,
                ),
                (
                    column_settings::SHOW_LAST_SEEN,
                    &*self.show_last_seen_column_switch_row,
                ),
            ] {
                settings.bind(key, row, "active").build();
            }
        }
    }

    impl WidgetImpl for PreferencesDialog {}
    impl AdwDialogImpl for PreferencesDialog {}
    impl PreferencesDialogImpl for PreferencesDialog {}
}

glib::wrapper! {
    pub struct PreferencesDialog(ObjectSubclass<imp::PreferencesDialog>)
        @extends gtk::Widget, adw::PreferencesDialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, adw::Dialog;
}

impl PreferencesDialog {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
