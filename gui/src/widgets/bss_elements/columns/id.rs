use gtk::SignalListItemFactory;
use gtk::prelude::*;

use crate::objects::IeTreeItem;

pub fn create_id_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.add_css_class("numeric");
        label.set_halign(gtk::Align::End);

        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&label));
    });

    factory.connect_bind(move |_, list_item| {
        let Some((label, tree_item)) = super::label_and_tree_item(list_item) else {
            return;
        };

        // Only show ID for IEs, leave empty for fields
        if let Some(ie) = tree_item.as_ie() {
            label.set_label(&ie.id().to_string());
        } else {
            label.set_label("");
        }
    });

    factory
}

pub fn create_id_sorter() -> gtk::CustomSorter {
    gtk::CustomSorter::new(|obj1, obj2| {
        let item1 = obj1.downcast_ref::<IeTreeItem>().unwrap();
        let item2 = obj2.downcast_ref::<IeTreeItem>().unwrap();

        let id1 = item1.as_ie().map(|ie| ie.id()).unwrap_or(u8::MAX);
        let id2 = item2.as_ie().map(|ie| ie.id()).unwrap_or(u8::MAX);

        id1.cmp(&id2).into()
    })
}
