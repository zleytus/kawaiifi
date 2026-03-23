use kawaiifi::Interface;

pub struct InterfaceList(Vec<Interface>);

/// Returns all available wireless interfaces as an opaque list.
/// The caller must free the returned list with `kawaiifi_interface_list_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interfaces() -> Box<InterfaceList> {
    Box::new(InterfaceList(kawaiifi::interfaces()))
}

/// Returns the number of interfaces in the list, or 0 if `list` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_list_count(list: Option<&InterfaceList>) -> usize {
    list.map(|l| l.0.len()).unwrap_or(0)
}

/// Returns a borrowed pointer to the interface at `index`, or null if out of bounds or `list` is null.
/// The pointer is valid for the lifetime of the list. Do NOT free it with `kawaiifi_interface_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_list_get(
    list: Option<&InterfaceList>,
    index: usize,
) -> *const Interface {
    list.and_then(|l| l.0.get(index))
        .map(|i| i as *const Interface)
        .unwrap_or(std::ptr::null())
}

/// Frees an interface list returned by `kawaiifi_interfaces`. Does nothing if `list` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_list_free(list: Option<Box<InterfaceList>>) {
    drop(list);
}

/// Returns the default wireless interface, or null if none is found.
/// The caller must free the returned interface with `kawaiifi_interface_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_default_interface() -> Option<Box<Interface>> {
    kawaiifi::default_interface().map(Box::new)
}

/// Frees an interface returned by `kawaiifi_default_interface`. Does nothing if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_free(interface: Option<&mut Interface>) {
    if let Some(interface) = interface {
        drop(unsafe { Box::from_raw(interface) });
    }
}
