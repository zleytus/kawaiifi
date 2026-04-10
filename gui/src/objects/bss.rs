use std::{cell::RefCell, ops::Deref, time::Duration};

use gtk::gdk::RGBA;
use gtk::glib;
use gtk::subclass::prelude::*;
use kawaiifi::{
    Band, CapabilityInfo, ChannelWidth, SecurityProtocols, WifiAmendments, WifiProtocols,
};

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
    pub fn bss(&self) -> std::cell::Ref<'_, BssInternal> {
        std::cell::Ref::map(self.imp().bss.borrow(), |opt| {
            opt.as_ref().expect("BssObject not properly initialized")
        })
    }

    fn bss_mut(&self) -> std::cell::RefMut<'_, BssInternal> {
        std::cell::RefMut::map(self.imp().bss.borrow_mut(), |opt| {
            opt.as_mut().expect("BssObject not properly initialized")
        })
    }

    /// How long ago this BSS was last seen, or `None` if the timestamp is unavailable.
    pub fn time_since_last_seen(&self) -> Option<Duration> {
        self.bss().time_since_last_seen()
    }

    /// The raw 6-byte BSSID.
    pub fn bssid_bytes(&self) -> [u8; 6] {
        *self.bss().bssid()
    }

    /// The display color derived from the BSSID.
    pub fn color(&self) -> RGBA {
        self.bss().color()
    }

    /// The SSID, or `None` for hidden networks.
    pub fn ssid(&self) -> Option<String> {
        self.bss().ssid().map(|s| s.replace('\0', "�"))
    }

    /// The BSSID formatted as a colon-separated hex string (e.g. `AA:BB:CC:DD:EE:FF`).
    pub fn bssid(&self) -> String {
        let bss = self.bss();
        let mut s = String::with_capacity(17); // "XX:XX:XX:XX:XX:XX"
        for (i, byte) in bss.bssid().iter().enumerate() {
            if i > 0 {
                s.push(':');
            }
            s.push_str(&format!("{:02X}", byte));
        }
        s
    }

    /// The OUI vendor name, or an empty string if unknown.
    pub fn vendor(&self) -> String {
        self.bss()
            .vendor()
            .map(|vendor| vendor.to_string())
            .unwrap_or_default()
    }

    /// Sets the OUI vendor name.
    pub fn set_vendor(&self, vendor: String) {
        self.bss_mut().set_vendor(vendor);
    }

    /// The received signal strength in dBm.
    pub fn signal_strength(&self) -> i32 {
        self.bss().signal_dbm()
    }

    /// The 802.11 channel number.
    pub fn channel_number(&self) -> u8 {
        u8::from(self.bss().channel_number())
    }

    /// The channel width.
    pub fn channel_width(&self) -> ChannelWidth {
        self.bss().channel_width()
    }

    /// The operating frequency in MHz.
    pub fn frequency_mhz(&self) -> u32 {
        self.bss().frequency_mhz()
    }

    /// The center frequency of the full channel in MHz.
    pub fn center_frequency_mhz(&self) -> u32 {
        self.bss().center_frequency_mhz()
    }

    /// The frequency band the BSS operates on.
    pub fn band(&self) -> Band {
        self.bss().band()
    }

    /// The Wi-Fi protocols supported by the BSS.
    pub fn protocols(&self) -> WifiProtocols {
        self.bss().wifi_protocols()
    }

    /// The 802.11 amendments supported by the BSS.
    pub fn amendments(&self) -> WifiAmendments {
        self.bss().wifi_amendments()
    }

    /// The security protocols supported by the BSS.
    pub fn security(&self) -> SecurityProtocols {
        self.bss().security_protocols()
    }

    /// The maximum supported data rate in Mbps.
    pub fn max_rate(&self) -> f64 {
        self.bss().max_rate_mbps()
    }

    /// The channel utilization as a value from 0 to 255, where 255 represents 100%, or `None` if unavailable.
    pub fn channel_utilization(&self) -> Option<u8> {
        self.bss().channel_utilization()
    }

    /// The number of associated stations, or `None` if unavailable.
    pub fn station_count(&self) -> Option<u16> {
        self.bss().station_count()
    }

    /// The estimated time the BSS has been running, derived from its TSF timer.
    pub fn uptime(&self) -> Duration {
        self.bss().uptime()
    }

    /// The uptime formatted as a human-readable string (e.g. `2d 3h 45m`).
    pub fn formatted_uptime(&self) -> String {
        let secs = self.uptime().as_secs();
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

    /// The 802.11 capability information flags.
    pub fn capability_info(&self) -> CapabilityInfo {
        self.bss().capability_info().clone()
    }
}

fn color_from_bssid(bssid: &[u8; 6]) -> RGBA {
    // Use last 3 bytes for RGB, scale to 0.4-0.9 range for pleasant colors
    let r = (bssid[3] as f64 / 255.0) * 0.5 + 0.4;
    let g = (bssid[4] as f64 / 255.0) * 0.5 + 0.4;
    let b = (bssid[5] as f64 / 255.0) * 0.5 + 0.4;

    RGBA::new(r as f32, g as f32, b as f32, 1.0)
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
        Self {
            color: color_from_bssid(bss.bssid()),
            bss,
            vendor: None,
        }
    }

    /// Replaces the underlying [`kawaiifi::Bss`] with a newer scan result.
    pub fn update(&mut self, bss: kawaiifi::Bss) {
        self.bss = bss;
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
