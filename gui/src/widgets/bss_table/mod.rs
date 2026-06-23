mod columns;
mod setup;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk::gio::prelude::ListModelExt;
use gtk::glib::object::CastNone;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::*;
use gtk::{ListScrollFlags, SingleSelection, SortListModel, gio, glib};

use crate::objects::BssObject;
use crate::widgets::column_view;
use columns::update_last_seen_label;

pub(super) mod column_settings {
    pub const SHOW_BSSID: &str = "show-bssid-column";
    pub const SHOW_VENDOR: &str = "show-vendor-column";
    pub const SHOW_SIGNAL: &str = "show-signal-column";
    pub const SHOW_CHANNEL: &str = "show-channel-column";
    pub const SHOW_CHANNEL_WIDTH: &str = "show-channel-width-column";
    pub const SHOW_FREQUENCY: &str = "show-frequency-column";
    pub const SHOW_BAND: &str = "show-band-column";
    pub const SHOW_PROTOCOLS: &str = "show-protocols-column";
    pub const SHOW_AMENDMENTS: &str = "show-amendments-column";
    pub const SHOW_SECURITY: &str = "show-security-column";
    pub const SHOW_MAX_RATE: &str = "show-max-rate-column";
    pub const SHOW_CHANNEL_UTILIZATION: &str = "show-channel-utilization-column";
    pub const SHOW_STATIONS: &str = "show-stations-column";
    pub const SHOW_STREAMS: &str = "show-streams-column";
    pub const SHOW_UPTIME: &str = "show-uptime-column";
    pub const SHOW_LAST_SEEN: &str = "show-last-seen-column";
}

mod imp {
    use std::cell::OnceCell;

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/bss_table.ui")]
    pub struct BssTable {
        pub sort_model: OnceCell<gtk::SortListModel>,
        pub selection_model: OnceCell<gtk::SingleSelection>,

        /// Labels currently bound to the Last Seen column (for periodic refresh)
        pub bound_last_seen_labels: Rc<RefCell<Vec<(gtk::Label, BssObject)>>>,
        /// Timer for refreshing Last Seen labels
        pub last_seen_timer_id: RefCell<Option<glib::SourceId>>,

        #[template_child]
        pub column_view: TemplateChild<gtk::ColumnView>,
        #[template_child]
        pub color_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub ssid_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub bssid_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub vendor_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub signal_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub channel_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub channel_width_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub frequency_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub band_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub protocols_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub amendments_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub security_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub max_rate_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub channel_utilization_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub station_count_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub streams_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub uptime_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub last_seen_column: TemplateChild<gtk::ColumnViewColumn>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BssTable {
        const NAME: &'static str = "BssTable";
        type Type = super::BssTable;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BssTable {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_columns();
            obj.setup_column_visibility();
            obj.start_last_seen_refresh_timer();
        }

        fn dispose(&self) {
            self.obj().stop_last_seen_refresh_timer();
        }
    }
    impl WidgetImpl for BssTable {
        fn realize(&self) {
            self.parent_realize();

            let column_view_title = self.column_view.first_child();
            column_view::adjust_header_alignment(
                column_view_title,
                &[
                    "Band",
                    "Channel",
                    "Channel Width",
                    "Frequency",
                    "Max Rate",
                    "Stations",
                    "Streams",
                    "Uptime",
                    "Last Seen",
                ],
            );
        }
    }
    impl BoxImpl for BssTable {}
}

glib::wrapper! {
    pub struct BssTable(ObjectSubclass<imp::BssTable>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Orientable;
}

impl BssTable {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn setup(&self, filter_model: &gtk::FilterListModel) {
        let imp = self.imp();
        if imp.selection_model.get().is_some() {
            return;
        }

        // Create sort model (wraps filtered data)
        let sort_model = SortListModel::new(Some(filter_model.clone()), imp.column_view.sorter());

        // Create selection model (wraps sorted data)
        let selection_model = SingleSelection::new(Some(sort_model.clone()));

        imp.sort_model.set(sort_model).unwrap();
        imp.selection_model.set(selection_model.clone()).unwrap();

        // Set model on column view
        imp.column_view.set_model(Some(&selection_model));

        self.setup_column_sorters();
    }

    pub fn selection_model(&self) -> Option<&SingleSelection> {
        self.imp().selection_model.get()
    }

    pub fn selected_bss(&self) -> Option<BssObject> {
        self.imp()
            .selection_model
            .get()
            .and_then(|selection_model| selection_model.selected_item())
            .and_downcast::<BssObject>()
    }

    pub fn set_selected_by_bssid(&self, bssid: &[u8; 6]) {
        let Some(selection_model) = self.imp().selection_model.get() else {
            return;
        };

        let Some(index_of_selected) = (0..selection_model.n_items()).find(|&i| {
            selection_model
                .item(i)
                .and_downcast::<BssObject>()
                .is_some_and(|bss| bss.data().bssid() == bssid)
        }) else {
            return;
        };

        self.imp()
            .column_view
            .scroll_to(index_of_selected, None, ListScrollFlags::SELECT, None);
    }

    fn start_last_seen_refresh_timer(&self) {
        let bound_labels = self.imp().bound_last_seen_labels.clone();
        let source_id = glib::timeout_add_local(Duration::from_secs(10), move || {
            for (label, bss) in bound_labels.borrow().iter() {
                update_last_seen_label(label, bss);
            }
            glib::ControlFlow::Continue
        });
        self.imp().last_seen_timer_id.replace(Some(source_id));
    }

    fn stop_last_seen_refresh_timer(&self) {
        if let Some(source_id) = self.imp().last_seen_timer_id.take() {
            source_id.remove();
        }
    }
}

impl Default for BssTable {
    fn default() -> Self {
        Self::new()
    }
}
