use gtk::SignalListItemFactory;
use gtk::prelude::*;

use crate::objects::BssObject;

pub fn create_max_rate_factory() -> SignalListItemFactory {
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
        let bss = list_item.item().and_downcast::<BssObject>().unwrap();
        let label = list_item.child().and_downcast::<gtk::Label>().unwrap();
        label.set_markup(
            &(format!("{:.1} <span alpha='50%'>Mbps</span>", bss.max_rate())
                .trim_end_matches("0")
                .trim_end_matches(".")),
        );
    });

    factory
}

pub fn create_max_rate_sorter() -> gtk::CustomSorter {
    gtk::CustomSorter::new(|obj1, obj2| {
        let bss1 = obj1.downcast_ref::<BssObject>().unwrap();
        let bss2 = obj2.downcast_ref::<BssObject>().unwrap();
        bss1.max_rate()
            .partial_cmp(&bss2.max_rate())
            .unwrap_or(std::cmp::Ordering::Equal)
            .into()
    })
}
