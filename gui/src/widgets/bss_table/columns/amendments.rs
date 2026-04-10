use gtk::SignalListItemFactory;
use gtk::prelude::*;

use super::set_bss_label;
use crate::objects::BssObject;

pub fn create_amendments_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::Start);
        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&label));
    });

    factory.connect_bind(move |_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let bss = list_item.item().and_downcast::<BssObject>().unwrap();
        let label = list_item.child().and_downcast::<gtk::Label>().unwrap();
        set_bss_label(&label, bss.amendments().to_string(), bss.is_associated());
    });

    factory
}

pub fn create_amendments_sorter() -> gtk::CustomSorter {
    gtk::CustomSorter::new(|obj1, obj2| {
        let bss1 = obj1.downcast_ref::<BssObject>().unwrap();
        let bss2 = obj2.downcast_ref::<BssObject>().unwrap();
        bss1.amendments().cmp(&bss2.amendments()).into()
    })
}
