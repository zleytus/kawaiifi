use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_uptime_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, None, |bss| Some(bss.formatted_uptime()))
}

pub fn create_uptime_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.uptime())
}
