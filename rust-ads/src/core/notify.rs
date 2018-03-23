use chrono::Duration;
use core::ads::{AdsError, AmsProxy, Result, VirtualConnection};
use core::connection::AmsConnection;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct AdsNotificationHeader {
    /// 64bit value representing the number of 100-nanosecond intervals since January 1, 1601 (UTC)
    time_stamp: u64,
    notification: u32,
    callback_size: u32,
}

#[derive(Debug, PartialEq)]
pub struct AdsNotificationSample {
    notification_handle: u32,
    sample_size: u32,
    data: [u8],
}

#[derive(Debug)]
pub struct Notification {
    conn: VirtualConnection,
}

#[derive(Debug)]
pub struct NotificationDispatcher<'a, T: AmsProxy + 'a> {
    ams_proxy: &'a T,
    notifications: HashMap<u32, Arc<RwLock<Notification>>>,
}

impl<'a, T: AmsProxy> NotificationDispatcher<'a, T> {
    pub fn erase(&mut self, notify: u32, timeout: Option<Duration>) {
        unimplemented!()
    }

    pub fn find_notification(&self, notify: u32) -> Result<&RwLock<Notification>> {
        self.notifications
            .get(&notify)
            .map(|x| &**x)
            .ok_or(AdsError::NotFound)
    }
}

impl<'a, T: AmsProxy> Drop for NotificationDispatcher<'a, T> {
    fn drop(&mut self) {
        // TODO release resources
    }
}
