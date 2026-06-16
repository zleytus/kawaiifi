use gtk::SignalListItemFactory;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_station_count_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, None, |bss| {
        bss.station_count().map(|count| count.to_string())
    })
}

pub fn create_station_count_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.station_count())
}
