use neli::{
    consts::nl::NlmF,
    genl::{AttrTypeBuilder, Genlmsghdr, GenlmsghdrBuilder, NlattrBuilder},
    nl::NlPayload,
    router::asynchronous,
    types::{Buffer, GenlBuffer},
};

use crate::nl80211::{Attr, Cmd, NL80211_FAMILY_NAME};
use crate::{Interface, scan::Error};

#[tracing::instrument(skip(socket, interface), fields(interface = %interface.name(), ifindex = interface.index()))]
pub(crate) async fn trigger_scan(
    socket: &asynchronous::NlRouter,
    interface: &Interface,
) -> Result<(), Error> {
    tracing::debug!("Triggering nl80211 scan");
    // Resolve the nl80211 family ID
    let family_id = socket.resolve_genl_family(NL80211_FAMILY_NAME).await?;

    // Build the netlink attribute specifying which interface to query
    let ifindex_attr = NlattrBuilder::default()
        .nla_type(AttrTypeBuilder::default().nla_type(Attr::Ifindex).build()?)
        .nla_payload(interface.index())
        .build()?;

    // Empty SSID list for wildcard/broadcast active scan
    let scan_ssids_attr = NlattrBuilder::default()
        .nla_type(
            AttrTypeBuilder::default()
                .nla_type(Attr::ScanSsids)
                .build()?,
        )
        .nla_payload(GenlBuffer::<u16, Buffer>::new()) // Empty nested attributes for wildcard
        .build()?;

    let attrs = vec![ifindex_attr, scan_ssids_attr]
        .into_iter()
        .collect::<GenlBuffer<_, _>>();

    // Build and send the NL80211_CMD_TRIGGER_SCAN request
    let genlmsghdr = GenlmsghdrBuilder::default()
        .cmd(Cmd::TriggerScan)
        .attrs(attrs)
        .version(1)
        .build()?;

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
        tracing::error!("Scan request not acknowledged, permission denied");
        return Err(Error::PermissionDenied);
    }

    Ok(())
}
