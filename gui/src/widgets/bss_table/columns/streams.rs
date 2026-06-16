use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_streams_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, |bss| {
        Some(bss.max_spatial_streams().to_string())
    })
}

pub fn create_streams_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.max_spatial_streams())
}
