mod amendments;
mod band;
mod bssid;
mod channel;
mod channel_utilization;
mod channel_width;
mod color;
mod frequency;
mod last_seen;
mod max_rate;
mod protocols;
mod security;
mod signal;
mod ssid;
mod station_count;
mod streams;
mod uptime;
mod vendor;

use gtk::SignalListItemFactory;
use gtk::prelude::*;

use crate::objects::BssObject;

pub(super) fn set_bss_label(label: &gtk::Label, text: impl AsRef<str>, associated: bool) {
    let text = text.as_ref();
    if associated {
        label.set_markup(&associated_markup(text));
    } else {
        label.set_label(text);
    }
}

pub(super) fn create_bss_text_factory<F>(
    halign: gtk::Align,
    ellipsize: Option<gtk::pango::EllipsizeMode>,
    text_for_bss: F,
) -> SignalListItemFactory
where
    F: Fn(&BssObject) -> Option<String> + 'static,
{
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(halign);
        if let Some(ellipsize) = ellipsize {
            label.set_ellipsize(ellipsize);
        }
        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&label));
    });

    factory.connect_bind(move |_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let bss = list_item.item().and_downcast::<BssObject>().unwrap();
        let label = list_item.child().and_downcast::<gtk::Label>().unwrap();

        if let Some(text) = text_for_bss(&bss) {
            label.set_visible(true);
            set_bss_label(&label, text, bss.is_associated());
        } else {
            label.set_label("");
            label.set_visible(false);
        }
    });

    factory
}

pub(super) fn create_bss_sorter_by<K, F>(key_for_bss: F) -> gtk::CustomSorter
where
    K: Ord + 'static,
    F: Fn(&BssObject) -> K + 'static,
{
    gtk::CustomSorter::new(move |obj1, obj2| {
        let bss1 = obj1.downcast_ref::<BssObject>().unwrap();
        let bss2 = obj2.downcast_ref::<BssObject>().unwrap();
        key_for_bss(bss1).cmp(&key_for_bss(bss2)).into()
    })
}

fn associated_markup(text: &str) -> String {
    let escaped = gtk::glib::markup_escape_text(text);
    format!("<b><i>{escaped}</i></b>")
}

pub use amendments::*;
pub use band::*;
pub use bssid::*;
pub use channel::*;
pub use channel_utilization::*;
pub use channel_width::*;
pub use color::*;
pub use frequency::*;
pub use last_seen::*;
pub use max_rate::*;
pub use protocols::*;
pub use security::*;
pub use signal::*;
pub use ssid::*;
pub use station_count::*;
pub use streams::*;
pub use uptime::*;
pub use vendor::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn associated_markup_escapes_text() {
        assert_eq!(
            associated_markup("Cafe <WiFi> & Guests"),
            "<b><i>Cafe &lt;WiFi&gt; &amp; Guests</i></b>"
        );
    }
}
