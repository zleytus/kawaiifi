use std::{collections::HashMap, ffi::CString, fmt::Display, hash::Hash};

use neli::{
    attr::Attribute,
    consts::{nl::NlmF, socket::NlFamily},
    genl::{Genlmsghdr, GenlmsghdrBuilder},
    nl::NlPayload,
    router::synchronous::NlRouter,
    utils::Groups,
};

use pci_ids::FromId as FromPciId;
use usb_ids::FromId as FromUsbId;

use crate::{
    Bss, ChannelWidth, Scan,
    nl80211::{Attr, ChanWidth, Cmd, IfType, NL80211_FAMILY_NAME, ParseError},
    scan::{self, Error},
};

pub(super) fn interfaces() -> Result<Vec<Interface>, Error> {
    // Create a generic netlink socket and resolve nl80211 family
    let (socket, _) = NlRouter::connect(NlFamily::Generic, None, Groups::empty())?;
    let family_id = socket.resolve_genl_family(NL80211_FAMILY_NAME)?;

    // Query system for WiFi interfaces using the 'GetInterface' command
    // No attributes needed because DUMP flag will return all interfaces
    let recv = socket.send::<_, _, u16, Genlmsghdr<Cmd, Attr>>(
        family_id,
        NlmF::DUMP | NlmF::REQUEST,
        NlPayload::Payload(
            GenlmsghdrBuilder::default()
                .cmd(Cmd::GetInterface)
                .version(1)
                .build()?,
        ),
    )?;

    // Receive all responses and attempt to convert each of them into an Interface
    let mut interfaces = Vec::new();
    for msg in recv {
        let msg = msg?;
        if let Some(payload) = msg.get_payload()
            && let Ok(interface) = Interface::try_from(payload)
        {
            interfaces.push(interface);
        }
    }

    Ok(interfaces)
}

#[derive(Debug, Clone, Eq)]
pub struct Interface {
    name: CString,
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
    pub fn name(&self) -> &str {
        self.name.to_str().unwrap_or_default()
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
        let subsystem_path = format!("/sys/class/net/{}/device/subsystem", self.name());

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

    #[tracing::instrument(skip(self), fields(interface = %self.name()))]
    pub async fn scan(&self, backend: scan::Backend) -> Result<Scan, scan::Error> {
        scan::scan(self, backend).await
    }

    #[tracing::instrument(skip(self), fields(interface = %self.name()))]
    pub fn scan_blocking(&self, backend: scan::Backend) -> Result<Scan, scan::Error> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(scan::scan(self, backend))
    }

    pub async fn cached_scan_results(&self) -> Result<Vec<Bss>, scan::Error> {
        let (socket, _) = neli::router::asynchronous::NlRouter::connect(
            neli::consts::socket::NlFamily::Generic,
            None,
            neli::utils::Groups::empty(),
        )
        .await?;
        scan::scan_results(self, &socket).await
    }

    pub fn cached_scan_results_blocking(&self) -> Result<Vec<Bss>, scan::Error> {
        let (socket, _) = neli::router::synchronous::NlRouter::connect(
            neli::consts::socket::NlFamily::Generic,
            None,
            neli::utils::Groups::empty(),
        )?;
        scan::scan_results_blocking(self, &socket)
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

impl TryFrom<&Genlmsghdr<Cmd, Attr>> for Interface {
    type Error = ParseError;

    fn try_from(msghdr: &Genlmsghdr<Cmd, Attr>) -> Result<Self, Self::Error> {
        if *msghdr.cmd() != Cmd::NewInterface {
            return Err(ParseError::UnexpectedCommand {
                expected: Cmd::NewInterface,
                got: *msghdr.cmd(),
            });
        }

        let attr_handle = msghdr.attrs().get_attr_handle();
        let interface_attrs: HashMap<_, _> = attr_handle
            .iter()
            .map(|attr| (attr.nla_type().nla_type(), attr))
            .collect();

        Ok(Interface {
            name: CString::from_vec_with_nul(
                interface_attrs
                    .get(&Attr::Ifname)
                    .ok_or(ParseError::MissingAttribute("Attr::Ifname"))?
                    .payload()
                    .as_ref()
                    .to_vec(),
            )
            .unwrap_or_default(),
            index: interface_attrs
                .get(&Attr::Ifindex)
                .ok_or(ParseError::MissingAttribute("Attr::Ifindex"))?
                .get_payload_as()?,
            interface_type: interface_attrs
                .get(&Attr::Iftype)
                .ok_or(ParseError::MissingAttribute("Attr::Iftype"))?
                .get_payload_as::<u32>()?
                .try_into()
                .map_err(|_| ParseError::TryFromPrimitive {
                    primitive: "u32",
                    expected_type: "IfType",
                })?,
            wiphy: interface_attrs
                .get(&Attr::Wiphy)
                .ok_or(ParseError::MissingAttribute("Attr::Wiphy"))?
                .get_payload_as()?,
            wdev: interface_attrs
                .get(&Attr::Wdev)
                .ok_or(ParseError::MissingAttribute("Attr::Wdev"))?
                .get_payload_as()?,
            mac_address: interface_attrs
                .get(&Attr::Mac)
                .ok_or(ParseError::MissingAttribute("Attr::Mac"))?
                .get_payload_as()?,
            generation: interface_attrs
                .get(&Attr::Generation)
                .ok_or(ParseError::MissingAttribute("Attr::Generation"))?
                .get_payload_as()?,
            four_address: interface_attrs
                .get(&Attr::FourAddr)
                .ok_or(ParseError::MissingAttribute("Attr::FourAddr"))?
                .get_payload_as::<u8>()?
                > 0,
            ssid: interface_attrs.get(&Attr::Ssid).map(|attr| {
                String::from_utf8(attr.payload().as_ref().to_vec()).unwrap_or_default()
            }),
            wiphy_freq_mhz: interface_attrs
                .get(&Attr::WiphyFreq)
                .and_then(|attr| attr.get_payload_as().ok()),
            wiphy_freq_offset_khz: interface_attrs
                .get(&Attr::WiphyFreqOffset)
                .and_then(|attr| attr.get_payload_as().ok()),
            wiphy_tx_power_level_mbm: interface_attrs
                .get(&Attr::WiphyTxPowerLevel)
                .and_then(|attr| attr.get_payload_as().ok()),
            center_freq_1_mhz: interface_attrs
                .get(&Attr::CenterFreq1)
                .and_then(|attr| attr.get_payload_as().ok()),
            center_freq_2_mhz: interface_attrs
                .get(&Attr::CenterFreq2)
                .and_then(|attr| attr.get_payload_as().ok()),
            channel_width: interface_attrs
                .get(&Attr::ChannelWidth)
                .and_then(|attr| attr.get_payload_as().ok())
                .map(|channel_width: u8| {
                    ChanWidth::try_from(channel_width).unwrap_or(ChanWidth::TwentyMhz)
                })
                .map(ChannelWidth::from),
            vif_radio_mask: interface_attrs
                .get(&Attr::VifRadioMask)
                .and_then(|attr| attr.get_payload_as().ok()),
        })
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
