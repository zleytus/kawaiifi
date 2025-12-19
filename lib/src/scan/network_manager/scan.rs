use std::collections::HashMap;

use super::wireless_device::{
    WirelessDeviceProxy, WirelessDeviceProxyBlocking, device_path, device_path_blocking,
};
use crate::{Interface, scan::Error};

pub(crate) async fn scan(interface: &Interface) -> Result<(), Error> {
    // Connect to D-Bus
    let connection = zbus::Connection::system()
        .await
        .map_err(|e| Error::NetworkManager(e.to_string()))?;

    // Get NM device for this interface
    let device_path = device_path(&connection, interface.name()).await?;

    let proxy = WirelessDeviceProxy::builder(&connection)
        .path(device_path)
        .map_err(|e| Error::NetworkManager(e.to_string()))?
        .build()
        .await
        .map_err(|e| Error::NetworkManager(e.to_string()))?;

    // Record timestamp and trigger scan via NetworkManager
    let scan_start = std::time::Instant::now();
    proxy
        .request_scan(HashMap::new())
        .await
        .map_err(|e| Error::NetworkManager(e.to_string()))?;

    crate::scan::nl80211::wait_for_new_scan_results(interface.index(), scan_start).await
}

pub(crate) fn scan_blocking(interface: &Interface) -> Result<(), Error> {
    // Connect to D-Bus
    let connection =
        zbus::blocking::Connection::system().map_err(|e| Error::NetworkManager(e.to_string()))?;

    // Get NM device for this interface
    let device_path = device_path_blocking(&connection, interface.name())?;

    let proxy = WirelessDeviceProxyBlocking::builder(&connection)
        .path(device_path)
        .map_err(|e| Error::NetworkManager(e.to_string()))?
        .build()
        .map_err(|e| Error::NetworkManager(e.to_string()))?;

    // Record timestamp and trigger scan via NetworkManager
    let scan_start = std::time::Instant::now();
    proxy
        .request_scan(HashMap::new())
        .map_err(|e| Error::NetworkManager(e.to_string()))?;

    crate::scan::nl80211::wait_for_new_scan_results_blocking(interface.index(), scan_start)
}
