use gtk::glib;
use gtk::subclass::prelude::*;
use kawaiifi;

mod imp {
    use gtk::Label;

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/kawaiifi/ui/interface_popover.ui")]
    pub struct InterfacePopover {
        #[template_child]
        pub(crate) interface_name_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) interface_vendor_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) interface_device_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) interface_bus_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) interface_driver_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) interface_mac_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) interface_index_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) interface_wiphy_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) interface_wdev_label: TemplateChild<Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InterfacePopover {
        const NAME: &'static str = "InterfacePopover";
        type Type = super::InterfacePopover;
        type ParentType = gtk::Popover;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for InterfacePopover {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for InterfacePopover {}
    impl PopoverImpl for InterfacePopover {}
}

glib::wrapper! {
    pub struct InterfacePopover(ObjectSubclass<imp::InterfacePopover>)
        @extends gtk::Widget, gtk::Popover,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::ShortcutManager;
}

impl InterfacePopover {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_interface(&self, interface: &kawaiifi::Interface) {
        let imp = self.imp();

        imp.interface_name_label.set_label(interface.name());
        imp.interface_vendor_label.set_label(&format!(
            "{} ({:#06X})",
            &interface
                .vendor_name()
                .unwrap_or_else(|| "Unknown".to_string()),
            interface.vendor_id().unwrap_or_default()
        ));
        imp.interface_device_label.set_label(&format!(
            "{} ({:#06X})",
            &interface
                .device_name()
                .unwrap_or_else(|| "Unknown".to_string()),
            interface.device_id().unwrap_or_default()
        ));
        imp.interface_bus_label
            .set_label(&interface.bus_type().to_string());
        imp.interface_driver_label
            .set_label(&interface.driver().unwrap_or_else(|| "Unknown".to_string()));
        let mut mac_address = String::with_capacity(17); // "XX:XX:XX:XX:XX:XX"
        for (i, byte) in interface.mac_address().iter().enumerate() {
            if i > 0 {
                mac_address.push(':');
            }
            mac_address.push_str(&format!("{:02X}", byte));
        }
        imp.interface_mac_label.set_label(&mac_address);
        imp.interface_index_label
            .set_label(&interface.index().to_string());
        imp.interface_wiphy_label
            .set_label(&interface.wiphy().to_string());
        imp.interface_wdev_label
            .set_label(&format!("{:#02X}", interface.wdev()));
    }
}

impl Default for InterfacePopover {
    fn default() -> Self {
        Self::new()
    }
}
