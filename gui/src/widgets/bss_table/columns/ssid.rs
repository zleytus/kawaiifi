use gtk::SignalListItemFactory;
use gtk::prelude::*;

use crate::objects::BssObject;

pub fn create_ssid_factory() -> SignalListItemFactory {
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
        if let Some(ssid) = bss.ssid() {
            label.set_label(&ssid);
        } else {
            label.set_markup("<i><span alpha='50%'>Hidden</span></i>");
        }
    });

    factory
}

pub fn create_ssid_sorter() -> gtk::CustomSorter {
    gtk::CustomSorter::new(|obj1, obj2| {
        let bss1 = obj1.downcast_ref::<BssObject>().unwrap();
        let bss2 = obj2.downcast_ref::<BssObject>().unwrap();

        let ssid1 = bss1.ssid().unwrap_or_default().to_ascii_lowercase();
        let ssid2 = bss2.ssid().unwrap_or_default().to_ascii_lowercase();
        ssid1.cmp(&ssid2).into()
    })
}
