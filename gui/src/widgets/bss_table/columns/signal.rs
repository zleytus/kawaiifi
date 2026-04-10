use gtk::SignalListItemFactory;
use gtk::prelude::*;

use crate::objects::BssObject;

pub fn create_signal_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::End);
        label.set_hexpand(true);
        let level_bar = gtk::LevelBar::new();
        level_bar.set_width_request(65);
        level_bar.set_height_request(5);
        level_bar.set_valign(gtk::Align::Center);

        // Remove default offsets
        level_bar.remove_offset_value(Some("low"));
        level_bar.remove_offset_value(Some("high"));
        level_bar.remove_offset_value(Some("full"));

        // Add 20 gradient levels
        // Each level spans 4 dBm (-100 to -20 = 80 dBm range / 20 levels)
        for i in 0..20 {
            let threshold = i as f64 * 0.05;
            let name = format!("signal-{:02}", i);
            level_bar.add_offset_value(&name, threshold);
        }
        level_bar.add_offset_value("signal-19", 1.0);

        let signal_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        signal_box.append(&label);
        signal_box.append(&level_bar);
        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&signal_box));
    });

    factory.connect_bind(move |_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let bss = list_item.item().and_downcast::<BssObject>().unwrap();
        let signal_box = list_item.child().and_downcast::<gtk::Box>().unwrap();
        let label = signal_box
            .first_child()
            .and_downcast::<gtk::Label>()
            .unwrap();
        let level_bar = signal_box
            .last_child()
            .and_downcast::<gtk::LevelBar>()
            .unwrap();

        let signal_fraction = ((bss.signal_strength() + 100) as f64 / 80.0).clamp(0.0, 1.0);

        level_bar.set_value(signal_fraction);
        label.set_markup(&format!(
            "{} <span alpha='50%'>dBm</span>",
            bss.signal_strength()
        ));
    });

    factory
}

pub fn create_signal_sorter() -> gtk::CustomSorter {
    gtk::CustomSorter::new(|obj1, obj2| {
        let bss1 = obj1.downcast_ref::<BssObject>().unwrap();
        let bss2 = obj2.downcast_ref::<BssObject>().unwrap();

        let sig1 = bss1.signal_strength();
        let sig2 = bss2.signal_strength();
        sig1.cmp(&sig2).into()
    })
}
