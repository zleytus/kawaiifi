mod columns;

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
use columns::*;

mod imp {
    use std::cell::OnceCell;

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/kawaiifi/ui/bss_table.ui")]
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
        pub channel_utilization_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub station_count_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub max_rate_column: TemplateChild<gtk::ColumnViewColumn>,
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
            .unwrap()
            .selected_item()
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
                .is_some_and(|bss| &bss.bssid_bytes() == bssid)
        }) else {
            return;
        };

        self.imp()
            .column_view
            .scroll_to(index_of_selected, None, ListScrollFlags::SELECT, None);
    }

    fn setup_columns(&self) {
        let imp = self.imp();

        // Set up factories for each column
        imp.color_column.set_factory(Some(&create_color_factory()));
        imp.ssid_column.set_factory(Some(&create_ssid_factory()));
        imp.bssid_column.set_factory(Some(&create_bssid_factory()));
        imp.vendor_column
            .set_factory(Some(&create_vendor_factory()));
        imp.signal_column
            .set_factory(Some(&create_signal_factory()));
        imp.channel_column
            .set_factory(Some(&create_channel_factory()));
        imp.channel_width_column
            .set_factory(Some(&create_channel_width_factory()));
        imp.frequency_column
            .set_factory(Some(&create_frequency_factory()));
        imp.band_column.set_factory(Some(&create_band_factory()));
        imp.protocols_column
            .set_factory(Some(&create_protocols_factory()));
        imp.amendments_column
            .set_factory(Some(&create_amendments_factory()));
        imp.security_column
            .set_factory(Some(&create_security_factory()));
        imp.channel_utilization_column
            .set_factory(Some(&create_channel_utilization_factory()));
        imp.station_count_column
            .set_factory(Some(&create_station_count_factory()));
        imp.max_rate_column
            .set_factory(Some(&create_max_rate_factory()));
        imp.uptime_column
            .set_factory(Some(&create_uptime_factory()));

        // Last Seen column needs the bound_labels collection for refresh tracking
        imp.last_seen_column
            .set_factory(Some(&create_last_seen_factory(
                imp.bound_last_seen_labels.clone(),
            )));
    }

    fn setup_column_sorters(&self) {
        let imp = self.imp();

        imp.ssid_column.set_sorter(Some(&create_ssid_sorter()));
        imp.bssid_column.set_sorter(Some(&create_bssid_sorter()));
        imp.vendor_column.set_sorter(Some(&create_vendor_sorter()));
        imp.signal_column.set_sorter(Some(&create_signal_sorter()));
        imp.channel_column
            .set_sorter(Some(&create_channel_sorter()));
        imp.frequency_column
            .set_sorter(Some(&create_frequency_sorter()));
        imp.band_column.set_sorter(Some(&create_band_sorter()));
        imp.channel_width_column
            .set_sorter(Some(&create_channel_width_sorter()));
        imp.protocols_column
            .set_sorter(Some(&create_protocols_sorter()));
        imp.amendments_column
            .set_sorter(Some(&create_amendments_sorter()));
        imp.security_column
            .set_sorter(Some(&create_security_sorter()));
        imp.channel_utilization_column
            .set_sorter(Some(&create_channel_utilization_sorter()));
        imp.station_count_column
            .set_sorter(Some(&create_station_count_sorter()));
        imp.max_rate_column
            .set_sorter(Some(&create_max_rate_sorter()));
        imp.uptime_column.set_sorter(Some(&create_uptime_sorter()));
        imp.last_seen_column
            .set_sorter(Some(&create_last_seen_sorter()));

        imp.column_view
            .sort_by_column(Some(&imp.signal_column), gtk::SortType::Descending);
    }

    /// Start a timer that periodically refreshes the "Last Seen" labels
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

    /// Stop the "Last Seen" refresh timer
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
