use core::ads::{AdsCommandId, AmsAddress, SizedData};

pub trait AdsCommandPayload {
    type Response;
    fn command_id() -> AdsCommandId;

    fn payload_legnth(&self) -> usize;
}

pub struct AdsRequest<T: AdsCommandPayload> {
    dest_addr: AmsAddress,
    port: u16,
    payload: T,
}

pub trait AmsRequest: SizedData {
    fn index_group(&self) -> &u32;

    fn index_offset(&self) -> &u32;
}

/// ADS Read Write
#[derive(Debug, PartialEq)]
pub struct AdsReadWriteRequest {
    index_group: u32,
    index_offset: u32,
    read_length: u32,
    write_length: u32,
    data: Vec<u8>,
}

impl SizedData for AdsReadWriteRequest {
    fn data_len(&self) -> u32 {
        self.read_length + self.write_length
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn read_write_len(&self) -> Option<(u32, u32)> {
        Some((self.read_length, self.write_length))
    }
}

/// ADS Delete Device Notification
#[derive(Debug, PartialEq, Clone)]
pub struct AdsDeleteDeviceNotificationRequest {
    notification_handle: u32,
}

/// ADS Add Device Notification
#[derive(Debug, PartialEq, Clone)]
pub struct AdsAddDeviceNotificationRequest {
    index_group: u32,
    index_offset: u32,
    length: u32,
    transmission_mode: u32,
    max_delay: u32,
    cycle_time: u32,
    reserved: [u8; 16],
}

/// ADS Write Control
#[derive(Debug, PartialEq)]
pub struct AdsWriteControlRequest {
    ads_state: u16,
    device_state: u16,
    length: u32,
    data: Vec<u8>,
}

/// ADS Write
#[derive(Debug, PartialEq)]
pub struct AdsWriteRequest {
    index_group: u32,
    index_offset: u32,
    data: Vec<u8>,
}

/// ADS Read
#[derive(Debug, PartialEq)]
pub struct AdsReadRequest {
    index_group: u32,
    index_offset: u32,
    length: u32,
}
