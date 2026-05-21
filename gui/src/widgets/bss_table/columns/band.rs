use gtk::SignalListItemFactory;
use kawaiifi::Band;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_band_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, |bss| {
        let band = match bss.band() {
            Band::TwoPointFourGhz => "2.4",
            Band::FiveGhz => "5",
            Band::SixGhz => "6",
        };
        Some(format!("{band} GHz"))
    })
}

pub fn create_band_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.band())
}
