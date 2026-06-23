use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk::SignalListItemFactory;
use gtk::prelude::*;

use super::set_bss_label;
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
        label.add_css_class("numeric");
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
        compare_last_seen_ages(
            bss1.data().time_since_last_seen(),
            bss2.data().time_since_last_seen(),
        )
        .into()
    })
}

fn compare_last_seen_ages(age1: Option<Duration>, age2: Option<Duration>) -> std::cmp::Ordering {
    match (age1, age2) {
        (Some(age1), Some(age2)) => age1.cmp(&age2),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    }
}

fn last_seen_text(age: Option<Duration>) -> String {
    let Some(age) = age else {
        return "Unknown".to_string();
    };

    if age < Duration::from_secs(30) {
        "Just Now".to_string()
    } else if age < Duration::from_secs(60) {
        format!("{} seconds ago", age.as_secs())
    } else if age < Duration::from_secs(3600) {
        let age_minutes = age.as_secs() / 60;
        if age_minutes == 1 {
            "1 minute ago".to_string()
        } else {
            format!("{} minutes ago", age_minutes)
        }
    } else {
        let age_hours = age.as_secs() / 3600;
        if age_hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", age_hours)
        }
    }
}

/// Updates a Last Seen label with the current age of the BSS.
/// This is used both during bind and by the periodic refresh timer.
pub fn update_last_seen_label(label: &gtk::Label, bss: &BssObject) {
    set_bss_label(
        label,
        last_seen_text(bss.data().time_since_last_seen()),
        bss.data().is_associated(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn last_seen_text_formats_expected_ranges() {
        assert_eq!(last_seen_text(Some(Duration::from_secs(0))), "Just Now");
        assert_eq!(last_seen_text(Some(Duration::from_secs(29))), "Just Now");
        assert_eq!(
            last_seen_text(Some(Duration::from_secs(30))),
            "30 seconds ago"
        );
        assert_eq!(
            last_seen_text(Some(Duration::from_secs(60))),
            "1 minute ago"
        );
        assert_eq!(
            last_seen_text(Some(Duration::from_secs(120))),
            "2 minutes ago"
        );
        assert_eq!(
            last_seen_text(Some(Duration::from_secs(3600))),
            "1 hour ago"
        );
        assert_eq!(
            last_seen_text(Some(Duration::from_secs(7200))),
            "2 hours ago"
        );
        assert_eq!(last_seen_text(None), "Unknown");
    }

    #[test]
    fn compare_last_seen_ages_sorts_recent_before_old_and_unknown_last() {
        assert_eq!(
            compare_last_seen_ages(Some(Duration::from_secs(5)), Some(Duration::from_secs(10))),
            Ordering::Less
        );
        assert_eq!(
            compare_last_seen_ages(Some(Duration::from_secs(10)), Some(Duration::from_secs(5))),
            Ordering::Greater
        );
        assert_eq!(
            compare_last_seen_ages(Some(Duration::from_secs(5)), None),
            Ordering::Less
        );
        assert_eq!(
            compare_last_seen_ages(None, Some(Duration::from_secs(5))),
            Ordering::Greater
        );
        assert_eq!(compare_last_seen_ages(None, None), Ordering::Equal);
    }
}
