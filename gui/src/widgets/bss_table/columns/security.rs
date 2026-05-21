use gtk::SignalListItemFactory;
use gtk::prelude::*;

use super::create_bss_text_factory;
use crate::objects::BssObject;

pub fn create_security_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::Start, |bss| Some(bss.security().to_string()))
}

pub fn create_security_sorter() -> gtk::CustomSorter {
    gtk::CustomSorter::new(|obj1, obj2| {
        let bss1 = obj1.downcast_ref::<BssObject>().unwrap();
        let bss2 = obj2.downcast_ref::<BssObject>().unwrap();
        bss1.security().cmp(&bss2.security()).into()
    })
}
