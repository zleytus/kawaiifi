use gtk::SignalListItemFactory;
use gtk::prelude::*;

use crate::objects::IeTreeItem;

pub fn create_id_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::End);

        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&label));
    });

    factory.connect_bind(move |_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let Some(row) = list_item.item().and_downcast::<gtk::TreeListRow>() else {
            return;
        };
        let Some(tree_item) = row.item().and_downcast::<IeTreeItem>() else {
            return;
        };
        let label = list_item.child().and_downcast::<gtk::Label>().unwrap();

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
