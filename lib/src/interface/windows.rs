use std::io;
use std::ptr::{null, null_mut};
use std::sync::{Condvar, Mutex};

use chrono::Utc;
use tokio::time::timeout;
use windows_sys::{
    Win32::NetworkManagement::WiFi::{
        L2_NOTIFICATION_DATA, WLAN_BSS_LIST, WLAN_INTERFACE_INFO, WLAN_INTERFACE_INFO_LIST,
        WLAN_NOTIFICATION_SOURCE_ACM, WLAN_NOTIFICATION_SOURCE_NONE, WlanCloseHandle,
        WlanEnumInterfaces, WlanFreeMemory, WlanGetNetworkBssList, WlanOpenHandle,
        WlanRegisterNotification, WlanScan, dot11_BSS_type_any,
        wlan_notification_acm_scan_complete, wlan_notification_acm_scan_fail,
    },
    core::GUID,
};

use crate::{Bss, Scan, ScanError};

const SCAN_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

fn wlan_error(operation: &str, code: u32) -> io::Error {
    io::Error::other(format!("{operation} failed with error code {code}"))
}

pub(super) fn interfaces() -> Vec<Interface> {
    let mut version = 0;
    let mut handle = null_mut();
    if unsafe { WlanOpenHandle(2, null(), &mut version, &mut handle) } != 0 {
        return Vec::new();
    }

    let mut interface_list: *mut WLAN_INTERFACE_INFO_LIST = null_mut();
    if unsafe { WlanEnumInterfaces(handle, null(), &mut interface_list) } != 0
        || interface_list.is_null()
    {
        unsafe { WlanCloseHandle(handle, null_mut()) };
        return Vec::new();
    }

    let interfaces = unsafe {
        let count = (*interface_list).dwNumberOfItems as usize;
        let first = (*interface_list).InterfaceInfo.as_ptr();
        (0..count).map(|i| Interface::from(*first.add(i))).collect()
    };

    unsafe { WlanFreeMemory(interface_list as *const _) };
    unsafe { WlanCloseHandle(handle, null_mut()) };

    interfaces
}

/// A Wi-Fi interface obtained from Native Wifi.
pub struct Interface {
    wlan_interface_info: WLAN_INTERFACE_INFO,
}

impl Interface {
    /// The GUID identifying this interface to Native Wifi.
    pub fn guid(&self) -> GUID {
        self.wlan_interface_info.InterfaceGuid
    }

    /// The human-readable description of the interface.
    pub fn description(&self) -> String {
        String::from_utf16_lossy(&self.wlan_interface_info.strInterfaceDescription)
            .trim_end_matches('\0')
            .to_string()
    }

