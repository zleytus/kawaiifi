use gtk::SignalListItemFactory;
use gtk::prelude::*;

use crate::objects::IeTreeItem;

pub fn create_name_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::Start);
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);

        let expander = gtk::TreeExpander::new();
        expander.set_child(Some(&label));

        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&expander));
    });

    factory.connect_bind(move |_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let Some(row) = list_item.item().and_downcast::<gtk::TreeListRow>() else {
            return;
        };
        let Some(tree_item) = row.item().and_downcast::<IeTreeItem>() else {
            return;
        };

        let expander = list_item
            .child()
            .and_downcast::<gtk::TreeExpander>()
            .unwrap();
        expander.set_list_row(Some(&row));

        let label = expander.child().and_downcast::<gtk::Label>().unwrap();
        label.set_label(&tree_item.name());
    });

    factory
}

pub fn create_name_sorter() -> gtk::CustomSorter {
    gtk::CustomSorter::new(|obj1, obj2| {
        let item1 = obj1.downcast_ref::<IeTreeItem>().unwrap();
        let item2 = obj2.downcast_ref::<IeTreeItem>().unwrap();

        item1.name().cmp(&item2.name()).into()
    })
}
