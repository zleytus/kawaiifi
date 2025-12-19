use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    hash::Hash,
    iter::once,
};

use neli::{
    attr::Attribute,
    consts::{nl::NlmF, socket::NlFamily},
    genl::{AttrTypeBuilder, Genlmsghdr, GenlmsghdrBuilder, Nlattr, NlattrBuilder},
    nl::{NlPayload, Nlmsghdr},
    router::synchronous::NlRouter,
    types::{Buffer, GenlBuffer},
    utils::Groups,
};

use crate::{
    Bss, ChannelWidth,
    nl80211::{Attr, ChanWidth, IfType},
    scan,
};
use crate::{Bss, ChannelWidth};

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

        }

        }
    }

    }


        None
    }


        None
    }



    }


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
