use std::{cell::RefCell, ops::Deref, time::Duration};

use gtk::gdk::RGBA;
use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    pub struct BssObject {
        pub(super) bss: RefCell<Option<BssInternal>>,
    }

    impl Default for BssObject {
        fn default() -> Self {
            Self {
                bss: RefCell::new(None),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BssObject {
        const NAME: &'static str = "BssObject";
        type Type = super::BssObject;
    }

    impl ObjectImpl for BssObject {}
}

glib::wrapper! {
    pub struct BssObject(ObjectSubclass<imp::BssObject>);
}

impl BssObject {
    /// Creates a new `BssObject` wrapping the given [`BssInternal`].
    pub fn new(bss: BssInternal) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().bss.replace(Some(bss));

        obj
    }

    /// Returns a reference to the underlying [`BssInternal`].
    pub fn data(&self) -> std::cell::Ref<'_, BssInternal> {
        std::cell::Ref::map(self.imp().bss.borrow(), |opt| {
            opt.as_ref().expect("BssObject not properly initialized")
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BssInternal {
    bss: kawaiifi::Bss,
    vendor: Option<String>,
    color: RGBA,
}

impl BssInternal {
    /// Creates a new `BssInternal` wrapping the given [`kawaiifi::Bss`].
    pub fn new(bss: kawaiifi::Bss) -> Self {
        let vendor = crate::vendor::lookup_vendor(&bss);
        Self {
            color: color_from_bssid(bss.bssid()),
            bss,
            vendor,
        }
    }

    /// Replaces the underlying [`kawaiifi::Bss`] with a newer scan result.
    pub fn update(&mut self, bss: kawaiifi::Bss) {
        self.bss = bss;
    }

    /// Returns true if the machine is associated with the BSS.
    pub fn is_associated(&self) -> bool {
        matches!(self.status(), Some(kawaiifi::BssStatus::Associated))
    }

    /// The SSID, or `None` for hidden networks.
    pub fn formatted_ssid(&self) -> Option<String> {
        self.ssid().map(|s| s.replace('\0', "�"))
    }

    /// The BSSID formatted as a colon-separated hex string (e.g. `AA:BB:CC:DD:EE:FF`).
    pub fn formatted_bssid(&self) -> String {
        crate::mac::format_mac(self.bssid())
    }

    /// The uptime formatted as a human-readable string (e.g. `2d 3h 45m`).
    pub fn formatted_uptime(&self) -> String {
        formatted_uptime_text(self.uptime())
    }

    /// The OUI vendor name, or an empty string if unknown.
    pub fn formatted_vendor(&self) -> String {
        self.vendor()
            .map(format_vendor_display_name)
            .unwrap_or_default()
    }

    /// The OUI vendor name, or `None` if unknown.
    pub fn vendor(&self) -> Option<&str> {
        self.vendor.as_deref()
    }

    /// Sets the OUI vendor name.
    pub fn set_vendor(&mut self, vendor: String) {
        self.vendor.replace(vendor);
    }

    /// The display color derived from the BSSID.
    pub fn color(&self) -> RGBA {
        self.color
    }

    /// How long ago this BSS was last seen, or `None` if the timestamp is unavailable.
    pub fn time_since_last_seen(&self) -> Option<Duration> {
        self.last_seen_utc()
            .and_then(|utc| chrono::Utc::now().signed_duration_since(utc).to_std().ok())
    }
}

impl Deref for BssInternal {
    type Target = kawaiifi::Bss;

    fn deref(&self) -> &Self::Target {
        &self.bss
    }
}

fn formatted_uptime_text(uptime: Duration) -> String {
    let secs = uptime.as_secs();
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;

    if days >= 365 {
        let years = days / 365;
        let remaining_days = days % 365;
        format!("{years}y {remaining_days}d {hours}h {mins}m")
    } else if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else if hours > 0 {
        format!("{hours}h {mins}m")
    } else {
        format!("{mins}m")
    }
}

fn format_vendor_display_name(vendor: &str) -> String {
    const MAX_UNCHANGED_LEN: usize = 20;

    if vendor.chars().count() <= MAX_UNCHANGED_LEN {
        return vendor.to_string();
    }

    let mut words = vendor
        .split_whitespace()
        .map(|word| word.trim().trim_end_matches([',', '-']).trim_end())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();

    while words.len() > 2 && words.join(" ").chars().count() > MAX_UNCHANGED_LEN {
        words.pop();
    }

    words.join(" ")
}

fn color_from_bssid(bssid: &[u8; 6]) -> RGBA {
    // Use last 3 bytes for RGB, scale to 0.4-0.9 range for pleasant colors
    let r = (bssid[3] as f64 / 255.0) * 0.5 + 0.4;
    let g = (bssid[4] as f64 / 255.0) * 0.5 + 0.4;
    let b = (bssid[5] as f64 / 255.0) * 0.5 + 0.4;

    RGBA::new(r as f32, g as f32, b as f32, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uptime_text_formats_minutes() {
        assert_eq!(formatted_uptime_text(Duration::from_secs(0)), "0m");
        assert_eq!(formatted_uptime_text(Duration::from_secs(59 * 60)), "59m");
    }

    #[test]
    fn uptime_text_formats_hours_and_minutes() {
        assert_eq!(formatted_uptime_text(Duration::from_secs(60 * 60)), "1h 0m");
        assert_eq!(
            formatted_uptime_text(Duration::from_secs((2 * 60 + 30) * 60)),
            "2h 30m"
        );
    }

    #[test]
    fn uptime_text_formats_days_hours_and_minutes() {
        assert_eq!(
            formatted_uptime_text(Duration::from_secs(24 * 60 * 60)),
            "1d 0h 0m"
        );
        assert_eq!(
            formatted_uptime_text(Duration::from_secs(((3 * 24 + 4) * 60 + 5) * 60)),
            "3d 4h 5m"
        );
    }

    #[test]
    fn uptime_text_formats_years_days_hours_and_minutes() {
        assert_eq!(
            formatted_uptime_text(Duration::from_secs(365 * 24 * 60 * 60)),
            "1y 0d 0h 0m"
        );
        assert_eq!(
            formatted_uptime_text(Duration::from_secs(
                (((2 * 365 + 10) * 24 + 3) * 60 + 4) * 60
            )),
            "2y 10d 3h 4m"
        );
    }

    #[test]
    fn vendor_display_name_leaves_short_names_unchanged() {
        assert_eq!(format_vendor_display_name("Cisco Meraki"), "Cisco Meraki");
        assert_eq!(format_vendor_display_name("CommScope Inc"), "CommScope Inc");
    }

    #[test]
    fn vendor_display_name_shortens_long_names_to_two_words() {
        assert_eq!(
            format_vendor_display_name("Extreme Networks Headquarters"),
            "Extreme Networks"
        );
        assert_eq!(
            format_vendor_display_name("Sagemcom Broadband SAS"),
            "Sagemcom Broadband"
        );
        assert_eq!(
            format_vendor_display_name("Hewlett Packard Enterprise"),
            "Hewlett Packard"
        );
    }

    #[test]
    fn vendor_display_name_keeps_more_than_two_words_when_they_fit() {
        assert_eq!(
            format_vendor_display_name("Hon Hai Precision Ind. Co.,Ltd."),
            "Hon Hai Precision"
        );
    }

    #[test]
    fn vendor_display_name_strips_trailing_commas() {
        assert_eq!(
            format_vendor_display_name("Cisco Systems, Incorporated"),
            "Cisco Systems"
        );
    }

    #[test]
    fn vendor_display_name_strips_trailing_hyphens_and_whitespace() {
        assert_eq!(
            format_vendor_display_name("Example- Vendor- Incorporated"),
            "Example Vendor"
        );
    }

    #[test]
    fn vendor_display_name_ignores_separator_tokens() {
        assert_eq!(
            format_vendor_display_name("Vantiva - Connected Home"),
            "Vantiva Connected"
        );
    }
}
