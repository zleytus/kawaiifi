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

    pub fn scan(&self) -> Result<Vec<Bss>, ScanError> {
        // Connect to the generic netlink socket
        let (socket, multicast) = NlRouter::connect(NlFamily::Generic, None, Groups::empty())?;

        // Resolve the nl80211 family ID
        let family_id = socket.resolve_genl_family(NL80211_FAMILY_NAME)?;

        // Build the netlink attribute specifying which interface to query
        let ifindex_attr = NlattrBuilder::default()
            .nla_type(
                AttrTypeBuilder::default()
                    .nla_type(Nl80211Attr::Ifindex)
                    .build()?,
            )
            .nla_payload(self.index)
            .build()?;

        let attrs = once(ifindex_attr).collect::<GenlBuffer<_, _>>();

        // Build and send the NL80211_CMD_TRIGGER_SCAN request
        let genlmsghdr = GenlmsghdrBuilder::default()
            .cmd(Nl80211Cmd::TriggerScan)
            .attrs(attrs)
            .version(1)
            .build()?;

        let responses = socket.send::<_, _, u16, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>(
            family_id,
            NlmF::REQUEST | NlmF::ACK,
            NlPayload::Payload(genlmsghdr),
        )?;

        // Wait for the message we just sent to be acknowledged
        // If we don't receive an ACK, assume the user was denied permission to scan
        if !responses
            .filter_map(|msg| msg.ok())
            .any(|msg| matches!(msg.nl_payload(), NlPayload::Ack(_)))
        {
            return Err(ScanError::PermissionDenied);
        }

        // Join the scan multicast group to receive a notification when the scan is completed
        let id = socket.resolve_nl_mcast_group(NL80211_FAMILY_NAME, SCAN_MULTICAST_NAME)?;
        socket.add_mcast_membership(Groups::new_groups(&[id]))?;

        // If we receive a message with a payload containing Nl80211Cmd::NewScanResults, we know
        // a new scan has successfully completed and we can get the scan results by calling
        // Interface::cached_scan_results()
        if multicast
            .filter_map(|msg| msg.ok())
            .filter_map(|msg| msg.get_payload().map(|payload| *payload.cmd()))
            .any(|cmd| cmd == u8::from(Nl80211Cmd::NewScanResults))
        {
            self.cached_scan_results()
        } else {
            Err(ScanError::AlreadyScanning)
        }
    }

    pub fn scan_for_ssid(&self, _: &str) {
        todo!()
    }

    pub fn cached_scan_results(&self) -> Result<Vec<Bss>, ScanError> {
        // Connect to the generic netlink socket
        let (socket, _) = NlRouter::connect(NlFamily::Generic, None, Groups::empty())?;

        // Resolve the nl80211 family ID
        let family_id = socket.resolve_genl_family(NL80211_FAMILY_NAME)?;

        // Build the netlink attribute specifying which interface to query
        let ifindex_attr = NlattrBuilder::default()
            .nla_type(
                AttrTypeBuilder::default()
                    .nla_type(Nl80211Attr::Ifindex)
                    .build()?,
            )
            .nla_payload(self.index)
            .build()?;

        let attrs = once(ifindex_attr).collect::<GenlBuffer<_, _>>();

        // Build and send the NL80211_CMD_GET_SCAN request
        let genlmsghdr = GenlmsghdrBuilder::default()
            .cmd(Nl80211Cmd::GetScan)
            .attrs(attrs)
            .version(1)
            .build()?;

        let responses = socket.send::<_, _, u16, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>(
            family_id,
            NlmF::DUMP | NlmF::ACK,
            NlPayload::Payload(genlmsghdr),
        )?;

        // Process responses and extract BSS information
        Ok(responses
            .into_iter()
            .filter_map(|msghdr| msghdr.ok())
            .filter_map(|msghdr| Self::extract_bss_from_message(msghdr))
            .collect())
    }

    /// Extract BSS information from a netlink message
    fn extract_bss_from_message(
        msghdr: Nlmsghdr<u16, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>,
    ) -> Option<Bss> {
        // Get the payload from the message
        let payload = msghdr.get_payload()?;

        // Only process NL80211_CMD_NEW_SCAN_RESULTS messages
        if *payload.cmd() != Nl80211Cmd::NewScanResults {
            return None;
        }

        // Get the nested BSS attributes
        let attr_handle = payload.attrs().get_attr_handle();
        let bss_attrs = attr_handle.get_nested_attributes(Nl80211Attr::Bss).ok()?;

        // Parse the attributes into a Bss struct
        Bss::from_attrs(bss_attrs.iter()).ok()
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
