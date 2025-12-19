use std::collections::HashMap;

use zbus::{proxy, zvariant};

use super::{NetworkManagerProxy, NetworkManagerProxyBlocking};
use crate::scan::Error;

#[proxy(
    interface = "org.freedesktop.NetworkManager.Device.Wireless",
    default_service = "org.freedesktop.NetworkManager"
)]
pub(crate) trait WirelessDevice {
    async fn request_scan(&self, options: HashMap<&str, zvariant::Value<'_>>) -> zbus::Result<()>;
}

pub(crate) async fn device_path(
    connection: &zbus::Connection,
    interface_name: &str,
) -> Result<zbus::zvariant::OwnedObjectPath, Error> {
    // Create proxy for NetworkManager main interface
    let nm_proxy = NetworkManagerProxy::new(&connection)
        .await
        .map_err(|e| Error::NetworkManager(format!("Failed to create NM proxy: {}", e)))?;

    // Get device path by interface name
    nm_proxy
        .get_device_by_ip_iface(interface_name)
        .await
        .map_err(|e| {
            Error::NetworkManager(format!(
                "Failed to get device path for {}: {}",
                interface_name, e
            ))
        })
}

pub(crate) fn device_path_blocking(
    connection: &zbus::blocking::Connection,
    interface_name: &str,
) -> Result<zbus::zvariant::OwnedObjectPath, Error> {
    // Create proxy for NetworkManager main interface
    let nm_proxy = NetworkManagerProxyBlocking::new(&connection)
        .map_err(|e| Error::NetworkManager(format!("Failed to create NM proxy: {}", e)))?;

    // Get device path by interface name
    nm_proxy
        .get_device_by_ip_iface(interface_name)
        .map_err(|e| {
            Error::NetworkManager(format!(
                "Failed to get device path for {}: {}",
                interface_name, e
            ))
        })
}
