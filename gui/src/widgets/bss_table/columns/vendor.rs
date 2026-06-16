use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_vendor_factory() -> SignalListItemFactory {
    create_bss_text_factory(
        gtk::Align::Start,
        Some(gtk::pango::EllipsizeMode::End),
        |bss| Some(bss.vendor()),
    )
}

pub fn create_vendor_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.vendor())
}
