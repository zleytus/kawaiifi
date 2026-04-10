use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk::SignalListItemFactory;
use gtk::prelude::*;

use crate::objects::BssObject;

/// Creates the factory for the Last Seen column.
///
/// This column is special because it needs to track bound labels for periodic refresh.
/// The `bound_labels` parameter is shared with the refresh timer.
pub fn create_last_seen_factory(
    bound_labels: Rc<RefCell<Vec<(gtk::Label, BssObject)>>>,
) -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::End);
        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&label));
    });

    let bound_labels_bind = bound_labels.clone();
    factory.connect_bind(move |_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let bss = list_item.item().and_downcast::<BssObject>().unwrap();
        let label = list_item.child().and_downcast::<gtk::Label>().unwrap();

        // Update the label text
        update_last_seen_label(&label, &bss);

        // Track this label for periodic refresh
        bound_labels_bind.borrow_mut().push((label, bss));
    });

    let bound_labels_unbind = bound_labels;
    factory.connect_unbind(move |_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let label = list_item.child().and_downcast::<gtk::Label>().unwrap();

        // Remove this label from tracking
        bound_labels_unbind
            .borrow_mut()
            .retain(|(l, _)| l != &label);
    });

    factory
}

pub fn create_last_seen_sorter() -> gtk::CustomSorter {
    gtk::CustomSorter::new(|obj1, obj2| {
        let bss1 = obj1.downcast_ref::<BssObject>().unwrap();
        let bss2 = obj2.downcast_ref::<BssObject>().unwrap();
        // Reverse comparison so more recent (higher boottime) comes first
        bss2.time_since_last_seen()
            .cmp(&&bss1.time_since_last_seen())
            .into()
    })
}

/// Updates a Last Seen label with the current age of the BSS.
/// This is used both during bind and by the periodic refresh timer.
pub fn update_last_seen_label(label: &gtk::Label, bss: &BssObject) {
    if let Some(age) = bss.time_since_last_seen() {
        if age < Duration::from_secs(30) {
            label.set_label("Just Now");
        } else if age < Duration::from_secs(60) {
            label.set_label(&format!("{} seconds ago", age.as_secs()));
        } else if age < Duration::from_secs(3600) {
            let age_minutes = age.as_secs() / 60;
            if age_minutes == 1 {
                label.set_label("1 minute ago");
            } else {
                label.set_label(&format!("{} minutes ago", age_minutes));
            }
        } else {
            let age_hours = age.as_secs() / 3600;
            if age_hours == 1 {
                label.set_label("1 hour ago");
            } else {
                label.set_label(&format!("{} hours ago", age_hours));
            }
        }
    } else {
        label.set_label("Unknown");
    }
}
