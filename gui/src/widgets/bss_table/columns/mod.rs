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
mod uptime;
mod vendor;

pub(super) fn set_bss_label(label: &gtk::Label, text: impl AsRef<str>, associated: bool) {
    let text = text.as_ref();
    if associated {
        label.set_markup(&associated_markup(text));
    } else {
        label.set_label(text);
    }
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
