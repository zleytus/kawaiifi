use adw::subclass::prelude::*;
use gtk::gio::prelude::{FileExt, ListModelExt, ListModelExtManual};
use gtk::{gio, glib};

use crate::objects::BssObject;
use crate::scan_file::ScanFile;
use crate::widgets::{BssChart, BssElements, BssTable};

mod scanning;
mod setup;

mod imp {
    use std::{
        cell::{Cell, OnceCell, RefCell},
        sync::{Arc, Mutex, OnceLock},
    };

    use std::rc::Rc;

    use adw::ButtonContent;
    use gtk::{
        Button, Label, MenuButton, Revealer, SearchEntry, ToggleButton, glib::types::StaticType,
    };

    use super::*;
    use crate::{
        vendor_cache::VendorCache,
        widgets::{InterfaceBox, ScanInfoPopover},
    };

    pub const SIGNAL_SCAN_STARTED: &str = "scan-started";
    pub const SIGNAL_SCAN_COMPLETED: &str = "scan-completed";
    pub const SIGNAL_SCAN_FAILED: &str = "scan-failed";

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/kawaiifi/ui/window.ui")]
    pub struct KawaiiFiWindow {
        // Models
        pub bss_list_store: OnceCell<gio::ListStore>, // All BSSs
        pub bss_filter_model: OnceCell<gtk::FilterListModel>, // Filtered view of BSSs
        pub bss_filter: OnceCell<gtk::CustomFilter>,

        // Caches
        pub vendor_cache: OnceCell<Arc<Mutex<VendorCache>>>,

        // Scanning state
        pub scan_interval_seconds: Cell<u32>, // Configurable interval
        pub scanning_enabled: Cell<bool>,
        pub scan_source_id: RefCell<Option<glib::SourceId>>, // To cancel scan timer

        // Filter state (index-based: true = item is included in filter)
        pub band_filter_state: Rc<RefCell<Vec<bool>>>,
        pub security_filter_state: Rc<RefCell<Vec<bool>>>,
        pub width_filter_state: Rc<RefCell<Vec<bool>>>,
        pub protocols_filter_state: Rc<RefCell<Vec<bool>>>,
        pub text_query: RefCell<String>,

        // UI components
        #[template_child]
        pub start_scanning_button: TemplateChild<Button>,
        #[template_child]
        pub stop_scanning_button: TemplateChild<Button>,
        #[template_child]
        pub search_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub search_entry: TemplateChild<SearchEntry>,
        #[template_child]
        pub search_bar: TemplateChild<Revealer>,
        #[template_child]
        pub band_filter_button: TemplateChild<MenuButton>,
        #[template_child]
        pub security_filter_button: TemplateChild<MenuButton>,
        #[template_child]
        pub width_filter_button: TemplateChild<MenuButton>,
        #[template_child]
        pub protocols_filter_button: TemplateChild<MenuButton>,
        #[template_child]
        pub reset_filter_button: TemplateChild<Button>,
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
        pub scan_info_popover: TemplateChild<ScanInfoPopover>,
        #[template_child]
        pub statusbar_content: TemplateChild<ButtonContent>,
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

            // Create the ListStore for all BSSs
            let store = gio::ListStore::new::<BssObject>();

            // Create the FilterModel
            let filter = gtk::CustomFilter::new(|_| {
                // Initially show all items
                true
            });
            let filter_model = gtk::FilterListModel::new(Some(store.clone()), Some(filter.clone()));
            filter_model.connect_items_changed(glib::clone!(
                #[weak(rename_to = window)]
                self.obj(),
                move |_, _, _, _| {
                    window.update_status_bar();
                }
            ));

            // Store the models
            self.bss_list_store.set(store).unwrap();
            self.bss_filter.set(filter).unwrap();
            self.bss_filter_model.set(filter_model).unwrap();

            // Initialize the vendor cache
            self.vendor_cache
                .set(Arc::new(Mutex::new(VendorCache::default())))
                .unwrap();

            self.obj().connect_components_to_models();
            self.obj().connect_components_to_signals();

