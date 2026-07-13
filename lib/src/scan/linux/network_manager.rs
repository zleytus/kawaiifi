use nmrs::NetworkManager;

use crate::{Interface, ScanError};

#[tracing::instrument(skip(interface), fields(interface = %interface.name(), ifindex = interface.index()))]
pub(crate) async fn trigger_scan(interface: &Interface) -> Result<(), ScanError> {
    tracing::debug!("Triggering NetworkManager scan");
    let nm = NetworkManager::new().await?;
    nm.scan_networks(Some(interface.name())).await?;
    Ok(())
}
