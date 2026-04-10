use gtk::SignalListItemFactory;
use gtk::prelude::*;

use crate::objects::BssObject;

pub fn create_channel_utilization_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::End);
        label.set_width_chars(4);
        let level_bar = gtk::LevelBar::new();
        level_bar.set_hexpand(true);
        level_bar.set_height_request(5);
        level_bar.set_valign(gtk::Align::Center);

        // Remove default offsets
        level_bar.remove_offset_value(Some("low"));
        level_bar.remove_offset_value(Some("high"));
        level_bar.remove_offset_value(Some("full"));
        // Add 20 gradient levels spanning 0.0–1.0
        for i in 0..20 {
            let threshold = i as f64 * 0.05;
            let name = format!("channel-utilization-{:02}", i);
            level_bar.add_offset_value(&name, threshold);
        }
        level_bar.add_offset_value("channel-utilization-19", 1.0);

        let channel_utilization_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        channel_utilization_box.append(&label);
        channel_utilization_box.append(&level_bar);
        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&channel_utilization_box));
    });

    factory.connect_bind(move |_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let bss = list_item.item().and_downcast::<BssObject>().unwrap();
        let channel_utilization_box = list_item.child().and_downcast::<gtk::Box>().unwrap();
        let label = channel_utilization_box
            .first_child()
            .and_downcast::<gtk::Label>()
            .unwrap();
        let level_bar = channel_utilization_box
            .last_child()
            .and_downcast::<gtk::LevelBar>()
            .unwrap();

        match bss.channel_utilization() {
            Some(utilization) => {
                let fraction = (utilization as f64 / 255.0).max(0.08);
                let percent = (utilization as f64 / 255.0 * 100.0).round() as u32;
                level_bar.set_value(fraction);
                label.set_markup(&format!("{}<span alpha='50%'>%</span>", percent));
                label.set_visible(true);
                level_bar.set_visible(true);
            }
            None => {
                label.set_visible(false);
                level_bar.set_visible(false);
            }
        }
    });

    factory
}

pub fn create_channel_utilization_sorter() -> gtk::CustomSorter {
    gtk::CustomSorter::new(|obj1, obj2| {
        let bss1 = obj1.downcast_ref::<BssObject>().unwrap();
        let bss2 = obj2.downcast_ref::<BssObject>().unwrap();

        let cu1 = bss1.channel_utilization();
        let cu2 = bss2.channel_utilization();
        cu1.cmp(&cu2).into()
    })
}
