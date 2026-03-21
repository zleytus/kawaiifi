use std::collections::HashMap;

use zbus::{proxy, zvariant};

use crate::{Interface, scan::Error};

#[proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
pub(crate) trait NetworkManager {
    async fn get_device_by_ip_iface(&self, iface: &str) -> zbus::Result<zvariant::OwnedObjectPath>;
}

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
    let nm_proxy = NetworkManagerProxy::new(&connection).await?;

    // Get device path by interface name
    Ok(nm_proxy.get_device_by_ip_iface(interface_name).await?)
}

#[tracing::instrument(skip(interface), fields(interface = %interface.name()))]
pub(crate) async fn trigger_scan(interface: &Interface) -> Result<(), Error> {
    tracing::debug!("Triggering NetworkManager scan");
    let connection = zbus::Connection::system().await?;

    let device_path = device_path(&connection, interface.name()).await?;

    let proxy = WirelessDeviceProxy::builder(&connection)
        .path(device_path)?
        .build()
        .await?;

    proxy.request_scan(HashMap::new()).await?;

    Ok(())
}
