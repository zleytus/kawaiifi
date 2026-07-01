use adw::prelude::ActionRowExt;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum InterfaceState {
    #[default]
    Uninitialized,
    Loaded(Vec<kawaiifi::Interface>),
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceRefreshResult {
    SelectionUnchanged,
    SelectionChanged,
    NoInterfaces,
    Error(String),
}

mod row_imp {
    use std::cell::RefCell;

    use gtk::{Image, Label};

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/interface_row.ui")]
    pub struct InterfaceRow {
        #[template_child]
        pub bus_icon: TemplateChild<Image>,
        #[template_child]
        pub interface_name_label: TemplateChild<Label>,
        #[template_child]
        pub interface_summary_label: TemplateChild<Label>,
        pub interface: RefCell<Option<kawaiifi::Interface>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InterfaceRow {
        const NAME: &'static str = "InterfaceRow";
        type Type = super::InterfaceRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for InterfaceRow {}
    impl WidgetImpl for InterfaceRow {}
    impl ListBoxRowImpl for InterfaceRow {}
}

glib::wrapper! {
    pub struct InterfaceRow(ObjectSubclass<row_imp::InterfaceRow>)
        @extends gtk::Widget, gtk::ListBoxRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl InterfaceRow {
    fn new(interface: &kawaiifi::Interface) -> Self {
        let row: Self = glib::Object::new();
        row.set_interface(interface);
        row
    }

    fn set_interface(&self, interface: &kawaiifi::Interface) {
        let imp = self.imp();
        imp.bus_icon
            .set_icon_name(Some(interface_icon_name(interface.bus_type())));
        imp.interface_name_label.set_label(&format!(
            "{} ({})",
            interface.name(),
            interface.bus_type()
        ));
        imp.interface_summary_label
            .set_label(&interface_summary(interface));
        imp.interface.replace(Some(interface.clone()));
    }

    fn interface(&self) -> kawaiifi::Interface {
        self.imp()
            .interface
            .borrow()
            .as_ref()
            .expect("InterfaceRow not initialized")
            .clone()
    }
}

mod imp {
    use std::{
        cell::{Cell, RefCell},
        sync::OnceLock,
    };

    use adw::{ActionRow, PreferencesGroup};
    use gtk::{ListBox, glib::types::StaticType};

    use super::*;

    pub const SIGNAL_INTERFACE_CHANGED: &str = "interface-changed";

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/interface_list.ui")]
    pub struct InterfaceList {
        #[template_child]
        pub interface_list_box: TemplateChild<ListBox>,
        #[template_child]
        pub details_group: TemplateChild<PreferencesGroup>,
        #[template_child]
        pub name_row: TemplateChild<ActionRow>,
        #[template_child]
        pub vendor_row: TemplateChild<ActionRow>,
        #[template_child]
        pub device_row: TemplateChild<ActionRow>,
        #[template_child]
        pub bus_type_row: TemplateChild<ActionRow>,
        #[template_child]
        pub driver_row: TemplateChild<ActionRow>,
        #[template_child]
        pub ssid_row: TemplateChild<ActionRow>,
        #[template_child]
        pub mac_row: TemplateChild<ActionRow>,
        #[template_child]
        pub freq_row: TemplateChild<ActionRow>,
        #[template_child]
        pub index_row: TemplateChild<ActionRow>,
        #[template_child]
        pub wiphy_row: TemplateChild<ActionRow>,
        #[template_child]
        pub wdev_row: TemplateChild<ActionRow>,
        pub(super) selected_ifindex: Cell<Option<u32>>,
        pub(super) interface_state: RefCell<InterfaceState>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InterfaceList {
        const NAME: &'static str = "InterfaceList";
        type Type = super::InterfaceList;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for InterfaceList {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            self.interface_list_box.connect_row_selected(glib::clone!(
                #[weak]
                obj,
                move |_, row| {
                    let Some(row) = row.and_then(|row| row.downcast_ref::<InterfaceRow>()) else {
                        return;
                    };
                    obj.select_interface(&row.interface(), true);
                }
            ));
        }

        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder(SIGNAL_INTERFACE_CHANGED)
                        .param_types([u32::static_type()])
                        .build(),
                ]
            })
        }
    }

    impl WidgetImpl for InterfaceList {}
    impl BoxImpl for InterfaceList {}
}

glib::wrapper! {
    pub struct InterfaceList(ObjectSubclass<imp::InterfaceList>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl InterfaceList {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn refresh_interfaces(&self) -> InterfaceRefreshResult {
        let previous_ifindex = self.selected_interface_index();
        self.clear_rows();

        match kawaiifi::interfaces() {
            Ok(mut interfaces) => {
                interfaces.sort_by(|i1, i2| i1.name().cmp(i2.name()));
                for interface in &interfaces {
                    self.imp()
                        .interface_list_box
                        .append(&InterfaceRow::new(interface));
                }

                let selected_interface = previous_ifindex
                    .and_then(|index| {
                        interfaces
                            .iter()
                            .find(|interface| interface.index() == index)
                    })
                    .or_else(|| interfaces.first())
                    .cloned();

                self.imp()
                    .interface_state
                    .replace(InterfaceState::Loaded(interfaces));

                if let Some(interface) = selected_interface {
                    let selected_ifindex = interface.index();
                    self.select_interface(&interface, false);
                    self.select_row_by_index(interface.index());
                    if previous_ifindex == Some(selected_ifindex) {
                        InterfaceRefreshResult::SelectionUnchanged
                    } else {
                        InterfaceRefreshResult::SelectionChanged
                    }
                } else {
                    self.imp().selected_ifindex.set(None);
                    self.clear_interface_details();
                    InterfaceRefreshResult::NoInterfaces
                }
            }
            Err(error) => {
                tracing::error!(error = %error, "Failed to enumerate Wi-Fi interfaces");
                let message = error.to_string();
                self.imp().interface_state.replace(InterfaceState::Error);
                self.imp().selected_ifindex.set(None);
                self.clear_interface_details();
                InterfaceRefreshResult::Error(message)
            }
        }
    }

    fn clear_rows(&self) {
        while let Some(row) = self.imp().interface_list_box.first_child() {
            self.imp().interface_list_box.remove(&row);
        }
    }

    fn select_row_by_index(&self, ifindex: u32) {
        let mut child = self.imp().interface_list_box.first_child();
        while let Some(widget) = child {
            let next = widget.next_sibling();
            if let Ok(row) = widget.downcast::<InterfaceRow>()
                && row.interface().index() == ifindex
            {
                self.imp().interface_list_box.select_row(Some(&row));
                break;
            }
            child = next;
        }
    }

    fn select_interface(&self, interface: &kawaiifi::Interface, emit_change: bool) {
        let previous_ifindex = self.imp().selected_ifindex.replace(Some(interface.index()));
        self.update_interface_details(interface);
        if emit_change && previous_ifindex != Some(interface.index()) {
            self.emit_by_name::<()>(imp::SIGNAL_INTERFACE_CHANGED, &[&interface.index()]);
        }
    }

    fn update_interface_details(&self, interface: &kawaiifi::Interface) {
        let imp = self.imp();
        imp.name_row.set_subtitle(interface.name());
        if let Some(vendor) = interface.vendor_name() {
            imp.vendor_row.set_visible(true);
            imp.vendor_row.set_subtitle(&vendor);
        } else {
            imp.vendor_row.set_visible(false);
        }
        if let Some(device) = interface.device_name() {
            imp.device_row.set_visible(true);
            imp.device_row.set_subtitle(&device);
        } else {
            imp.device_row.set_visible(false);
        }
        imp.bus_type_row
            .set_subtitle(&interface.bus_type().to_string());
        if let Some(driver) = interface.driver() {
            imp.driver_row.set_visible(true);
            imp.driver_row.set_subtitle(&driver);
        } else {
            imp.driver_row.set_visible(false);
        }
        if let Some(ssid) = interface.ssid() {
            imp.ssid_row.set_visible(true);
            imp.ssid_row.set_subtitle(ssid);
        } else {
            imp.ssid_row.set_visible(false);
        }
        imp.mac_row
            .set_subtitle(&crate::mac::format_mac(&interface.mac_address()));
        if let Some(freq_mhz) = interface.wiphy_freq_mhz() {
            imp.freq_row.set_visible(true);
            imp.freq_row.set_subtitle(&format!("{} MHz", freq_mhz));
        } else {
            imp.freq_row.set_visible(false);
        }
        imp.index_row.set_subtitle(&interface.index().to_string());
        imp.wiphy_row.set_subtitle(&interface.wiphy().to_string());
        imp.wdev_row
            .set_subtitle(&format!("{:#X}", interface.wdev()));
        imp.details_group.set_visible(true);
    }

    fn clear_interface_details(&self) {
        self.imp().details_group.set_visible(false);
    }

    pub fn selected_interface_index(&self) -> Option<u32> {
        self.imp().selected_ifindex.get()
    }

    pub fn selected_interface(&self) -> Option<kawaiifi::Interface> {
        let index = self.selected_interface_index()?;
        match &*self.imp().interface_state.borrow() {
            InterfaceState::Loaded(interfaces) => interfaces
                .iter()
                .find(|interface| interface.index() == index)
                .cloned(),
            InterfaceState::Uninitialized | InterfaceState::Error => None,
        }
    }

    pub fn connect_interface_changed<F: Fn(&Self, u32) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local(imp::SIGNAL_INTERFACE_CHANGED, false, move |args| {
            let interface_list = args[0].get::<Self>().unwrap();
            let ifindex = args[1].get::<u32>().unwrap();
            f(&interface_list, ifindex);
            None
        })
    }
}

fn interface_icon_name(bus_type: kawaiifi::BusType) -> &'static str {
    match bus_type {
        kawaiifi::BusType::Pci => "pci-card-symbolic",
        kawaiifi::BusType::Usb => "drive-harddisk-usb-symbolic",
        _ => "network-wireless-symbolic",
    }
}

fn interface_summary(interface: &kawaiifi::Interface) -> String {
    interface
        .vendor_name()
        .or_else(|| interface.driver())
        .unwrap_or_else(|| "Unknown adapter".to_string())
}

impl Default for InterfaceList {
    fn default() -> Self {
        Self::new()
    }
}
