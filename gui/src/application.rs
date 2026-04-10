use adw::AboutDialog;
use adw::prelude::AdwDialogExt;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::widgets::{BssCapabilityInfo, BssChart, BssElements, BssFilter, BssTable};
use crate::window::KawaiiFiWindow;

mod imp {
    use crate::widgets::{InterfaceBox, InterfacePopover, ProbeRequest, ScanInfoPopover};

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
        BssCapabilityInfo::static_type();
        ProbeRequest::static_type();
    }

    fn load_icons() {
        gtk::IconTheme::for_display(
            &gtk::gdk::Display::default().expect("Could not connect to a display"),
        )
        .add_resource_path("/com/github/kawaiifi/icons");
    }

    fn load_css() {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/com/github/kawaiifi/style.css");

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
        // About action
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_, _| {
                let dialog = AboutDialog::builder()
                    .application_name("KawaiiFi")
                    .version("1.0.0")
                    .website("https://kawaii.fi")
                    .license_type(gtk::License::Apache20)
                    .issue_url("https://github.com/zleytus/kawaiifi/issues")
                    .build();
                dialog.present(app.active_window().as_ref());
            }
        ));
        self.add_action(&action_about);
    }
}
