use std::{io, iter::once, time::Duration};

use neli::{
    consts::{nl::NlmF, socket::NlFamily},
    genl::{AttrTypeBuilder, Genlmsghdr, GenlmsghdrBuilder, NlattrBuilder},
    nl::NlPayload,
    router::{asynchronous, synchronous},
    types::GenlBuffer,
    utils::Groups,
};
use tokio::{pin, select, time::sleep};

use super::{
    Backend, Error, Scan, ScanCompleted, ScanInternal, ScanTriggered, network_manager, nl80211,
};
use crate::{
    Bss, Interface,
    nl80211::{Attr, Cmd, NL80211_FAMILY_NAME},
};

// Long timeout: waiting for scan to complete (5/6 GHz can take a while)
const SCAN_TIMEOUT: Duration = Duration::from_secs(25);

// Short timeout: waiting to see if another scan will start
const IDLE_TIMEOUT: Duration = Duration::from_secs(2);

const SCAN_MULTICAST_NAME: &str = "scan";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScanState {
    ScanInProgress,     // TriggerScan Received, waiting for NewScanResults
    WaitingForNextScan, // NewScanResults received, waiting for next TriggerScan
}

/// Performs a WiFi scan and waits for results using nl80211 multicast events.
///
/// This function:
/// 1. Subscribes to nl80211 multicast events before triggering the scan
/// 2. Triggers the scan using the specified backend
/// 3. Waits for scan events (TriggerScan and NewScanResults)
/// 4. Collects all scan results, potentially from multiple sub-scans
///
/// # Multiple Scans
///
/// NetworkManager may split a scan into multiple sub-scans (e.g., one per band).
/// This function collects all sub-scans and combines them into a single `Scan`.
///
/// # Timeouts
///
/// - SCAN_TIMEOUT: Max time to wait for an in-progress scan to complete
/// - IDLE_TIMEOUT: Short wait to see if another sub-scan will start
#[tracing::instrument(skip(interface), fields(interface = %interface.name(), ifindex = interface.index()))]
pub(crate) async fn scan(interface: &Interface, backend: Backend) -> Result<Scan, Error> {
    // Subscribe to nl80211 multicast events BEFORE triggering the scan
    // to avoid missing events due to race conditions
    let (socket, mut multicast) =
        asynchronous::NlRouter::connect(NlFamily::Generic, None, Groups::empty()).await?;
    let id = socket
        .resolve_nl_mcast_group(NL80211_FAMILY_NAME, SCAN_MULTICAST_NAME)
        .await?;
    socket.add_mcast_membership(Groups::new_groups(&[id]))?;

    // Trigger scan using the specified backend
    // Both backends broadcast nl80211 events that we'll receive below
    match backend {
        Backend::Nl80211 => nl80211::trigger_scan(&socket, interface).await?,
        Backend::NetworkManager => network_manager::trigger_scan(interface).await?,
    };

    // Track whether we're waiting for a scan to complete or for another scan to start
    let mut state = ScanState::ScanInProgress;

    // Timeout future that we'll reset as scans progress
    let timeout_fut = sleep(SCAN_TIMEOUT);
    pin!(timeout_fut);

    // Store the most recent TriggerScan event to pair with NewScanResults
    let mut last_scan_triggered = None;

    // Collect all scan results (may be multiple sub-scans from NetworkManager)
    let mut scans = Vec::new();

    loop {
        select! {
            // Branch 1: Received a multicast event from nl80211
            Some(msg) = multicast.next::<u16, Genlmsghdr<Cmd, Attr>>() => {
                // Skip malformed messages
                let Ok(msg) = msg else {
                    continue;
                };
                let Some(payload) = msg.get_payload() else {
                    continue;
                };

                match *payload.cmd() {
                    // A scan was triggered (by us or someone else)
                    Cmd::TriggerScan => {
                        // Parse the trigger event
                        let Ok(scan_triggered) = ScanTriggered::try_from(payload) else {
                            continue;
                        };

                        // Ignore scans on other interfaces
                        if scan_triggered.ifindex != interface.index() {
                            continue;
                        };

                        // Store this trigger to pair with the completion event
                        last_scan_triggered = Some(scan_triggered);

                        // Reset to long timeout since a scan is now in progress
                        timeout_fut.set(sleep(SCAN_TIMEOUT));
                        state = ScanState::ScanInProgress;
                    }

                    // A scan completed
                    Cmd::NewScanResults => {
                        // Parse the completion event
                        let Ok(scan_completed) = ScanCompleted::try_from(payload) else {
                            continue;
                        };

                        // Ignore scans on other interfaces
                        if scan_completed.ifindex != interface.index() {
                            continue;
                        }

                        // Fetch the actual BSS results
                        let Ok(bss_list) = scan_results(interface, &socket).await else {
                            continue;
                        };

                        // Pair the trigger and completion events with results
                        if let Some(scan_triggered) = last_scan_triggered.take() {
                            scans.push(ScanInternal {
                                bss_list,
                                scan_triggered,
                                scan_completed,
                            });
                        }

                        // Switch to short timeout to wait for potential next sub-scan
                        // (NetworkManager may trigger multiple scans for different bands)
                        timeout_fut.set(sleep(IDLE_TIMEOUT));
                        state = ScanState::WaitingForNextScan;
                    }

                    // Ignore other nl80211 commands
                    _ => {}
                }
            },

            // Branch 2: Timeout expired
            _ = &mut timeout_fut => {
                match state {
                    // Timeout while waiting for scan to complete
                    ScanState::ScanInProgress => {
                        // If we got some results before timing out, return them
                        // Otherwise, this is a timeout error
                        return if scans.is_empty() {
                            Err(io::Error::new(io::ErrorKind::TimedOut, "Scanning timed out").into())
                        } else {
                            Ok(Scan::new(scans))
                        }
                    }

                    // Timeout while waiting for another sub-scan to start
                    // This means all sub-scans are done - break and return results
                    ScanState::WaitingForNextScan => {
                        break;
                    }
                }
            }
        }
    }

    // All scans completed, combine and return
    Ok(Scan::new(scans))
}

#[tracing::instrument(skip(interface, socket), fields(interface = %interface.name(), ifindex = interface.index()))]
pub(crate) async fn scan_results(
    interface: &Interface,
    socket: &asynchronous::NlRouter,
) -> Result<Vec<Bss>, Error> {
    tracing::debug!("Fetching nl80211 scan results");

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
    while let Some(response) = responses.next::<u16, Genlmsghdr<Cmd, Attr>>().await {
        if let Ok(response) = &response
            && let Some(payload) = response.get_payload()
            && let Ok(bss) = Bss::try_from(payload)
        {
            scan_results.push(bss);
        }
    }

    tracing::debug!(bss_count = scan_results.len(), "Retrieved scan results");
    Ok(scan_results)
}

#[tracing::instrument(skip(interface, socket), fields(interface = %interface.name(), ifindex = interface.index()))]
pub(crate) fn scan_results_blocking(
    interface: &Interface,
    socket: &synchronous::NlRouter,
) -> Result<Vec<Bss>, Error> {
    tracing::debug!("Fetching nl80211 scan results (blocking)");

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
    let scan_results: Vec<Bss> = responses
        .into_iter()
        .filter_map(|msghdr| msghdr.ok())
        .filter_map(|msghdr| Bss::try_from(msghdr.get_payload()?).ok())
        .collect();

    tracing::debug!(bss_count = scan_results.len(), "Retrieved scan results");
    Ok(scan_results)
}
