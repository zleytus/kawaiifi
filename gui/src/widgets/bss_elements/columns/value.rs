use gtk::SignalListItemFactory;
use gtk::prelude::*;

use crate::objects::IeTreeItem;

pub fn create_value_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::End);
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);
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
        label.set_markup(&tree_item.value_with_markup());
    });

    factory
}