            self.obj().setup_actions();
            self.obj().setup_search();
            self.obj().setup_filter_buttons();
            self.obj().setup_scan_controls();
            self.obj().setup_bottom_panel_toggles();
            self.obj().show_cached_scan_results();
            self.obj().start_scanning(10);
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
        self.imp().bss_filter.get().unwrap()
    }

    pub fn bss_filter_model(&self) -> &gtk::FilterListModel {
        self.imp().bss_filter_model.get().unwrap()
    }

    fn kwifi_file_filter() -> gtk::FileFilter {
        let filter = gtk::FileFilter::new();
        filter.set_name(Some("KawaiiFi Scan (.kwifi)"));
        filter.add_suffix("kwifi");
        filter
    }

    pub fn open(&self) {
        self.stop_scanning();

        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&Self::kwifi_file_filter());

        let dialog = gtk::FileDialog::builder()
            .title("Open")
            .filters(&filters)
            .build();

        dialog.open(
            Some(self),
            None::<&gio::Cancellable>,
            glib::clone!(
                #[weak(rename_to = window)]
                self,
                move |result| {
                    let Ok(file) = result else {
                        return;
                    };
                    let path = file.path().expect("File should have a path");
                    match std::fs::read_to_string(&path) {
                        Ok(json) => match ScanFile::from_json(&json) {
                            Ok(scan_file) => {
                                window.apply_loaded_scan(scan_file, &path);
                            }
                            Err(e) => tracing::error!(error = %e, "Failed to parse scan file"),
                        },
                        Err(e) => tracing::error!(error = %e, "Failed to read scan file"),
                    }
                }
            ),
        );
    }

    pub fn save_all(&self) {
        let bss_list: Vec<kawaiifi::Bss> = self
            .bss_list_store()
            .iter::<BssObject>()
            .filter_map(|obj| obj.ok())
            .map(|obj| kawaiifi::Bss::clone(&obj.bss()))
            .collect();
        if !bss_list.is_empty() {
            self.save("All", "All-BSSs", bss_list);
        }
    }

    pub fn save_visible(&self) {
        let bss_list: Vec<kawaiifi::Bss> = self
            .bss_filter_model()
            .iter::<BssObject>()
            .filter_map(|obj| obj.ok())
            .map(|obj| kawaiifi::Bss::clone(&obj.bss()))
            .collect();
        if !bss_list.is_empty() {
            self.save("Visible", "Visible-BSSs", bss_list);
        }
    }

    pub fn save_selected(&self) {
        let bss_list: Vec<kawaiifi::Bss> = self
            .imp()
            .bss_table
            .selected_bss()
            .map(|obj| kawaiifi::Bss::clone(&obj.bss()))
            .into_iter()
            .collect();
        if !bss_list.is_empty() {
            self.save("Selected", "Selected-BSSs", bss_list);
        }
    }

    fn save(&self, title: &str, initial_name: &str, bss_list: Vec<kawaiifi::Bss>) {
        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&Self::kwifi_file_filter());

        let dialog = gtk::FileDialog::builder()
            .title(title)
            .initial_name(format!("{}.kwifi", initial_name))
            .filters(&filters)
            .build();

        dialog.save(Some(self), None::<&gio::Cancellable>, move |result| {
            let Ok(file) = result else {
                return;
            };
            let path = file.path().expect("File should have a path");
            let scan_file = ScanFile::new(bss_list);
            match scan_file.to_json() {
                Ok(json) => {
                    if let Err(e) = std::fs::write(&path, json) {
                        tracing::error!(error = %e, "Failed to write scan file");
                    }
                }
                Err(e) => tracing::error!(error = %e, "Failed to serialize scan file"),
            }
        });
    }

    fn update_status_bar(&self) {
        let total = self.bss_list_store().n_items();
        let displayed = self.bss_filter_model().n_items();
        let label = if total == 1 {
            "1 BSS".to_string()
        } else if total != displayed {
            format!("{} BSSs ({} displayed)", total, displayed)
        } else {
            format!("{} BSSs", total)
        };
        self.imp().statusbar_content.set_label(&label);
    }
}
