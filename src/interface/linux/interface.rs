use super::{InterfaceType, Nl80211Attr, Nl80211Cmd};
use crate::{bss::Bss, interface::ScanError};
use macaddr::MacAddr6;
use neli::{
    attr::Attribute,
    consts::nl::{NlmF, NlmFFlags, Nlmsg},
    consts::socket::NlFamily,
    genl::{Genlmsghdr, Nlattr},
    nl::{NlPayload, Nlmsghdr},
    socket::NlSocketHandle,
    types::Buffer,
};
use std::{
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
    hash::Hash,
};

#[derive(Debug, Clone)]
pub struct Interface {
    name: String,
    index: u32,
    interface_type: InterfaceType,
    wiphy: u32,
    wdev: u64,
    mac_address: MacAddr6,
}

impl Interface {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn interface_type(&self) -> InterfaceType {
        self.interface_type
    }

    pub fn wiphy(&self) -> u32 {
        self.wiphy
    }

    pub fn mac_address(&self) -> MacAddr6 {
        self.mac_address
    }

    pub fn scan(&self) -> Result<HashSet<Bss>, ScanError> {
        // Create a generic netlink message header containing the TriggerScan command
        let genl_msghdr = {
            let attr = Nlattr::new(None, false, true, Nl80211Attr::Ifindex, self.index);
            Genlmsghdr::new(Nl80211Cmd::TriggerScan, 1, attr.into_iter().collect())
        };

        let mut socket = NlSocketHandle::connect(NlFamily::Generic, None, &[])?;

        // Create a netlink message header with the generic netlink message header as its payload
        let nl_msghdr = {
            let id = socket.resolve_genl_family(super::NL80211_FAMILY_NAME)?;
            let flags = NlmFFlags::new(&[NlmF::Request, NlmF::Ack]);
            let payload = NlPayload::Payload(genl_msghdr);
            Nlmsghdr::new(None, id, flags, None, None, payload)
        };

        // Send the message header with the TriggerScan command
        socket.send(nl_msghdr)?;

        // Wait for the message we just sent to be acknowledged
        socket.recv_all::<Nlmsg, Buffer>()?;

        // Join the scan multicast group to receive a notification when the scan is completed
        let id = socket
            .resolve_nl_mcast_group(super::NL80211_FAMILY_NAME, super::SCAN_MULTICAST_NAME)?;
        socket.add_mcast_membership(&[id])?;

        // If we receive a message with a payload containing Nl80211Cmd::NewScanResults, we know
        // a new scan has successfully completed and we can get the scan results by calling
        // Interface::cached_scan_results()
        let received_new_scan_notification = socket
            .recv_all::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>()?
            .iter()
            .filter_map(|nl_msghdr| nl_msghdr.get_payload().ok())
            .find(|payload| payload.cmd == Nl80211Cmd::NewScanResults)
            .is_some();
        if received_new_scan_notification {
            self.cached_scan_results()
        } else {
            Err(ScanError::AlreadyScanning)
        }
    }

    pub fn scan_for_ssid(&self, _: &str) {
        todo!()
    }

    pub fn cached_scan_results(&self) -> Result<HashSet<Bss>, ScanError> {
        // Create a generic netlink message header containing the GetScan command
        let genl_msghdr = {
            let attr = Nlattr::new(None, false, true, Nl80211Attr::Ifindex, self.index);
            Genlmsghdr::new(Nl80211Cmd::GetScan, 1, attr.into_iter().collect())
        };

        let mut socket = NlSocketHandle::connect(NlFamily::Generic, None, &[])?;

        // Create a netlink message header with the generic netlink message header as its payload
        let nl_msghdr = {
            let id = socket.resolve_genl_family(super::NL80211_FAMILY_NAME)?;
            let flags = NlmFFlags::new(&[NlmF::Request, NlmF::Dump]);
            let payload = NlPayload::Payload(genl_msghdr);
            Nlmsghdr::new(None, id, flags, None, None, payload)
        };

        socket.send(nl_msghdr)?;

        // Receive all responses and attempt to convert each one of them to a Bss
        Ok(socket
            .recv_all::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>()?
            .iter()
            .filter_map(|nl_msghdr| nl_msghdr.get_payload().ok())
            .filter(|payload| payload.cmd == Nl80211Cmd::NewScanResults)
            .map(|payload| payload.get_attr_handle())
            .filter_map(|mut attr_handle| {
                if let Ok(bss_attr_handle) = attr_handle.get_nested_attributes(Nl80211Attr::Bss) {
                    Bss::try_from(bss_attr_handle.get_attrs()).ok()
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>())
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

impl Eq for Interface {}

impl TryFrom<&[Nlattr<Nl80211Attr, Buffer>]> for Interface {
    type Error = ();

    fn try_from(iface_attrs: &[Nlattr<Nl80211Attr, Buffer>]) -> Result<Self, Self::Error> {
        let iface_attrs: HashMap<_, _> = iface_attrs
            .iter()
            .map(|attr| (attr.nla_type, attr))
            .collect();

        Ok(Interface {
            name: iface_attrs
                .get(&Nl80211Attr::Ifname)
                .and_then(|attr| attr.payload().as_ref().split_last())
                .and_then(|(_, name_bytes)| String::from_utf8(name_bytes.to_vec()).ok())
                .ok_or(())?,
            index: iface_attrs
                .get(&Nl80211Attr::Ifindex)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            interface_type: iface_attrs
                .get(&Nl80211Attr::Iftype)
                .and_then(|attr| attr.get_payload_as().ok())
                .and_then(|if_type: u32| InterfaceType::try_from(if_type).ok())
                .ok_or(())?,
            wiphy: iface_attrs
                .get(&Nl80211Attr::Wiphy)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            wdev: iface_attrs
                .get(&Nl80211Attr::Wdev)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            mac_address: iface_attrs
                .get(&Nl80211Attr::Mac)
                .and_then(|attr| attr.payload().as_ref().try_into().ok())
                .map(|mac_bytes: [u8; 6]| MacAddr6::from(mac_bytes))
                .ok_or(())?,
        })
    }
}
