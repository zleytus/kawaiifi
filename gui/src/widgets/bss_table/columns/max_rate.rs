use gtk::SignalListItemFactory;
use gtk::prelude::*;

use super::create_bss_text_factory;
use crate::objects::BssObject;

pub fn create_max_rate_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, |bss| {
        let rate = format!("{:.1}", bss.max_rate())
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string();
        Some(format!("{rate} Mbps"))
    })
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
