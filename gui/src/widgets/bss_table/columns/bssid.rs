use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_bssid_factory() -> SignalListItemFactory {
    create_bss_text_factory(
        gtk::Align::Start,
        Some(gtk::pango::EllipsizeMode::Middle),
        &["numeric"],
        |bss| Some(bss.data().formatted_bssid()),
    )
}

pub fn create_bssid_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.data().formatted_bssid())
}
