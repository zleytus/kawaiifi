use std::{collections::HashMap, fmt::Display, hash::Hash};

use neli::{attr::Attribute, genl::Nlattr, types::Buffer};
use pci_ids::FromId as FromPciId;
use usb_ids::FromId as FromUsbId;

use crate::{
    Bss, ChannelWidth,
    nl80211::{Attr, ChanWidth, IfType},
    scan,
};

#[derive(Debug, Clone, Eq)]
pub struct Interface {
    name: String,
    index: u32,
    interface_type: IfType,
    wiphy: u32,
    wdev: u64,
    mac_address: [u8; 6],
    generation: u32,
    four_address: bool,
    ssid: Option<String>,
    wiphy_freq_mhz: Option<u32>,
    wiphy_freq_offset_khz: Option<u32>,
    wiphy_tx_power_level_mbm: Option<u32>,
    center_freq_1_mhz: Option<u32>,
    center_freq_2_mhz: Option<u32>,
    channel_width: Option<ChannelWidth>,
    vif_radio_mask: Option<u32>,
}

impl Interface {
    pub(crate) fn from_attrs<'a, I>(iface_attrs: I) -> Result<Self, ()>
    where
        I: IntoIterator<Item = &'a Nlattr<Attr, Buffer>>,
    {
        let iface_attrs: HashMap<_, _> = iface_attrs
            .into_iter()
            .map(|attr| (attr.nla_type().nla_type(), attr))
            .collect();

        Ok(Interface {
            name: iface_attrs
                .get(&Attr::Ifname)
                .and_then(|attr| attr.payload().as_ref().split_last())
                .and_then(|(_, name_bytes)| String::from_utf8(name_bytes.to_vec()).ok())
                .ok_or(())?,
            index: iface_attrs
                .get(&Attr::Ifindex)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            interface_type: iface_attrs
                .get(&Attr::Iftype)
                .and_then(|attr| attr.get_payload_as().ok())
                .and_then(|if_type: u32| IfType::try_from(if_type).ok())
                .ok_or(())?,
            wiphy: iface_attrs
                .get(&Attr::Wiphy)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            wdev: iface_attrs
                .get(&Attr::Wdev)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            mac_address: iface_attrs
                .get(&Attr::Mac)
                .and_then(|attr| attr.payload().as_ref().try_into().ok())
                .ok_or(())?,
            generation: iface_attrs
                .get(&Attr::Generation)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            four_address: iface_attrs
                .get(&Attr::FourAddr)
                .and_then(|attr| attr.get_payload_as().ok())
                .map(|four_address: u8| four_address > 0)
                .ok_or(())?,
            ssid: iface_attrs.get(&Attr::Ssid).map(|attr| {
                String::from_utf8(attr.payload().as_ref().to_vec()).unwrap_or_default()
            }),
            wiphy_freq_mhz: iface_attrs
                .get(&Attr::WiphyFreq)
                .and_then(|attr| attr.get_payload_as().ok()),
            wiphy_freq_offset_khz: iface_attrs
                .get(&Attr::WiphyFreqOffset)
                .and_then(|attr| attr.get_payload_as().ok()),
            wiphy_tx_power_level_mbm: iface_attrs
                .get(&Attr::WiphyTxPowerLevel)
                .and_then(|attr| attr.get_payload_as().ok()),
            center_freq_1_mhz: iface_attrs
                .get(&Attr::CenterFreq1)
                .and_then(|attr| attr.get_payload_as().ok()),
            center_freq_2_mhz: iface_attrs
                .get(&Attr::CenterFreq2)
                .and_then(|attr| attr.get_payload_as().ok()),
            channel_width: iface_attrs
                .get(&Attr::ChannelWidth)
                .and_then(|attr| attr.get_payload_as().ok())
                .map(|channel_width: u8| {
                    ChanWidth::try_from(channel_width).unwrap_or(ChanWidth::TwentyMhz)
                })
                .map(ChannelWidth::from),
            vif_radio_mask: iface_attrs
                .get(&Attr::VifRadioMask)
                .and_then(|attr| attr.get_payload_as().ok()),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn interface_type(&self) -> IfType {
        self.interface_type
    }

    pub fn wiphy(&self) -> u32 {
        self.wiphy
    }

    pub fn wdev(&self) -> u64 {
        self.wdev
    }

    pub fn mac_address(&self) -> [u8; 6] {
        self.mac_address
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn four_address(&self) -> bool {
        self.four_address
    }

    pub fn ssid(&self) -> Option<&str> {
        self.ssid.as_deref()
    }

    pub fn wiphy_freq_mhz(&self) -> Option<u32> {
        self.wiphy_freq_mhz
    }

    pub fn wiphy_freq_offset_khz(&self) -> Option<u32> {
        self.wiphy_freq_offset_khz
    }

    pub fn wiphy_tx_power_level_mbm(&self) -> Option<u32> {
        self.wiphy_tx_power_level_mbm
    }

    pub fn center_freq_1_mhz(&self) -> Option<u32> {
        self.center_freq_1_mhz
    }

    pub fn center_freq_2_mhz(&self) -> Option<u32> {
        self.center_freq_2_mhz
    }

    pub fn channel_width(&self) -> Option<ChannelWidth> {
        self.channel_width
    }

    pub fn vif_radio_mask(&self) -> Option<u32> {
        self.vif_radio_mask
    }

    pub fn vendor_name(&self) -> Option<String> {
        // Fall back to database lookup using vendor ID
        let vendor_id = self.vendor_id()?;

        // Try USB database
        if let Some(vendor) = usb_ids::Vendor::from_id(vendor_id) {
            return Some(vendor.name().to_string());
        }

        // Try PCI database
        if let Some(vendor) = pci_ids::Vendor::from_id(vendor_id) {
            return Some(vendor.name().to_string());
        }

        None
    }

    pub fn device_name(&self) -> Option<String> {
        // Fall back to database lookup
        let vendor_id = self.vendor_id()?;
        let device_id = self.device_id()?;

        // Try USB database
        if let Some(vendor) = usb_ids::Vendor::from_id(vendor_id) {
            if let Some(device) = vendor.devices().find(|d| d.id() == device_id) {
                return Some(device.name().to_string());
            }
        }

        // Try PCI database
        if let Some(vendor) = pci_ids::Vendor::from_id(vendor_id) {
            if let Some(device) = vendor.devices().find(|d| d.id() == device_id) {
                return Some(device.name().to_string());
            }
        }

        None
    }

    pub fn vendor_id(&self) -> Option<u16> {
        let uevent =
            std::fs::read_to_string(format!("/sys/class/net/{}/device/uevent", self.name()))
                .ok()?;

        for line in uevent.lines() {
            // Try PCI format: PCI_ID=14C3:0616
            if let Some(pci_id) = line.strip_prefix("PCI_ID=") {
                let vendor_hex = pci_id.split(':').next()?;
                return u16::from_str_radix(vendor_hex, 16).ok();
            }
            // Try USB format: PRODUCT=846/9072/100 (decimal)
            if let Some(product) = line.strip_prefix("PRODUCT=") {
                let vendor_hex = product.split('/').next()?;
                return u16::from_str_radix(vendor_hex, 16).ok();
            }
        }

        None
    }

    pub fn device_id(&self) -> Option<u16> {
        let uevent =
            std::fs::read_to_string(format!("/sys/class/net/{}/device/uevent", self.name()))
                .ok()?;

        for line in uevent.lines() {
            // Try PCI format: PCI_ID=14C3:0616
            if let Some(pci_id) = line.strip_prefix("PCI_ID=") {
                let device_hex = pci_id.split(':').nth(1)?;
                return u16::from_str_radix(device_hex, 16).ok();
            }
            // Try USB format: PRODUCT=846/9072/100 (decimal)
            if let Some(product) = line.strip_prefix("PRODUCT=") {
                let device_hex = product.split('/').nth(1)?;
                return u16::from_str_radix(device_hex, 16).ok();
            }
        }

        None
    }

    pub fn driver(&self) -> Option<String> {
        let uevent =
            std::fs::read_to_string(format!("/sys/class/net/{}/device/uevent", self.name()))
                .ok()?;

        for line in uevent.lines() {
            if let Some(driver) = line.strip_prefix("DRIVER=") {
                return Some(driver.to_string());
            }
        }

        None
    }

    pub fn bus_type(&self) -> BusType {
        let subsystem_path = format!("/sys/class/net/{}/device/subsystem", self.name);

        if let Ok(link) = std::fs::read_link(&subsystem_path) {
            let subsystem = link.to_string_lossy();

            if subsystem.contains("pci") {
                return BusType::Pci;
            } else if subsystem.contains("usb") {
                return BusType::Usb;
            } else if subsystem.contains("sdio") {
                return BusType::Sdio;
            }
        }

        BusType::Unknown
    }

    pub async fn scan(&self, scan_backend: scan::Backend) -> Result<(), scan::Error> {
        crate::scan::scan(&self, scan_backend).await
    }

    pub fn scan_blocking(&self, scan_backend: scan::Backend) -> Result<(), scan::Error> {
        crate::scan::scan_blocking(&self, scan_backend)
    }

    pub async fn scan_and_get_results(
        &self,
        scan_backend: scan::Backend,
    ) -> Result<Vec<Bss>, scan::Error> {
        crate::scan::scan(&self, scan_backend).await?;
        crate::scan::scan_results(&self).await
    }

    pub fn scan_and_get_results_blocking(
        &self,
        scan_backend: scan::Backend,
    ) -> Result<Vec<Bss>, scan::Error> {
        crate::scan::scan_blocking(&self, scan_backend)?;
        crate::scan::scan_results_blocking(&self)
    }

    pub async fn scan_results(&self) -> Result<Vec<Bss>, scan::Error> {
        crate::scan::scan_results(&self).await
    }

    pub fn scan_results_blocking(&self) -> Result<Vec<Bss>, scan::Error> {
        crate::scan::scan_results_blocking(&self)
    }
}

impl Hash for Interface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusType {
    Pci,
    Usb,
    Sdio,
    Unknown,
}

impl Display for BusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Pci => write!(f, "PCIe"),
            Self::Usb => write!(f, "USB"),
            Self::Sdio => write!(f, "SDIO"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
