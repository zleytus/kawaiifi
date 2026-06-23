use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_band_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, None, |bss| {
        Some(bss.data().band().to_string())
    })
}

pub fn create_band_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.data().band())
}
