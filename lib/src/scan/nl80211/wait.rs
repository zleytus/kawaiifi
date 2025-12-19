use neli::{
    attr::Attribute,
    consts::socket::NlFamily,
    genl::Genlmsghdr,
    router::{asynchronous, synchronous},
    utils::Groups,
};

use super::SCAN_MULTICAST_NAME;
use crate::{
    nl80211::{Attr, Cmd, NL80211_FAMILY_NAME},
    scan::Error,
};

pub(crate) async fn wait_for_new_scan_results(
    interface_index: u32,
    scan_start: std::time::Instant,
) -> Result<(), Error> {
    // Now join multicast group to listen for scan completion
    let (socket, mut multicast) =
        asynchronous::NlRouter::connect(NlFamily::Generic, None, Groups::empty()).await?;

    let id = socket
        .resolve_nl_mcast_group(NL80211_FAMILY_NAME, SCAN_MULTICAST_NAME)
        .await?;
    socket.add_mcast_membership(Groups::new_groups(&[id]))?;

    // Wait for NewScanResults message for this interface
    // Filter by: 1) correct ifindex, 2) received at least 1000ms after scan trigger
    while let Some(msg) = multicast.next::<u16, Genlmsghdr<Cmd, Attr>>().await {
        let Ok(msg) = msg else {
            continue;
        };
        let Some(payload) = msg.get_payload() else {
            continue;
        };
        // Check if this is a NewScanResults command
        if *payload.cmd() != Cmd::NewScanResults {
            continue;
        }
        // Check if it's for our interface
        let attr_handle = payload.attrs().get_attr_handle();
        let Some(ifindex_attr) = attr_handle.get_attribute(Attr::Ifindex) else {
            continue;
        };
        let Ok(msg_ifindex) = ifindex_attr.get_payload_as::<u32>() else {
            continue;
        };
        if msg_ifindex != interface_index {
            continue;
        }
        // Check timing - ignore stale messages received too quickly
        if scan_start.elapsed() >= std::time::Duration::from_millis(1000) {
            // This is a fresh NewScanResults for our interface

            // Get the scan frequencies
            if let Some(scan_freqs) = attr_handle.get_attribute(Attr::ScanFrequencies) {
                let handle = scan_freqs.get_attr_handle::<u16>().unwrap();
                let freqs: Vec<u32> = handle
                    .iter()
                    .map(|attr| attr.get_payload_as::<u32>().unwrap_or_default())
                    .collect();
                dbg!(freqs);
            }
            return Ok(());
        }
        // Else: received too soon, probably stale - keep waiting
    }

    Err(Error::AlreadyScanning)
}

pub(crate) fn wait_for_new_scan_results_blocking(
    interface_index: u32,
    scan_start: std::time::Instant,
) -> Result<(), Error> {
    // Now join multicast group to listen for scan completion
    let (socket, mut multicast) =
        synchronous::NlRouter::connect(NlFamily::Generic, None, Groups::empty())?;

    let id = socket.resolve_nl_mcast_group(NL80211_FAMILY_NAME, SCAN_MULTICAST_NAME)?;
    socket.add_mcast_membership(Groups::new_groups(&[id]))?;

    // Wait for NewScanResults message for this interface
    // Filter by: 1) correct ifindex, 2) received at least 1000ms after scan trigger
    while let Some(msg) = multicast.next() {
        if let Ok(msg) = msg {
            if let Some(payload) = msg.get_payload() {
                // Check if this is a NewScanResults command
                if *payload.cmd() == u8::from(Cmd::NewScanResults) {
                    // Check if it's for our interface
                    let attr_handle = payload.attrs().get_attr_handle();
                    if let Some(ifindex_attr) = attr_handle.get_attribute(u16::from(Attr::Ifindex))
                    {
                        if let Ok(msg_ifindex) = ifindex_attr.get_payload_as::<u32>() {
                            if msg_ifindex == interface_index {
                                // Check timing - ignore stale messages received too quickly
                                if scan_start.elapsed() >= std::time::Duration::from_millis(1000) {
                                    // This is a fresh NewScanResults for our interface
                                    return Ok(());
                                }
                                // Else: received too soon, probably stale - keep waiting
                            }
                        }
                    }
                }
            }
        }
    }

    Err(Error::AlreadyScanning)
}
