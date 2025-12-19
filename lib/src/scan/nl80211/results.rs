use std::iter::once;

use neli::{
    consts::{nl::NlmF, socket::NlFamily},
    genl::{AttrTypeBuilder, Genlmsghdr, GenlmsghdrBuilder, NlattrBuilder},
    nl::{NlPayload, Nlmsghdr},
    router::{asynchronous, synchronous},
    types::GenlBuffer,
    utils::Groups,
};

use crate::nl80211::{Attr, Cmd, NL80211_FAMILY_NAME};
use crate::{Bss, Interface, scan::Error};

pub(crate) async fn scan_results(interface: &Interface) -> Result<Vec<Bss>, Error> {
    // Connect to the generic netlink socket
    let (socket, _) =
        asynchronous::NlRouter::connect(NlFamily::Generic, None, Groups::empty()).await?;

    // Resolve the nl80211 family ID
    let family_id = socket.resolve_genl_family(NL80211_FAMILY_NAME).await?;

    // Build the netlink attribute specifying which interface to query
    let ifindex_attr = NlattrBuilder::default()
        .nla_type(AttrTypeBuilder::default().nla_type(Attr::Ifindex).build()?)
        .nla_payload(interface.index())
        .build()?;

    let attrs = once(ifindex_attr).collect::<GenlBuffer<_, _>>();

    // Build and send the NL80211_CMD_GET_SCAN request
    let genlmsghdr = GenlmsghdrBuilder::default()
        .cmd(Cmd::GetScan)
        .attrs(attrs)
        .version(1)
        .build()?;

    let mut responses = socket
        .send::<_, _, u16, Genlmsghdr<Cmd, Attr>>(
            family_id,
            NlmF::DUMP | NlmF::ACK,
            NlPayload::Payload(genlmsghdr),
        )
        .await?;

    // Process responses and extract BSS information
    let mut scan_results = Vec::new();
    while let Some(response) = responses.next().await {
        if let Ok(response) = response {
            if let Some(bss) = extract_bss_from_message(response) {
                scan_results.push(bss);
            }
        }
    }
    Ok(scan_results)
}

pub(crate) fn scan_results_blocking(interface: &Interface) -> Result<Vec<Bss>, Error> {
    // Connect to the generic netlink socket
    let (socket, _) = synchronous::NlRouter::connect(NlFamily::Generic, None, Groups::empty())?;

    // Resolve the nl80211 family ID
    let family_id = socket.resolve_genl_family(NL80211_FAMILY_NAME)?;

    // Build the netlink attribute specifying which interface to query
    let ifindex_attr = NlattrBuilder::default()
        .nla_type(AttrTypeBuilder::default().nla_type(Attr::Ifindex).build()?)
        .nla_payload(interface.index())
        .build()?;

    let attrs = once(ifindex_attr).collect::<GenlBuffer<_, _>>();

    // Build and send the NL80211_CMD_GET_SCAN request
    let genlmsghdr = GenlmsghdrBuilder::default()
        .cmd(Cmd::GetScan)
        .attrs(attrs)
        .version(1)
        .build()?;

    let responses = socket.send::<_, _, u16, Genlmsghdr<Cmd, Attr>>(
        family_id,
        NlmF::DUMP | NlmF::ACK,
        NlPayload::Payload(genlmsghdr),
    )?;

    // Process responses and extract BSS information
    Ok(responses
        .into_iter()
        .filter_map(|msghdr| msghdr.ok())
        .filter_map(|msghdr| extract_bss_from_message(msghdr))
        .collect())
}

/// Extract BSS information from a netlink message
fn extract_bss_from_message(msghdr: Nlmsghdr<u16, Genlmsghdr<Cmd, Attr>>) -> Option<Bss> {
    // Get the payload from the message
    let payload = msghdr.get_payload()?;

    // Only process NL80211_CMD_NEW_SCAN_RESULTS messages
    if *payload.cmd() != Cmd::NewScanResults {
        return None;
    }

    // Get the nested BSS attributes
    let attr_handle = payload.attrs().get_attr_handle();
    let bss_attrs = attr_handle.get_nested_attributes(Attr::Bss).ok()?;

    // Parse the attributes into a Bss struct
    Bss::from_attrs(bss_attrs.iter()).ok()
}
