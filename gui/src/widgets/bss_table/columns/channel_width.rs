use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_channel_width_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, None, |bss| {
        Some(bss.data().channel_width().to_string())
    })
}

pub fn create_channel_width_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.data().channel_width())
}
