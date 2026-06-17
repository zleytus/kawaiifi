use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

#[derive(Default)]
enum InterfaceState {
    #[default]
    Uninitialized,
    Loaded(Vec<kawaiifi::Interface>),
    Error(kawaiifi::InterfaceError),
}

mod imp {
    use std::{
        cell::{Cell, RefCell},
        sync::OnceLock,
    };

    use adw::ButtonContent;
    use gtk::{MenuButton, glib::types::StaticType};

    use super::*;
    use crate::widgets::InterfacePopover;

    pub const SIGNAL_INTERFACES_LOAD_FAILED: &str = "interfaces-load-failed";

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/interface_box.ui")]
    pub struct InterfaceBox {
        #[template_child]
        pub(crate) interface_button_label: TemplateChild<ButtonContent>,
        #[template_child]
        pub(crate) interface_popover: TemplateChild<InterfacePopover>,
        #[template_child]
        pub(crate) interface_menu_button: TemplateChild<MenuButton>,
        pub(crate) selected_ifindex: Cell<Option<u32>>,
        pub(super) interface_state: RefCell<InterfaceState>,
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
            obj.setup_interfaces();
            obj.setup_action();
        }

        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder(SIGNAL_INTERFACES_LOAD_FAILED)
                        .param_types([String::static_type()])
                        .build(),
                ]
            })
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

    fn setup_action(&self) {
        let action_group = gio::SimpleActionGroup::new();
        let initial_ifindex = self.selected_interface_index().unwrap_or_default() as i32;

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

            if let Some(interface) = widget.interface_by_index(ifindex) {
                widget.set_selected_interface(&interface);
            }
        });

        action_group.add_action(&action);

        self.insert_action_group("interface", Some(&action_group));
    }

    fn setup_interfaces(&self) {
        let menu = gio::Menu::new();

        match kawaiifi::interfaces() {
            Ok(interfaces) => {
                for interface in &interfaces {
                    let label = format!("{} ({})", interface.name(), interface.bus_type());
                    menu.append(
                        Some(&label),
                        Some(&format!(
                            "interface.select-interface({})",
                            interface.index()
                        )),
                    );
                }

                let default_interface = interfaces.first().cloned();
                self.imp()
                    .interface_state
                    .replace(InterfaceState::Loaded(interfaces));

                if let Some(default_interface) = default_interface {
                    self.set_selected_interface(&default_interface);
                    self.imp().interface_menu_button.set_sensitive(true);
                } else {
                    self.imp().interface_button_label.set_label("No Interfaces");
                    self.imp().interface_button_label.set_icon_name("");
                    self.imp().interface_menu_button.set_sensitive(false);
                }
            }
            Err(error) => {
                tracing::error!(error = %error, "Failed to enumerate Wi-Fi interfaces");
                self.imp()
                    .interface_state
                    .replace(InterfaceState::Error(error));
                self.imp()
                    .interface_button_label
                    .set_label("Interfaces Unavailable");
                self.imp().interface_button_label.set_icon_name("");
                self.imp().interface_menu_button.set_sensitive(false);

                glib::idle_add_local_once(glib::clone!(
                    #[weak(rename_to = interface_box)]
                    self,
                    move || {
                        let Some(error) = interface_box.interface_error() else {
                            return;
                        };
                        interface_box
                            .emit_by_name::<()>(imp::SIGNAL_INTERFACES_LOAD_FAILED, &[&error]);
                    }
                ));
            }
        }

        self.imp().interface_menu_button.set_menu_model(Some(&menu));
    }

    pub fn selected_interface_index(&self) -> Option<u32> {
        self.imp().selected_ifindex.get()
    }

    pub fn selected_interface(&self) -> Option<kawaiifi::Interface> {
        self.selected_interface_index()
            .and_then(|index| self.interface_by_index(index))
    }

    fn interface_by_index(&self, index: u32) -> Option<kawaiifi::Interface> {
        match &*self.imp().interface_state.borrow() {
            InterfaceState::Loaded(interfaces) => interfaces
                .iter()
                .find(|interface| interface.index() == index)
                .cloned(),
            InterfaceState::Uninitialized | InterfaceState::Error(_) => None,
        }
    }

    fn interface_error(&self) -> Option<String> {
        match &*self.imp().interface_state.borrow() {
            InterfaceState::Error(error) => Some(error.to_string()),
            InterfaceState::Uninitialized | InterfaceState::Loaded(_) => None,
        }
    }

    pub fn connect_interfaces_load_failed<F: Fn(&Self, &str) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local(imp::SIGNAL_INTERFACES_LOAD_FAILED, false, move |args| {
            let interface_box = args[0].get::<Self>().unwrap();
            let error = args[1].get::<String>().unwrap();
            f(&interface_box, &error);
            None
        })
    }

    fn set_selected_interface(&self, interface: &kawaiifi::Interface) {
        let button_label = &self.imp().interface_button_label;
        button_label.set_label(&format!("{} ({})", interface.name(), interface.bus_type()));
        match interface.bus_type() {
            kawaiifi::BusType::Pci => button_label.set_icon_name("pci-card-symbolic"),
            kawaiifi::BusType::Usb => button_label.set_icon_name("drive-harddisk-usb-symbolic"),
            _ => button_label.set_icon_name(""),
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