    /// Triggers a new scan and returns the results.
    #[tracing::instrument(skip(self), fields(interface = %self.description()))]
    pub async fn scan(&self) -> Result<Scan, ScanError> {
        let mut version = 0;
        let mut handle = null_mut();
        if unsafe { WlanOpenHandle(2, null(), &mut version, &mut handle) } != 0 {
            return Err(std::io::Error::other("WlanOpenHandle failed").into());
        }

        let scan_state = Box::new((Mutex::new(None), tokio::sync::Notify::new()));
        let scan_state_ptr = Box::into_raw(scan_state);

        unsafe extern "system" fn on_notification(
            data: *mut L2_NOTIFICATION_DATA,
            context: *mut core::ffi::c_void,
        ) {
            if data.is_null() || context.is_null() {
                return;
            }
            let code = unsafe { (*data).NotificationCode as i32 };
            if code == wlan_notification_acm_scan_complete
                || code == wlan_notification_acm_scan_fail
            {
                let (notification_code, notify) =
                    unsafe { &*(context as *const (Mutex<Option<i32>>, tokio::sync::Notify)) };
                if let Ok(mut notification_code) = notification_code.lock() {
                    notification_code.replace(code);
                    notify.notify_one();
                }
            }
        }

        let start_time = Utc::now();

        let result = unsafe {
            WlanRegisterNotification(
                handle,
                WLAN_NOTIFICATION_SOURCE_ACM,
                1, // ignore duplicates
                Some(on_notification),
                scan_state_ptr as *const _,
                null(),
                null_mut(),
            )
        };
        if result != 0 {
            unsafe {
                drop(Box::from_raw(scan_state_ptr));
                WlanCloseHandle(handle, null_mut());
            }
            return Err(wlan_error("WlanRegisterNotification", result).into());
        }

        let result = unsafe { WlanScan(handle, &self.guid(), null(), null(), null()) };
        if result != 0 {
            unsafe {
                WlanRegisterNotification(
                    handle,
                    WLAN_NOTIFICATION_SOURCE_NONE,
                    0,
                    None,
                    null(),
                    null(),
                    null_mut(),
                );
                drop(Box::from_raw(scan_state_ptr));
                WlanCloseHandle(handle, null_mut());
            }
            return Err(wlan_error("WlanScan", result).into());
        }

        // SAFETY: scan_state_ptr is valid until we call WlanRegisterNotification(NONE)
        // below, which unregisters the callback before we drop the Box. The callback
        // cannot fire after unregistration, so there is no aliasing after the drop.
        let scan_state = unsafe { &*scan_state_ptr };
        let timed_out = timeout(SCAN_TIMEOUT, scan_state.1.notified())
            .await
            .is_err();
        let notification_code = scan_state
            .0
            .lock()
            .map_err(|_| io::Error::other("scan notification mutex poisoned"))
            .map(|notification_code| *notification_code);

        unsafe {
            WlanRegisterNotification(
                handle,
                WLAN_NOTIFICATION_SOURCE_NONE,
                0,
                None,
                null(),
                null(),
                null_mut(),
            );
            drop(Box::from_raw(scan_state_ptr));
            WlanCloseHandle(handle, null_mut());
        }

        let notification_code = notification_code?;
        if timed_out {
            return Err(io::Error::new(io::ErrorKind::TimedOut, "Scanning timed out").into());
        }
        if notification_code == Some(wlan_notification_acm_scan_fail) {
            return Err(io::Error::other("WlanScan failed").into());
        }

        let bss_list = self.cached_scan_results_blocking()?;
        let end_time = Utc::now();
        Ok(Scan::new(bss_list, start_time, end_time))
    }

    /// Triggers a new scan and returns the results, blocking the current thread.
    #[tracing::instrument(skip(self), fields(interface = %self.description()))]
    pub fn scan_blocking(&self) -> Result<Scan, ScanError> {
        let mut version = 0;
        let mut handle = null_mut();
        if unsafe { WlanOpenHandle(2, null(), &mut version, &mut handle) } != 0 {
            return Err(std::io::Error::other("WlanOpenHandle failed").into());
        }

        // Allocate the sync primitive on the heap and pass a raw pointer as the
        // callback context. The callback borrows it; we own it and drop it after
        // unregistering notifications.
        let sync = Box::new((Mutex::new(None), Condvar::new()));
        let sync_ptr = Box::into_raw(sync);

        unsafe extern "system" fn on_notification(
            data: *mut L2_NOTIFICATION_DATA,
            context: *mut core::ffi::c_void,
        ) {
            if data.is_null() || context.is_null() {
                return;
            }
            let code = unsafe { (*data).NotificationCode as i32 };
            if code == wlan_notification_acm_scan_complete
                || code == wlan_notification_acm_scan_fail
            {
                let pair = unsafe { &*(context as *const (Mutex<Option<i32>>, Condvar)) };
                if let Ok(mut notification_code) = pair.0.lock() {
                    notification_code.replace(code);
                    pair.1.notify_one();
                }
            }
        }

        let start_time = Utc::now();

        let result = unsafe {
            WlanRegisterNotification(
                handle,
                WLAN_NOTIFICATION_SOURCE_ACM,
                1, // ignore duplicates
                Some(on_notification),
                sync_ptr as *const _,
                null(),
                null_mut(),
            )
        };
        if result != 0 {
            unsafe {
                drop(Box::from_raw(sync_ptr));
                WlanCloseHandle(handle, null_mut());
            }
            return Err(wlan_error("WlanRegisterNotification", result).into());
        }

        let result = unsafe { WlanScan(handle, &self.guid(), null(), null(), null()) };
        if result != 0 {
            unsafe {
                WlanRegisterNotification(
                    handle,
                    WLAN_NOTIFICATION_SOURCE_NONE,
                    0,
                    None,
                    null(),
                    null(),
                    null_mut(),
                );
                drop(Box::from_raw(sync_ptr));
                WlanCloseHandle(handle, null_mut());
            }
            return Err(wlan_error("WlanScan", result).into());
        }

        // Block until the scan completes (or times out)
        let sync = unsafe { &*sync_ptr };
        let wait_result = sync
            .0
            .lock()
            .map_err(|_| io::Error::other("scan notification mutex poisoned"))
            .and_then(|notification_code| {
                sync.1
                    .wait_timeout_while(notification_code, SCAN_TIMEOUT, |code| code.is_none())
                    .map_err(|_| io::Error::other("scan notification mutex poisoned"))
                    .map(|(notification_code, wait_result)| {
                        (*notification_code, wait_result.timed_out())
                    })
            });

        // Unregister before dropping the sync primitive so the callback can't fire after the drop
        unsafe {
            WlanRegisterNotification(
                handle,
                WLAN_NOTIFICATION_SOURCE_NONE,
                0,
                None,
                null(),
                null(),
                null_mut(),
            );
            drop(Box::from_raw(sync_ptr));
            WlanCloseHandle(handle, null_mut());
        }

        let (notification_code, timed_out) = wait_result?;
        if timed_out {
            return Err(io::Error::new(io::ErrorKind::TimedOut, "Scanning timed out").into());
        }
        if notification_code == Some(wlan_notification_acm_scan_fail) {
            return Err(io::Error::other("WlanScan failed").into());
        }

        let bss_list = self.cached_scan_results_blocking()?;
        let end_time = Utc::now();
        Ok(Scan::new(bss_list, start_time, end_time))
    }

