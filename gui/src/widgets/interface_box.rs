use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use kawaiifi;

use crate::window::KawaiiFiWindow;

mod imp {
    use adw::ButtonContent;
    use gtk::{MenuButton, Widget};

    use super::*;
    use crate::widgets::InterfacePopover;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/kawaiifi/ui/interface_box.ui")]
    pub struct InterfaceBox {
        #[template_child]
        pub(crate) interface_button_label: TemplateChild<ButtonContent>,
        #[template_child]
        pub(crate) interface_popover: TemplateChild<InterfacePopover>,
        #[template_child]
        pub(crate) active_scan_spinner: TemplateChild<Widget>,
        #[template_child]
        pub(crate) interface_menu_button: TemplateChild<MenuButton>,
        pub(crate) selected_ifindex: std::cell::Cell<Option<u32>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InterfaceBox {
        const NAME: &'static str = "InterfaceBox";
        type Type = super::InterfaceBox;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for InterfaceBox {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_action();
            obj.setup_interfaces();
        }
    }
    impl WidgetImpl for InterfaceBox {}
    impl BoxImpl for InterfaceBox {}
}

glib::wrapper! {
    pub struct InterfaceBox(ObjectSubclass<imp::InterfaceBox>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl InterfaceBox {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_signal_handlers(&self, window: &KawaiiFiWindow) {
        window.connect_closure("scan-started", false, {
            let interface_box = self.clone();
            glib::closure_local!(move |_: KawaiiFiWindow| {
                interface_box.on_scan_started();
            })
        });

        window.connect_closure("scan-completed", false, {
            let interface_box = self.clone();
            glib::closure_local!(move |_: KawaiiFiWindow| {
                interface_box.on_scan_completed();
            })
        });
    }

    fn on_scan_started(&self) {
        let imp = self.imp();
        imp.active_scan_spinner.set_visible(true);
    }

    fn on_scan_completed(&self) {
        let imp = self.imp();
        imp.active_scan_spinner.set_visible(false);
    }

    fn setup_action(&self) {
        let action_group = gio::SimpleActionGroup::new();
        let initial_ifindex = kawaiifi::default_interface()
            .map(|i| i.index() as i32)
            .unwrap_or(0);

        let action = gio::SimpleAction::new_stateful(
            "select-interface",
            Some(&i32::static_variant_type()),
            &initial_ifindex.to_variant(),
        );

        let widget_weak = glib::object::WeakRef::new();
        widget_weak.set(Some(self));

        action.connect_activate(move |action, parameter| {
            let Some(widget) = widget_weak.upgrade() else {
                return;
            };

            // Get ifindex directly from parameter
            let ifindex = parameter
                .and_then(|p| p.get::<i32>())
                .expect("Parameter should be i32") as u32;

            action.set_state(&(ifindex as i32).to_variant());

            if let Some(interface) = kawaiifi::interfaces()
                .iter()
                .find(|interface| interface.index() == ifindex)
            {
                widget.set_selected_interface(interface);
            }
        });

        action_group.add_action(&action);

        self.insert_action_group("interface", Some(&action_group));
    }

    fn setup_interfaces(&self) {
        // Get available interfaces
        let interfaces = kawaiifi::interfaces();

        // Create menu
        let menu = gio::Menu::new();

        for interface in interfaces.iter() {
            let label = format!("{} ({})", interface.name(), interface.bus_type(),);
            menu.append(
                Some(&label),
                Some(&format!(
                    "interface.select-interface({})",
                    interface.index()
                )),
            );
        }

        // Set menu on MenuButton
        self.imp().interface_menu_button.set_menu_model(Some(&menu));

        // Set initial interface
        if let Some(default_interface) = kawaiifi::default_interface() {
            self.set_selected_interface(&default_interface);
        }
    }

    pub fn selected_interface_index(&self) -> Option<u32> {
        self.imp().selected_ifindex.get()
    }

    pub fn selected_interface(&self) -> Option<kawaiifi::Interface> {
        self.selected_interface_index().and_then(|index| {
            kawaiifi::interfaces()
                .into_iter()
                .find(|interface| interface.index() == index)
        })
    }

    fn set_selected_interface(&self, interface: &kawaiifi::Interface) {
        let button_label = &self.imp().interface_button_label;
        button_label.set_label(&format!("{} ({})", interface.name(), interface.bus_type()));
        match interface.bus_type() {
            kawaiifi::BusType::Pci => button_label.set_icon_name("pci-card-symbolic"),
            kawaiifi::BusType::Usb => button_label.set_icon_name("drive-harddisk-usb-symbolic"),
            _ => (),
        }

        let popover = &self.imp().interface_popover;
        popover.set_interface(interface);

        self.imp().selected_ifindex.set(Some(interface.index()))
    }
}

impl Default for InterfaceBox {
    fn default() -> Self {
        Self::new()
    }
}
