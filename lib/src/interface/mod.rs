mod interface;

pub use interface::{BusType, Interface};

use neli::{
    consts::{nl::NlmF, socket::NlFamily},
    genl::{Genlmsghdr, GenlmsghdrBuilder},
    nl::NlPayload,
    router::synchronous::NlRouter,
    utils::Groups,
};

use crate::{
    nl80211::{Attr, Cmd, NL80211_FAMILY_NAME},
    scan::Error,
};

pub fn default_interface() -> Option<Interface> {
    interfaces().into_iter().next()
}

pub fn interfaces() -> Vec<Interface> {
    match interfaces_internal() {
        Ok(interfaces) => interfaces,
        Err(e) => {
            eprintln!("Failed to get interfaces: {:?}", e);
            Vec::new()
        }
    }
}

fn interfaces_internal() -> Result<Vec<Interface>, Error> {
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