    /// Returns the most recently cached scan results without triggering a new scan.
    ///
    /// Native Wifi exposes cached scan results synchronously, so this async method
    /// blocks the current task while reading them.
    pub async fn cached_scan_results(&self) -> Result<Vec<Bss>, ScanError> {
        self.cached_scan_results_blocking()
    }

    /// Returns the most recently cached scan results without triggering a new scan, blocking the current thread.
    pub fn cached_scan_results_blocking(&self) -> Result<Vec<Bss>, ScanError> {
        let mut version = 0;
        let mut handle = null_mut();
        if unsafe { WlanOpenHandle(2, null(), &mut version, &mut handle) } != 0 {
            return Err(std::io::Error::other("WlanOpenHandle failed").into());
        }

        let mut bss_list_ptr: *mut WLAN_BSS_LIST = null_mut();
        if unsafe {
            WlanGetNetworkBssList(
                handle,
                &self.guid(),
                null(),
                dot11_BSS_type_any,
                0,
                null(),
                &mut bss_list_ptr,
            )
        } != 0
            || bss_list_ptr.is_null()
        {
            unsafe { WlanCloseHandle(handle, null_mut()) };
            return Ok(Vec::new());
        }

        // Windows packs all fixed WLAN_BSS_ENTRY structs contiguously, followed by a
        // trailing region containing all IE blobs. Each entry's ulIeOffset is the offset
        // to its own IE data relative to the entry pointer (pointing into that trailing
        // region). The stride between fixed structs is therefore sizeof(WLAN_BSS_ENTRY).
        let bss_list = unsafe {
            let count = (*bss_list_ptr).dwNumberOfItems as usize;
            let mut entry_ptr = (*bss_list_ptr).wlanBssEntries.as_ptr();
            let mut results = Vec::with_capacity(count);
            for _ in 0..count {
                results.push(Bss::from_wlan_entry(entry_ptr));
                entry_ptr = entry_ptr.add(1);
            }
            results
        };

        unsafe { WlanFreeMemory(bss_list_ptr as *const _) };
        unsafe { WlanCloseHandle(handle, null_mut()) };

        Ok(bss_list)
    }
}

impl Clone for Interface {
    fn clone(&self) -> Self {
        Self {
            wlan_interface_info: self.wlan_interface_info,
        }
    }
}

impl std::fmt::Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface")
            .field("description", &self.description())
            .finish()
    }
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        let a = self.wlan_interface_info.InterfaceGuid;
        let b = other.wlan_interface_info.InterfaceGuid;
        a.data1 == b.data1 && a.data2 == b.data2 && a.data3 == b.data3 && a.data4 == b.data4
    }
}

impl Eq for Interface {}

impl std::hash::Hash for Interface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let guid = self.wlan_interface_info.InterfaceGuid;
        guid.data1.hash(state);
        guid.data2.hash(state);
        guid.data3.hash(state);
        guid.data4.hash(state);
    }
}

impl From<WLAN_INTERFACE_INFO> for Interface {
    fn from(wlan_interface_info: WLAN_INTERFACE_INFO) -> Self {
        Self {
            wlan_interface_info,
        }
    }
}
