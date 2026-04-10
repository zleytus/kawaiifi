use gtk::SignalListItemFactory;
use gtk::prelude::*;
use kawaiifi::Band;

use crate::objects::BssObject;

pub fn create_band_factory() -> SignalListItemFactory {
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
        label.set_markup(&format!(
            "{} <span alpha='50%'>GHz</span>",
            match bss.band() {
                Band::TwoPointFourGhz => "2.4",
                Band::FiveGhz => "5",
                Band::SixGhz => "6",
            }
        ));
    });

    factory
}

pub fn create_band_sorter() -> gtk::CustomSorter {
    gtk::CustomSorter::new(|obj1, obj2| {
        let bss1 = obj1.downcast_ref::<BssObject>().unwrap();
        let bss2 = obj2.downcast_ref::<BssObject>().unwrap();
        bss1.band().cmp(&bss2.band()).into()
    })
}
