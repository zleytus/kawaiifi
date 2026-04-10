use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct IeTreeItem {
        pub(super) kind: RefCell<ItemKind>,
    }

    #[derive(Clone)]
    pub(super) enum ItemKind {
        Ie(super::super::IeObject),
        Field(super::super::IeFieldObject),
    }

    impl Default for ItemKind {
        fn default() -> Self {
            // Dummy default, should never actually be used
            Self::Field(super::super::IeFieldObject::new(kawaiifi::ies::Field::new(
                "", "",
            )))
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IeTreeItem {
        const NAME: &'static str = "IeTreeItem";
        type Type = super::IeTreeItem;
    }

    impl ObjectImpl for IeTreeItem {}
}

glib::wrapper! {
    pub struct IeTreeItem(ObjectSubclass<imp::IeTreeItem>);
}

impl IeTreeItem {
    pub fn from_ie(ie: super::IeObject) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().kind.replace(imp::ItemKind::Ie(ie));
        obj
    }

    pub fn from_field(field: super::IeFieldObject) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().kind.replace(imp::ItemKind::Field(field));
        obj
    }

    pub fn name(&self) -> String {
        match &*self.imp().kind.borrow() {
            imp::ItemKind::Ie(ie) => ie.ie_name().to_string(),
            imp::ItemKind::Field(field) => field.name(),
        }
    }

    pub fn value_with_markup(&self) -> String {
        match &*self.imp().kind.borrow() {
            imp::ItemKind::Ie(ie) => {
                format!(
                    "<span alpha='75%' style='oblique'>{}</span>",
                    glib::markup_escape_text(&ie.summary())
                )
            }
            imp::ItemKind::Field(field) => {
                if let Some(units) = field.units() {
                    // Use Pango markup to make units muted
                    format!(
                        "{} <span alpha='50%'>{}</span>",
                        glib::markup_escape_text(&field.value()),
                        glib::markup_escape_text(&units)
                    )
                } else {
                    field.value()
                }
            }
        }
    }

    pub fn bits_or_bytes(&self) -> String {
        match &*self.imp().kind.borrow() {
            imp::ItemKind::Ie(ie) => const_hex::encode_upper_prefixed(ie.bytes()),
            imp::ItemKind::Field(field) => {
                if let Some(byte) = field.byte() {
                    format!("{:#04X}", byte)
                } else if let Some(bytes) = field.bytes() {
                    const_hex::encode_upper_prefixed(bytes)
                } else if let Some(bits) = field.bits() {
                    bits
                } else {
                    "".to_string()
                }
            }
        }
    }

    /// Generate children for this tree item
    pub fn children(&self) -> Vec<Self> {
        match &*self.imp().kind.borrow() {
            imp::ItemKind::Ie(ie) => {
                // Get fields from the IE and wrap them
                ie.fields()
                    .into_iter()
                    .map(|field_obj| Self::from_field(field_obj))
                    .collect()
            }
            imp::ItemKind::Field(field) => {
                // Get subfields and wrap them
                field
                    .subfields()
                    .into_iter()
                    .map(|field_obj| Self::from_field(field_obj))
                    .collect()
            }
        }
    }

    /// Get the underlying IeObject if this is an IE (not a field)
    pub fn as_ie(&self) -> Option<super::IeObject> {
        match &*self.imp().kind.borrow() {
            imp::ItemKind::Ie(ie) => Some(ie.clone()),
            imp::ItemKind::Field(_) => None,
        }
    }
}
