use gtk::SignalListItemFactory;
use kawaiifi::ChannelWidth;

use super::{create_bss_sorter_by, create_bss_text_factory};

pub fn create_channel_width_factory() -> SignalListItemFactory {
    create_bss_text_factory(gtk::Align::End, |bss| {
        let width = match bss.channel_width() {
            ChannelWidth::TwentyMhz => "20",
            ChannelWidth::FortyMhz => "40",
            ChannelWidth::EightyMhz => "80",
            ChannelWidth::EightyPlusEightyMhz => "80+80",
            ChannelWidth::OneSixtyMhz => "160",
            ChannelWidth::ThreeHundredTwentyMhz => "320",
        };
        Some(format!("{width} MHz"))
    })
}

pub fn create_channel_width_sorter() -> gtk::CustomSorter {
    create_bss_sorter_by(|bss| bss.channel_width())
}
