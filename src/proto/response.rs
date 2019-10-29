use crate::proto::DataMessage;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Error, Read};

#[derive(Debug)]
pub enum Response {
    Read(DataMessage),
    Write(u32),
    ReadWrite(DataMessage),
    WriteControl(u32),
    AddDeviceNotification(AddDeviceNotificationResponse),
    DeleteDeviceNotification(u32),
    ReadState(ReadStateResponse),
    ReadDeviceInfo(ReadDeviceInfoResponse),
}

/// ADS Add Device Notification
#[derive(Debug, PartialEq, Clone)]
pub struct AddDeviceNotificationResponse {
    result: u32,
    notification_handle: u32,
}
impl ReadFrom for AddDeviceNotificationResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(Self {
            result: read.read_u32::<LittleEndian>()?,
            notification_handle: read.read_u32::<LittleEndian>()?,
        })
    }
}

/// ADS Read State
#[derive(Debug, PartialEq, Clone)]
pub struct ReadStateResponse {
    result: u32,
    ads_state: u16,
    device_state: u16,
}

impl ReadFrom for ReadStateResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(Self {
            result: read.read_u32::<LittleEndian>()?,
            ads_state: read.read_16::<LittleEndian>()?,
            device_state: read.read_16::<LittleEndian>()?,
        })
    }
}

/// ADS Read Device Info
#[derive(Debug, PartialEq, Clone)]
pub struct ReadDeviceInfoResponse {
    result: u32,
    major_version: u8,
    minor_version: u8,
    version_build: u16,
    device_name: [u8; 16],
}

impl ReadFrom for ReadDeviceInfoResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let result = read.read_u32::<LittleEndian>()?;
        let major_version = read.read_u8()?;
        let minor_version = read.read_u8()?;
        let version_build = read.read_u16::<LittleEndian>()?;
        let mut device_name = [0; 16];
        read.read_exact(&mut device_name)?;
        Ok(Self {
            result,
            major_version,
            minor_version,
            version_build,
            device_name,
        })
    }
}

pub trait ReadFrom: Sized {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self>;
}