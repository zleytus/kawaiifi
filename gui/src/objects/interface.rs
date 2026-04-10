use std::cell::RefCell;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use kawaiifi::Interface;

mod imp {
    use super::*;

    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::InterfaceObject)]
    pub struct InterfaceObject {
        #[property(get, set)]
        pub display_name: RefCell<String>,
        #[property(get, set)]
        pub name: RefCell<String>,
        #[property(get, set)]
        pub index: RefCell<u32>,
    }

    impl Default for InterfaceObject {
        fn default() -> Self {
            Self {
                display_name: RefCell::default(),
                name: RefCell::default(),
                index: RefCell::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InterfaceObject {
        const NAME: &'static str = "InterfaceObject";
        type Type = super::InterfaceObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for InterfaceObject {}
}

glib::wrapper! {
    pub struct InterfaceObject(ObjectSubclass<imp::InterfaceObject>);
}

impl InterfaceObject {
    pub fn new(interface: &Interface) -> Self {
        // Build a friendly display name
        let display_name = interface
            .vendor_name()
            .unwrap_or_else(|| interface.driver().unwrap_or_default());

        glib::Object::builder()
            .property("display-name", display_name)
            .property("name", interface.name())
            .property("index", interface.index())
            .build()
    }
}
