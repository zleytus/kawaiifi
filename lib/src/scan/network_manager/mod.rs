mod scan;
mod wireless_device;

pub(crate) use scan::*;

use zbus::{proxy, zvariant};

#[proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
pub(crate) trait NetworkManager {
    async fn get_device_by_ip_iface(&self, iface: &str) -> zbus::Result<zvariant::OwnedObjectPath>;
}
