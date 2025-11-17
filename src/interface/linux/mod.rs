mod interface;
mod interface_type;
mod nl80211_attr;
mod nl80211_cmd;
mod scan_error;

pub use interface::Interface;
pub use interface_type::InterfaceType;
use nl80211_attr::Nl80211Attr;
use nl80211_cmd::Nl80211Cmd;
pub use scan_error::ScanError;

use std::collections::HashSet;

use neli::{
    consts::{nl::NlmF, socket::NlFamily},
    genl::{Genlmsghdr, GenlmsghdrBuilder},
    nl::NlPayload,
    router::synchronous::NlRouter,
    utils::Groups,
};

const NL80211_FAMILY_NAME: &str = "nl80211";
const SCAN_MULTICAST_NAME: &str = "scan";

pub fn default_interface() -> Option<Interface> {
    interfaces().into_iter().next()
}

pub fn interfaces() -> HashSet<Interface> {
    match interfaces_internal() {
        Ok(interfaces) => interfaces,
        Err(e) => {
            eprintln!("Failed to get interfaces: {:?}", e);
            HashSet::new()
        }
    }
}

fn interfaces_internal() -> Result<HashSet<Interface>, ScanError> {
    // Create a generic netlink socket and resolve nl80211 family
    let (socket, _) = NlRouter::connect(NlFamily::Generic, None, Groups::empty())?;
    let family_id = socket.resolve_genl_family(NL80211_FAMILY_NAME)?;

    // Query system for WiFi interfaces using the 'GetInterface' command
    // No attributes needed because DUMP flag will return all interfaces
    let recv = socket.send::<_, _, u16, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>(
        family_id,
        NlmF::DUMP | NlmF::REQUEST,
        NlPayload::Payload(
            GenlmsghdrBuilder::default()
                .cmd(Nl80211Cmd::GetInterface)
                .version(1)
                .build()?,
        ),
    )?;

    // Receive all responses and attempt to convert each of them into an Interface
    let mut interfaces = HashSet::new();
    for msg in recv {
        let msg = msg?;
        let Some(payload) = msg.get_payload() else {
            continue;
        };
        let attrs = payload.attrs().iter();
        if let Ok(interface) = Interface::from_attrs(attrs) {
            interfaces.insert(interface);
        }
    }

    Ok(interfaces)
}
