use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio::prelude::{ListModelExt, SettingsExt};
use gtk::glib::object::Cast;
use gtk::{gio, glib};
use kawaiifi::Interface;

use crate::config;
use crate::objects::{BssInternal, BssObject};
use crate::vendor::VendorCache;
use crate::widgets::{BssChart, BssElements, BssFilter, BssTable};

mod scan_file_actions;
mod scanning;
mod setup;

/// Interval between automatic Wi-Fi scans, in seconds.
const SCAN_INTERVAL_SECONDS: u64 = 10;

mod imp {
    use std::cell::{Cell, OnceCell, RefCell};

    use adw::Banner;
    use gtk::{Button, Label, ToggleButton, Widget};

    use super::*;
    use crate::{
        vendor::VendorCache,
        widgets::{InterfaceList, InterfaceToggle},
    };

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/window.ui")]
    pub struct KawaiiFiWindow {
        // Models
        pub bss_list_store: OnceCell<gio::ListStore>, // All BSSs
        pub bss_filter_model: OnceCell<gtk::FilterListModel>, // Filtered view of BSSs
        pub bss_custom_filter: OnceCell<gtk::CustomFilter>,
        pub settings: OnceCell<gio::Settings>,

        // Caches
        pub vendor_cache: OnceCell<RefCell<VendorCache>>,

        // Scanning state
        pub scanning_enabled: Cell<bool>,
        pub scan_source_id: RefCell<Option<glib::SourceId>>, // To cancel scan timer
        pub scan_generation: Cell<u64>,

        // UI components
        #[template_child]
        pub start_scanning_button: TemplateChild<Button>,
        #[template_child]
        pub stop_scanning_button: TemplateChild<Button>,
        #[template_child]
        pub active_scan_spinner: TemplateChild<Widget>,
        #[template_child]
        pub scan_failed_banner: TemplateChild<Banner>,
        #[template_child]
        pub filter_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub interface_split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub overlay_split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub bss_filter: TemplateChild<BssFilter>,
        #[template_child]
        pub interface_toggle: TemplateChild<InterfaceToggle>,
        #[template_child]
        pub refresh_interfaces_button: TemplateChild<Button>,
        #[template_child]
        pub interface_list: TemplateChild<InterfaceList>,
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
                .set(RefCell::new(VendorCache::default()))
                .unwrap();

            self.obj().setup();
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

    pub fn bss_filter_model(&self) -> &gtk::FilterListModel {
        self.imp().bss_filter_model.get().unwrap()
    }

    pub fn bss_filter(&self) -> &gtk::CustomFilter {
        self.imp().bss_custom_filter.get().unwrap()
    }

    pub fn settings(&self) -> &gio::Settings {
        self.imp().settings.get().unwrap()
    }

    pub(super) fn load_interface(&self, interface: Interface, start_scanning: bool) {
        let was_showing_file = self.imp().file_label.is_visible();
        self.imp().file_label.set_visible(false);
        self.imp().interface_toggle.set_visible(true);

        if was_showing_file {
            self.invalidate_scan_generation();
            self.apply_merged_results(Vec::new());
        }

        if start_scanning {
            self.start_scanning(interface);
        }
    }

    pub(super) fn current_bss_data(&self) -> Vec<BssInternal> {
        self.bss_list_store()
            .iter::<BssObject>()
            .filter_map(|obj| obj.ok())
            .map(|obj| obj.data().clone())
            .collect()
    }

    pub(super) fn apply_merged_results(&self, merged_bss_list: Vec<BssInternal>) {
        let bss_objects: Vec<BssObject> = merged_bss_list.into_iter().map(BssObject::new).collect();

        let list_store = self.bss_list_store();
        list_store.splice(0, list_store.n_items(), &bss_objects);
        self.update_status_bar();
    }

    pub(super) fn update_filter(&self) {
        let filter = self.bss_filter();
        let show_hidden = self.settings().boolean("show-hidden-bsss");
        let state = self.imp().bss_filter.state(show_hidden);

        filter.set_filter_func(move |obj| {
            let bss = obj.downcast_ref::<BssObject>().unwrap();
            state.matches(bss)
        });
    }

    pub(super) fn scan_generation(&self) -> u64 {
        self.imp().scan_generation.get()
    }

    pub(super) fn invalidate_scan_generation(&self) {
        self.imp()
            .scan_generation
            .set(self.scan_generation().wrapping_add(1));
    }

    pub(super) fn generation_is_current(&self, generation: u64) -> bool {
        self.scan_generation() == generation
    }

    pub(super) fn vendor_cache_snapshot(&self) -> VendorCache {
        self.imp().vendor_cache.get().unwrap().borrow().clone()
    }

    pub(super) fn install_vendor_cache(&self, vendor_cache: VendorCache) {
        *self.imp().vendor_cache.get().unwrap().borrow_mut() = vendor_cache;
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
