use std::sync::OnceLock;

use oui::OuiDatabase;

const WIRESHARK_MANUF_DATA: &str = include_str!("../../data/resources/manuf");
const KAWAIIFI_MANUF_DATA: &str = include_str!("../../data/resources/manuf.kawaiifi");

static WIRESHARK_OUI_DB: OnceLock<OuiDatabase> = OnceLock::new();
static KAWAIIFI_OUI_DB: OnceLock<OuiDatabase> = OnceLock::new();

fn wireshark_oui_db() -> &'static OuiDatabase {
    WIRESHARK_OUI_DB.get_or_init(|| {
        OuiDatabase::new_from_str(WIRESHARK_MANUF_DATA)
            .expect("Failed to parse embedded OUI database")
    })
}

fn kawaiifi_oui_db() -> &'static OuiDatabase {
    KAWAIIFI_OUI_DB.get_or_init(|| {
        OuiDatabase::new_from_str(KAWAIIFI_MANUF_DATA)
            .expect("Failed to parse embedded OUI database")
    })
}

pub(super) fn lookup_vendor(mac: &[u8]) -> Option<String> {
    let mac_str = mac
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<_>>()
        .join(":");

    // First look in IEEE OUI database
    if let Some(organization) =
        oui_data::lookup(&mac_str).map(|oui_data| oui_data.organization().to_string())
    {
        return Some(organization);
    }

    // Then try Wireshark's database
    if let Some(name) = wireshark_oui_db()
        .query_by_str(&mac_str)
        .ok()
        .flatten()
        .map(|entry| entry.name_long.unwrap_or(entry.name_short))
    {
        return Some(name);
    }

    // Then try kawaiifi's database
    if let Some(name) = kawaiifi_oui_db()
        .query_by_str(&mac_str)
        .ok()
        .flatten()
        .map(|entry| entry.name_long.unwrap_or(entry.name_short))
    {
        return Some(name);
    }

    None
}
