mod backend;
mod error;
mod network_manager;
mod nl80211;

pub use backend::Backend;
pub use error::Error;
pub(crate) use nl80211::{scan_results, scan_results_blocking};

use crate::Interface;

pub(crate) async fn scan(interface: &Interface, scan_backend: Backend) -> Result<(), Error> {
    match scan_backend {
        Backend::Nl80211 => nl80211::scan(interface).await,
        Backend::NetworkManager => network_manager::scan(interface).await,
    }
}

pub(crate) fn scan_blocking(interface: &Interface, scan_backend: Backend) -> Result<(), Error> {
    match scan_backend {
        Backend::Nl80211 => nl80211::scan_blocking(interface),
        Backend::NetworkManager => network_manager::scan_blocking(interface),
    }
}
