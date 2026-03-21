use std::ptr::{null, null_mut};
use std::sync::{Condvar, Mutex};

use chrono::Utc;
use tokio::time::timeout;
use windows_sys::{
    Win32::NetworkManagement::WiFi::{
        L2_NOTIFICATION_DATA, WLAN_BSS_LIST, WLAN_INTERFACE_INFO, WLAN_INTERFACE_INFO_LIST,
        WLAN_NOTIFICATION_SOURCE_ACM, WLAN_NOTIFICATION_SOURCE_NONE, WlanEnumInterfaces,
        WlanFreeMemory, WlanGetNetworkBssList, WlanOpenHandle, WlanRegisterNotification, WlanScan,
        dot11_BSS_type_any, wlan_notification_acm_scan_complete, wlan_notification_acm_scan_fail,
    },
    core::GUID,
};

use crate::{Bss, Scan, scan};

const SCAN_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

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
        return Vec::new();
    }

    let interfaces = unsafe {
        let count = (*interface_list).dwNumberOfItems as usize;
        let first = (*interface_list).InterfaceInfo.as_ptr();
        (0..count).map(|i| Interface::from(*first.add(i))).collect()
    };

    unsafe { WlanFreeMemory(interface_list as *const _) };

    interfaces
}

pub struct Interface {
    wlan_interface_info: WLAN_INTERFACE_INFO,
}

impl Interface {
    pub fn guid(&self) -> GUID {
        self.wlan_interface_info.InterfaceGuid
    }

    pub fn description(&self) -> String {
        String::from_utf16_lossy(&self.wlan_interface_info.strInterfaceDescription)
    }

    #[tracing::instrument(skip(self), fields(interface = %self.description()))]
    pub async fn scan(&self) -> Result<Scan, scan::Error> {
        let mut version = 0;
        let mut handle = null_mut();
        if unsafe { WlanOpenHandle(2, null(), &mut version, &mut handle) } != 0 {
            return Err(std::io::Error::other("WlanOpenHandle failed").into());
        }

        let notify = Box::new(tokio::sync::Notify::new());
        let notify_ptr = Box::into_raw(notify);

        unsafe extern "system" fn on_notification(
            data: *mut L2_NOTIFICATION_DATA,
            context: *mut core::ffi::c_void,
        ) {
            let code = unsafe { (*data).NotificationCode as i32 };
            if code == wlan_notification_acm_scan_complete
                || code == wlan_notification_acm_scan_fail
            {
                let notify = unsafe { &*(context as *const tokio::sync::Notify) };
                notify.notify_one();
            }
        }

        let start_time = Utc::now();

        unsafe {
            WlanRegisterNotification(
                handle,
                WLAN_NOTIFICATION_SOURCE_ACM,
                1, // ignore duplicates
                Some(on_notification),
                notify_ptr as *const _,
                null(),
                null_mut(),
            );
            WlanScan(handle, &self.guid(), null(), null(), null());
        }

        let notify = unsafe { &*notify_ptr };
        timeout(SCAN_TIMEOUT, notify.notified()).await.ok();

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
            drop(Box::from_raw(notify_ptr));
        }

        let bss_list = self.cached_scan_results_blocking()?;
        let end_time = Utc::now();
        Ok(Scan::new(bss_list, start_time, end_time))
    }

    #[tracing::instrument(skip(self), fields(interface = %self.description()))]
    pub fn scan_blocking(&self) -> Result<Scan, scan::Error> {
        let mut version = 0;
        let mut handle = null_mut();
        if unsafe { WlanOpenHandle(2, null(), &mut version, &mut handle) } != 0 {
            return Err(std::io::Error::other("WlanOpenHandle failed").into());
        }

        // Allocate the sync primitive on the heap and pass a raw pointer as the
        // callback context. The callback borrows it; we own it and drop it after
        // unregistering notifications.
        let sync = Box::new((Mutex::new(false), Condvar::new()));
        let sync_ptr = Box::into_raw(sync);

        unsafe extern "system" fn on_notification(
            data: *mut L2_NOTIFICATION_DATA,
            context: *mut core::ffi::c_void,
        ) {
            let code = unsafe { (*data).NotificationCode as i32 };
            if code == wlan_notification_acm_scan_complete
                || code == wlan_notification_acm_scan_fail
            {
                let pair = unsafe { &*(context as *const (Mutex<bool>, Condvar)) };
                *pair.0.lock().unwrap() = true;
                pair.1.notify_one();
            }
        }

        let start_time = Utc::now();

        unsafe {
            WlanRegisterNotification(
                handle,
                WLAN_NOTIFICATION_SOURCE_ACM,
                1, // ignore duplicates
                Some(on_notification),
                sync_ptr as *const _,
                null(),
                null_mut(),
            );
            WlanScan(handle, &self.guid(), null(), null(), null());
        }

        // Block until the scan completes (or times out)
        let sync = unsafe { &*sync_ptr };
        sync.1
            .wait_timeout_while(sync.0.lock().unwrap(), SCAN_TIMEOUT, |done| !*done)
            .ok();

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
        }

        let bss_list = self.cached_scan_results_blocking()?;
        let end_time = Utc::now();
        Ok(Scan::new(bss_list, start_time, end_time))
    }

    pub fn cached_scan_results_blocking(&self) -> Result<Vec<Bss>, scan::Error> {
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
            return Ok(Vec::new());
        }

        // Entries are packed end-to-end with IE data appended after each fixed struct,
        // so we walk using variable strides (ulIeOffset + ulIeSize) rather than a simple
        // array index.
        let bss_list = unsafe {
            let count = (*bss_list_ptr).dwNumberOfItems as usize;
            let mut entry_ptr = (*bss_list_ptr).wlanBssEntries.as_ptr();
            let mut results = Vec::with_capacity(count);
            for _ in 0..count {
                results.push(Bss::from_wlan_entry(entry_ptr));
                let stride = (*entry_ptr).ulIeOffset as usize + (*entry_ptr).ulIeSize as usize;
                entry_ptr = (entry_ptr as *const u8).add(stride) as *const _;
            }
            results
        };

        unsafe { WlanFreeMemory(bss_list_ptr as *const _) };

        Ok(bss_list)
    }
}

impl From<WLAN_INTERFACE_INFO> for Interface {
    fn from(wlan_interface_info: WLAN_INTERFACE_INFO) -> Self {
        Self {
            wlan_interface_info,
        }
    }
}
