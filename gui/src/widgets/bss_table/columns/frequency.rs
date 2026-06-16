use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_frequency_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, None, |bss| {
        Some(format!("{} MHz", bss.frequency_mhz()))
    })
}

pub fn create_frequency_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.frequency_mhz())
}
