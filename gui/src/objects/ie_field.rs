use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct IeFieldObject {
        pub(super) field: RefCell<Option<kawaiifi::ies::Field>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IeFieldObject {
        const NAME: &'static str = "IeFieldObject";
        type Type = super::IeFieldObject;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for IeFieldObject {}
}

glib::wrapper! {
    pub struct IeFieldObject(ObjectSubclass<imp::IeFieldObject>);
}

impl IeFieldObject {
    pub fn new(field: kawaiifi::ies::Field) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().field.replace(Some(field));
        obj
    }

    /// Get a reference to the underlying Field
    fn field(&self) -> std::cell::Ref<'_, kawaiifi::ies::Field> {
        std::cell::Ref::map(self.imp().field.borrow(), |opt| {
            opt.as_ref()
                .expect("IeFieldObject not properly initialized")
        })
    }

    pub fn name(&self) -> String {
        self.field().title().to_string()
    }

    pub fn value(&self) -> String {
        self.field().value().replace('\0', "�")
    }

    pub fn byte(&self) -> Option<u8> {
        self.field().byte()
    }

    pub fn bytes(&self) -> Option<Vec<u8>> {
        self.field().bytes().map(|bytes| bytes.to_vec())
    }

    pub fn bits(&self) -> Option<String> {
        self.field().bits()
    }

    pub fn subfields(&self) -> Vec<Self> {
        self.field()
            .subfields()
            .iter()
            .map(|f| IeFieldObject::new(f.clone()))
            .collect()
    }

    pub fn units(&self) -> Option<String> {
        self.field().units().map(|units| units.to_string())
    }
}
