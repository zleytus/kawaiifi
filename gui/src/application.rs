use adw::AboutDialog;
use adw::prelude::AdwDialogExt;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::config;
use crate::widgets::{BssChart, BssElements, BssFilter, BssTable, PreferencesDialog};
use crate::window::KawaiiFiWindow;

mod imp {
    use crate::widgets::{InterfaceBox, InterfacePopover, ScanInfoPopover};

    use super::*;

    #[derive(Default)]
    pub struct KawaiiFiApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for KawaiiFiApplication {
        const NAME: &'static str = "KawaiiFiApplication";
        type Type = super::KawaiiFiApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for KawaiiFiApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_actions();
        }
    }

    fn register_custom_types() {
        InterfaceBox::static_type();
        InterfacePopover::static_type();
        ScanInfoPopover::static_type();
        BssTable::static_type();
        BssChart::static_type();
        BssFilter::static_type();
        BssElements::static_type();
        PreferencesDialog::static_type();
    }

    fn load_icons() {
        gtk::IconTheme::for_display(
            &gtk::gdk::Display::default().expect("Could not connect to a display"),
        )
        .add_resource_path("/fi/kawaii/kawaiifi/icons");
    }

    fn load_css() {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/fi/kawaii/kawaiifi/style.css");

        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to a display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    impl ApplicationImpl for KawaiiFiApplication {
        fn startup(&self) {
            self.parent_startup();

            // Register custom widget types
            register_custom_types();

            // Load icons and CSS
            load_icons();
            load_css();
        }

        fn activate(&self) {
            let application = self.obj();
            // Get the first window if exists, or create a new one
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = KawaiiFiWindow::new(application.upcast_ref());
                window.upcast()
            };
            window.present();
        }
    }

    impl GtkApplicationImpl for KawaiiFiApplication {}
    impl AdwApplicationImpl for KawaiiFiApplication {}
}

glib::wrapper! {
    pub struct KawaiiFiApplication(ObjectSubclass<imp::KawaiiFiApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl KawaiiFiApplication {
    pub fn new(application_id: &str) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", gio::ApplicationFlags::default())
            .build()
    }

    fn setup_actions(&self) {
        // Accels for window actions
        self.set_accels_for_action("win.open", &["<Ctrl>O"]);
        self.set_accels_for_action("win.save-visible", &["<Ctrl>S"]);
        self.set_accels_for_action("win.save-selected", &["<Ctrl><Shift>S"]);
        self.set_accels_for_action("win.save-all", &["<Ctrl><Alt>S"]);

        // Preferences action
        let action_preferences = gio::SimpleAction::new("preferences", None);
        action_preferences.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_, _| {
                let dialog = PreferencesDialog::new();
                dialog.present(app.active_window().as_ref());
            }
        ));
        self.add_action(&action_preferences);
        self.set_accels_for_action("app.preferences", &["<Ctrl>comma"]);

        // About action
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_, _| {
                let dialog = AboutDialog::builder()
                    .application_name("KawaiiFi")
                    .application_icon(config::app_id())
                    .version(env!("CARGO_PKG_VERSION"))
                    .website("https://kawaii.fi")
                    .license_type(gtk::License::Custom)
                    .license("Licensed under either the Apache License, Version 2.0 or the MIT License, at your option.")
                    .issue_url("https://github.com/zleytus/kawaiifi/issues")
                    .build();
                dialog.present(app.active_window().as_ref());
            }
        ));
        self.add_action(&action_about);
    }
}
