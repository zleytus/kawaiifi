use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio::prelude::ListModelExt;
use gtk::{gio, glib};

use crate::config;
use crate::objects::BssObject;
use crate::widgets::{BssChart, BssElements, BssFilter, BssTable};

mod scan_file_actions;
mod scanning;
mod setup;

/// Interval between automatic Wi-Fi scans, in seconds.
const SCAN_INTERVAL_SECONDS: u32 = 10;

mod imp {
    use std::{
        cell::{Cell, OnceCell, RefCell},
        sync::{Arc, Mutex, OnceLock},
    };

    use gtk::{Button, Label, ToggleButton, Widget, glib::types::StaticType};

    use super::*;

    pub const SIGNAL_SCAN_STARTED: &str = "scan-started";
    pub const SIGNAL_SCAN_COMPLETED: &str = "scan-completed";
    pub const SIGNAL_SCAN_FAILED: &str = "scan-failed";

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/window.ui")]
    pub struct KawaiiFiWindow {
        // Models
        pub bss_list_store: OnceCell<gio::ListStore>, // All BSSs
        pub bss_filter_model: OnceCell<gtk::FilterListModel>, // Filtered view of BSSs
        pub bss_custom_filter: OnceCell<gtk::CustomFilter>,
        pub settings: OnceCell<gio::Settings>,

        // Caches
        pub vendor_cache: OnceCell<Arc<Mutex<VendorCache>>>,

        // Scanning state
        pub scanning_enabled: Cell<bool>,
        pub scan_source_id: RefCell<Option<glib::SourceId>>, // To cancel scan timer

        // UI components
        #[template_child]
        pub start_scanning_button: TemplateChild<Button>,
        #[template_child]
        pub stop_scanning_button: TemplateChild<Button>,
        #[template_child]
        pub active_scan_spinner: TemplateChild<Widget>,
        #[template_child]
        pub filter_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub overlay_split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub bss_filter: TemplateChild<BssFilter>,
        #[template_child]
        pub interface_box: TemplateChild<InterfaceBox>,
        #[template_child]
        pub file_label: TemplateChild<Label>,
        #[template_child]
        pub bss_table: TemplateChild<BssTable>,
        #[template_child]
        pub bss_chart_2_4: TemplateChild<BssChart>,
        #[template_child]
        pub bss_chart_5: TemplateChild<BssChart>,
        #[template_child]
        pub bss_chart_6: TemplateChild<BssChart>,
        #[template_child]
        pub bss_elements: TemplateChild<BssElements>,

        // Bottom Panel
        #[template_child]
        pub statusbar_label: TemplateChild<Label>,
        #[template_child]
        pub bottom_stack: TemplateChild<adw::ViewStack>,

        // Toggle Buttons
        #[template_child]
        pub ies_toggle_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub two_point_four_ghz_toggle_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub five_ghz_toggle_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub six_ghz_toggle_button: TemplateChild<gtk::ToggleButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for KawaiiFiWindow {
        const NAME: &'static str = "KawaiiFiWindow";
        type Type = super::KawaiiFiWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for KawaiiFiWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let store = gio::ListStore::new::<BssObject>();

            let filter = gtk::CustomFilter::new(|_| true);
            let filter_model = gtk::FilterListModel::new(Some(store.clone()), Some(filter.clone()));
            filter_model.connect_items_changed(glib::clone!(
                #[weak(rename_to = window)]
                self.obj(),
                move |_, _, _, _| {
                    window.update_status_bar();
                }
            ));

            self.bss_list_store.set(store).unwrap();
            self.bss_custom_filter.set(filter).unwrap();
            self.bss_filter_model.set(filter_model).unwrap();
            self.settings
                .set(gio::Settings::new(config::app_id()))
                .unwrap();

            self.vendor_cache
                .set(Arc::new(Mutex::new(VendorCache::default())))
                .unwrap();

            self.obj().connect_components_to_models();

            self.interface_box
                .connect_interfaces_load_failed(glib::clone!(
                    #[weak(rename_to = window)]
                    self.obj(),
                    move |_, error| {
                        window.show_error("Could Not Load Wi-Fi Interfaces", error);
                    }
                ));

            self.obj().setup_actions();
            self.obj().setup_search();
            self.obj().setup_scan_controls();
            self.obj().setup_bottom_panel_toggles();
            self.obj().setup_settings();
            self.obj().show_cached_scan_results();
            self.obj().start_scanning();
        }

        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    // Signal: scan-started (no parameters)
                    glib::subclass::Signal::builder(SIGNAL_SCAN_STARTED).build(),
                    // Signal: scan-completed (no parameters)
                    glib::subclass::Signal::builder(SIGNAL_SCAN_COMPLETED).build(),
                    // Signal: scan-failed (passes error message)
                    glib::subclass::Signal::builder(SIGNAL_SCAN_FAILED)
                        .param_types([String::static_type()])
                        .build(),
                ]
            })
        }

        fn dispose(&self) {
            self.obj().stop_scanning();
        }
    }
    impl WidgetImpl for KawaiiFiWindow {}
    impl WindowImpl for KawaiiFiWindow {}
    impl ApplicationWindowImpl for KawaiiFiWindow {}
    impl AdwApplicationWindowImpl for KawaiiFiWindow {}
}

glib::wrapper! {
    pub struct KawaiiFiWindow(ObjectSubclass<imp::KawaiiFiWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl KawaiiFiWindow {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn bss_list_store(&self) -> &gio::ListStore {
        self.imp().bss_list_store.get().unwrap()
    }

    pub fn bss_filter(&self) -> &gtk::CustomFilter {
        self.imp().bss_custom_filter.get().unwrap()
    }

    pub fn settings(&self) -> &gio::Settings {
        self.imp().settings.get().unwrap()
    }

    pub fn bss_filter_model(&self) -> &gtk::FilterListModel {
        self.imp().bss_filter_model.get().unwrap()
    }

    pub(super) fn show_error(&self, heading: &str, body: impl AsRef<str>) {
        let dialog = adw::AlertDialog::new(Some(heading), Some(body.as_ref()));
        dialog.add_response("ok", "_OK");
        dialog.set_default_response(Some("ok"));
        dialog.present(Some(self));
    }

    pub(super) fn update_status_bar(&self) {
        let total = self.bss_list_store().n_items();
        let displayed = self.bss_filter_model().n_items();
        self.imp()
            .statusbar_label
            .set_label(&bss_status_label(total, displayed));
    }
}

fn bss_status_label(total: u32, displayed: u32) -> String {
    if total != displayed {
        format!("{} ({displayed} displayed)", bss_count_label(total))
    } else if total == 1 {
        "1 BSS".to_string()
    } else {
        format!("{total} BSSs")
    }
}

fn bss_count_label(count: u32) -> String {
    if count == 1 {
        "1 BSS".to_string()
    } else {
        format!("{count} BSSs")
    }
}

#[cfg(test)]
mod tests {
    use super::bss_status_label;

    #[test]
    fn bss_status_label_handles_filtered_singular_total() {
        assert_eq!(bss_status_label(1, 0), "1 BSS (0 displayed)");
    }

    #[test]
    fn bss_status_label_handles_unfiltered_counts() {
        assert_eq!(bss_status_label(0, 0), "0 BSSs");
        assert_eq!(bss_status_label(1, 1), "1 BSS");
        assert_eq!(bss_status_label(2, 2), "2 BSSs");
    }
}
