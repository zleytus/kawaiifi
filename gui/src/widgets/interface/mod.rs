mod list;
mod toggle;

pub use list::{InterfaceList, InterfaceRefreshResult};
pub use toggle::InterfaceToggle;

fn icon_name(bus_type: kawaiifi::BusType) -> &'static str {
    match bus_type {
        kawaiifi::BusType::Pci => "pci-card-symbolic",
        kawaiifi::BusType::Usb => "drive-harddisk-usb-symbolic",
        _ => "network-wireless-symbolic",
    }
}
