use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_channel_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, |bss| {
        Some(bss.channel_number().to_string())
    })
}

pub fn create_channel_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.channel_number())
}
