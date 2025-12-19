use std::iter::once;

use neli::{
    consts::{nl::NlmF, socket::NlFamily},
    genl::{AttrTypeBuilder, Genlmsghdr, GenlmsghdrBuilder, NlattrBuilder},
    nl::NlPayload,
    router::{asynchronous, synchronous},
    types::GenlBuffer,
    utils::Groups,
};

use crate::nl80211::{Attr, Cmd, NL80211_FAMILY_NAME};
use crate::{Interface, scan::Error};

pub(crate) async fn scan(interface: &Interface) -> Result<(), Error> {
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

    // let requested_ies_attr = NlattrBuilder::default()
    // .nla_type(
    // AttrTypeBuilder::default()
    // .nla_type(Nl80211Attr::Ie)
    // .build()?,
    // )
    // .nla_payload(vec![10, 1, 11])
    // .build()?;

    // let attrs = [ifindex_attr, requested_ies_attr]
    // .into_iter()
    // .collect::<GenlBuffer<_, _>>();
    let attrs = once(ifindex_attr).collect::<GenlBuffer<_, _>>();

    // Build and send the NL80211_CMD_TRIGGER_SCAN request
    let genlmsghdr = GenlmsghdrBuilder::default()
        .cmd(Cmd::TriggerScan)
        .attrs(attrs)
        .version(1)
        .build()?;

    let scan_start = std::time::Instant::now();
    let mut responses = socket
        .send::<_, _, u16, Genlmsghdr<Cmd, Attr>>(
            family_id,
            NlmF::REQUEST | NlmF::ACK,
            NlPayload::Payload(genlmsghdr),
        )
        .await?;

    // Wait for the message we just sent to be acknowledged
    let mut ack_received = false;
    while let Some(msg) = responses.next::<u16, Genlmsghdr<Cmd, Attr>>().await {
        if let Ok(msg) = msg {
            if matches!(msg.nl_payload(), NlPayload::Ack(_)) {
                ack_received = true;
            }
        }
    }

    // If we don't receive an ACK, assume the user was denied permission to scan
    if !ack_received {
        return Err(Error::PermissionDenied);
    }

    super::wait_for_new_scan_results(interface.index(), scan_start).await
}

pub(crate) fn scan_blocking(interface: &Interface) -> Result<(), Error> {
    // Connect to the generic netlink socket
    let (socket, _) = synchronous::NlRouter::connect(NlFamily::Generic, None, Groups::empty())?;

    // Resolve the nl80211 family ID
    let family_id = socket.resolve_genl_family(NL80211_FAMILY_NAME)?;

    // Build the netlink attribute specifying which interface to query
    let ifindex_attr = NlattrBuilder::default()
        .nla_type(AttrTypeBuilder::default().nla_type(Attr::Ifindex).build()?)
        .nla_payload(interface.index())
        .build()?;

    // let requested_ies_attr = NlattrBuilder::default()
    // .nla_type(
    // AttrTypeBuilder::default()
    // .nla_type(Nl80211Attr::Ie)
    // .build()?,
    // )
    // .nla_payload(vec![10, 1, 11])
    // .build()?;

    // let attrs = [ifindex_attr, requested_ies_attr]
    // .into_iter()
    // .collect::<GenlBuffer<_, _>>();
    let attrs = once(ifindex_attr).collect::<GenlBuffer<_, _>>();

    // Build and send the NL80211_CMD_TRIGGER_SCAN request
    let genlmsghdr = GenlmsghdrBuilder::default()
        .cmd(Cmd::TriggerScan)
        .attrs(attrs)
        .version(1)
        .build()?;

    let scan_start = std::time::Instant::now();
    let responses = socket.send::<_, _, u16, Genlmsghdr<Cmd, Attr>>(
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
        return Err(Error::PermissionDenied);
    }

    super::wait_for_new_scan_results_blocking(interface.index(), scan_start)
}
