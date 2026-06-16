use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_amendments_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::Start, None, |bss| {
        Some(bss.amendments().to_string())
    })
}

pub fn create_amendments_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.amendments())
}
