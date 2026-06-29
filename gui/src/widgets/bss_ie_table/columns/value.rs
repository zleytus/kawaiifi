use gtk::SignalListItemFactory;
use gtk::prelude::*;

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
        let Some((label, tree_item)) = super::label_and_tree_item(list_item) else {
            return;
        };
        label.set_markup(&tree_item.value_with_markup());
    });

    factory
}
