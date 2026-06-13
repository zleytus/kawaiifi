use gtk::SignalListItemFactory;
use gtk::pango::{self, AttrList, FontDescription};
use gtk::prelude::*;

pub fn create_data_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::End);
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);
        label.set_yalign(1.0);
        let attrs = AttrList::new();
        let mut font_desc = FontDescription::new();
        font_desc.set_family("monospace");
        attrs.insert(pango::AttrFontDesc::new(&font_desc));
        label.set_attributes(Some(&attrs));
        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&label));
    });

    factory.connect_bind(move |_, list_item| {
        let Some((label, tree_item)) = super::label_and_tree_item(list_item) else {
            return;
        };
        label.set_label(&tree_item.bits_or_bytes());
    });

    factory
}
