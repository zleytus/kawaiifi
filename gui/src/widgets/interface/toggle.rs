use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use std::sync::OnceLock;

    use adw::ButtonContent;
    use gtk::ToggleButton;

    use super::*;

    pub(super) const PROP_ACTIVE: &str = "active";

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/interface_toggle.ui")]
    pub struct InterfaceToggle {
        // UI components
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

    impl ObjectImpl for InterfaceToggle {
        fn constructed(&self) {
            self.parent_constructed();

            self.interface_button.connect_active_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self.obj(),
                move |_| {
                    obj.notify(PROP_ACTIVE);
                }
            ));
        }

        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
            PROPERTIES.get_or_init(|| {
                vec![
                    glib::ParamSpecBoolean::builder(PROP_ACTIVE)
                        .default_value(false)
                        .explicit_notify()
                        .build(),
                ]
            })
        }

        fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                PROP_ACTIVE => self.interface_button.is_active().to_value(),
                name => unimplemented!("Unknown property {name}"),
            }
        }

        fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.name() {
                PROP_ACTIVE => {
                    self.interface_button
                        .set_active(value.get::<bool>().unwrap());
                }
                name => unimplemented!("Unknown property {name}"),
            }
        }
    }
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
            content.set_icon_name(super::icon_name(interface.bus_type()));
            self.imp().interface_button.set_sensitive(true);
        } else {
            content.set_label("No Interfaces");
            content.set_icon_name("");
            self.imp().interface_button.set_sensitive(false);
            self.imp().interface_button.set_active(false);
        }
    }
}

impl Default for InterfaceToggle {
    fn default() -> Self {
        Self::new()
    }
}
