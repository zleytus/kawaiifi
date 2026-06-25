use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use adw::ButtonContent;
    use gtk::ToggleButton;

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/interface_toggle.ui")]
    pub struct InterfaceToggle {
        #[template_child]
        pub interface_button: TemplateChild<ToggleButton>,
        #[template_child]
        pub interface_button_content: TemplateChild<ButtonContent>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InterfaceToggle {
        const NAME: &'static str = "InterfaceToggle";
        type Type = super::InterfaceToggle;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for InterfaceToggle {}
    impl WidgetImpl for InterfaceToggle {}
    impl BoxImpl for InterfaceToggle {}
}

glib::wrapper! {
    pub struct InterfaceToggle(ObjectSubclass<imp::InterfaceToggle>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl InterfaceToggle {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_interface(&self, interface: Option<&kawaiifi::Interface>) {
        let content = &self.imp().interface_button_content;

        if let Some(interface) = interface {
            content.set_label(interface.name());
            content.set_icon_name(interface_icon_name(interface.bus_type()));
            self.imp().interface_button.set_sensitive(true);
        } else {
            content.set_label("No Interfaces");
            content.set_icon_name("");
            self.imp().interface_button.set_sensitive(false);
            self.imp().interface_button.set_active(false);
        }
    }

    pub fn is_active(&self) -> bool {
        self.imp().interface_button.is_active()
    }

    pub fn set_active(&self, active: bool) {
        self.imp().interface_button.set_active(active);
    }

    pub fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.imp().interface_button.connect_toggled(glib::clone!(
            #[weak(rename_to = interface_box)]
            self,
            move |_| f(&interface_box)
        ))
    }
}

fn interface_icon_name(bus_type: kawaiifi::BusType) -> &'static str {
    match bus_type {
        kawaiifi::BusType::Pci => "pci-card-symbolic",
        kawaiifi::BusType::Usb => "drive-harddisk-usb-symbolic",
        _ => "network-wireless-symbolic",
    }
}

impl Default for InterfaceToggle {
    fn default() -> Self {
        Self::new()
    }
}
