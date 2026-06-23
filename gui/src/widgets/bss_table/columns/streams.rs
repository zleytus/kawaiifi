use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_streams_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, None, &["numeric"], |bss| {
        Some(bss.data().max_spatial_streams().to_string())
    })
}

pub fn create_streams_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.data().max_spatial_streams())
}
