use std::cell::RefCell;

use gtk::glib;
use gtk::subclass::prelude::*;
use kawaiifi::{Ie, ies::Field};

use crate::objects::IeFieldObject;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct IeObject {
        pub(super) ie: RefCell<Option<Ie>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IeObject {
        const NAME: &'static str = "IeObject";
        type Type = super::IeObject;
    }

    impl ObjectImpl for IeObject {}
}

glib::wrapper! {
    pub struct IeObject(ObjectSubclass<imp::IeObject>);
}

impl IeObject {
    pub fn new(ie: Ie) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().ie.replace(Some(ie));

        obj
    }

    /// Get a reference to the underlying Ie
    fn ie(&self) -> std::cell::Ref<'_, Ie> {
        std::cell::Ref::map(self.imp().ie.borrow(), |opt| {
            opt.as_ref().expect("IeObject not properly initialized")
        })
    }

    pub fn id(&self) -> u8 {
        self.ie().id
    }

    pub fn id_ext(&self) -> Option<u8> {
        self.ie().id_ext
    }

    pub fn length(&self) -> u8 {
        self.ie().len
    }

    pub fn ie_name(&self) -> &'static str {
        self.ie().name()
    }

    pub fn summary(&self) -> String {
        self.ie().summary().replace('\0', "�")
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.ie().bytes()
    }

    pub fn fields(&self) -> Vec<IeFieldObject> {
        let mut fields: Vec<IeFieldObject> = [
            Field::builder()
                .title("ID")
                .value(self.id())
                .byte(self.id())
                .build(),
            Field::builder()
                .title("Length")
                .value(self.length())
                .units(if self.length() == 1 { "byte" } else { "bytes" })
                .byte(self.length())
                .build(),
        ]
        .into_iter()
        .map(IeFieldObject::new)
        .chain(self.ie().fields().into_iter().map(IeFieldObject::new))
        .collect();

        if let Some(id_ext) = self.id_ext() {
            fields.insert(
                2,
                IeFieldObject::new(
                    Field::builder()
                        .title("ID (Extension)")
                        .value(id_ext)
                        .byte(id_ext)
                        .build(),
                ),
            );
        }

        fields
    }
}
