
/// ADS Read Write
#[derive(Debug, PartialEq)]
pub struct AdsReadWriteResponse {
    result: u32,
    length: u32,
    data: [u8]
}

/// ADS Write Control
#[derive(Debug, PartialEq, Clone)]
pub struct AdsWriteControlResponse {
    result: u32
}


/// ADS Add Device Notification
#[derive(Debug, PartialEq, Clone)]
pub struct AdsAddDeviceNotificationResponse {
    result: u32,
    notification_handle: u32
}


/// ADS Delete Device Notification
#[derive(Debug, PartialEq, Clone)]
pub struct AdsDeleteDeviceNotificationResponse {
    result: u32
}

/// ADS Read
#[derive(Debug, PartialEq)]
pub struct AdsReadResponse {
    result: u32,
    length: u32,
    data: [u8]
}

/// ADS Write
#[derive(Debug, PartialEq, Clone)]
pub struct AdsWriteResponse {
    result: u32
}

/// ADS Read State
#[derive(Debug, PartialEq, Clone)]
pub struct AdsReadStateResponse {
    result: u32,
    ads_state: u16,
    device_state: u16
}

/// ADS Read Device Info
#[derive(Debug, PartialEq, Clone)]
pub struct AdsReadDeviceInfoResponse {
    result: u32,
    major_version: u8,
    minor_version: u8,
    version_build: u16,
    device_name: [u8; 16]
}