/// Implementation of BECKHOFF's ADS protocol

use std::result;
use std::convert::Into;
use std::net::{SocketAddrV4, TcpStream, UdpSocket, TcpListener, Ipv4Addr};

#[derive(Debug, PartialEq, Clone)]
pub enum AdsError {
    InvalidAddress
}

pub type Result<T> = result::Result<T, AdsError>;

pub const MAXDATALEN: usize = 8192;

#[repr(packed)]
pub struct AdsTcpHeader {
    reserved: u16,
    length: u32
}

/// The ADS Net ID
/// addresses the transmitter or receiver
/// The AMS Net ID is composed of the TCP/IP of the local computer plus the suffix ".1.1".
/// The AMS Net ID is based on the TCP/IP address, but the relationship is not entirely fixed.
#[derive(Debug, PartialEq, Clone)]
pub struct AmsNetId {
    b: [u8; 6]
}

impl From<[u8; 6]> for AmsNetId {
    fn from(value: [u8; 6]) -> AmsNetId {
        AmsNetId {
            b: value
        }
    }
}

impl AmsNetId {
    pub fn parse(s: &str) -> Result<AmsNetId> {
        let parts: Vec<&str> = s.split(".").collect();
        if parts.len() != 6 {
            return Err(AdsError::InvalidAddress);
        }
        let mut b = [0; 6];
        for (i, p) in parts.iter().enumerate() {
            match p.parse::<u8>() {
                Ok(v) => b[i] = v,
                Err(_) => return Err(AdsError::InvalidAddress)
            }
        }
        // the AmsNetId must end with ".1.1"
        for i in 4..6 {
            if b[i] != 1 {
                return Err(AdsError::InvalidAddress);
            }
        }
        Ok(AmsNetId { b })
    }

    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> AmsNetId {
        AmsNetId {
            b: [a, b, c, d, e, f, ]
        }
    }
}

impl Into<Ipv4Addr> for AmsNetId {
    fn into(self) -> Ipv4Addr {
        Ipv4Addr::from([self.b[0], self.b[1], self.b[2], self.b[3]])
    }
}

impl Into<AmsNetId> for Ipv4Addr {
    fn into(self) -> AmsNetId {
        let o = self.octets();
        AmsNetId::new(o[0], o[1], o[2], o[3], 1, 1)
    }
}

pub trait ToAmsId<'a> {
    fn to_ams_id(&'a self) -> Result<AmsNetId>;
}

impl<'a> ToAmsId<'a> for str {
    fn to_ams_id(&'a self) -> Result<AmsNetId> {
        AmsNetId::parse(self)
    }
}


impl<'a> ToAmsId<'a> for String {
    fn to_ams_id(&'a self) -> Result<AmsNetId> {
        AmsNetId::parse(self.as_ref())
    }
}

///
#[derive(Debug, PartialEq, Clone)]
pub struct AmsHeader {
    target_id: AmsNetId,
    target_port: u16,
    source_id: AmsNetId,
    source_port: u16,
    command_id: u16,
    state_flags: u16,
    data_length: u32,
    // the size of the data in the ADS packet in bytes
    error_code: u32,
    invoke_id: u32
}


/// state flags
pub const SF_ADS_REQ_RESP: u32 = 0x0001;
pub const SF_ADS_COMMAND: u32 = 0x0004;


pub struct AdsPacket {
    ads_tcp_header: AdsTcpHeader,
    // 6 bytes
    ams_header: AmsHeader,
    // 32 bytes
    ads_data: [u8; MAXDATALEN] // contains the data
}

/// ADS Commands

/// Command ids
#[derive(Debug, PartialEq, Clone)]
pub enum AdsCommandId {
    AdsInvalid = 0x0000,
    AdsReadDeviceInfo = 0x0001,
    AdsRead = 0x0002,
    AdsWrite = 0x0003,
    AdsReadState = 0x0004,
    AdsWriteControl = 0x0005,
    AdsAddDeviceNotification = 0x0006,
    AdsDeleteDeviceNotification = 0x0007,
    AdsDeviceNotification = 0x0008,
    AdsReadWrite = 0x0009
}

#[derive(Debug, PartialEq, Clone)]
pub enum AdsPortNumber {
    Logger = 100,
    EventLogger = 110,
    IO = 300,
    AdditionalTask1 = 301,
    AdditionalTask2 = 302,
    NC = 500,
    PlcRuntimeSystem1 = 801,
    PlcRuntimeSystem2 = 811,
    PlcRuntimeSystem3 = 821,
    PlcRuntimeSystem4 = 831,
    CamshaftController = 900,
    SystemService = 10000,
    Scope = 14000
}


/// ADS Device Notification
/// Request: `The data which are transfered at the Device Notification are multiple nested into one another.
/// The Notification Stream contains an array with elements of type AdsStampHeader.
/// This array again contains elements of type AdsNotificationSample.`
#[derive(Debug, PartialEq)]
pub struct AdsNotificationStream {
    length: u32,
    stamps: u32,
    // number of AdsStampHeaders in the ads_stamp_header field
    ads_stamp_header: AdsStampHeader
}

#[derive(Debug, PartialEq)]
pub struct AdsStampHeader {
    timestamp: u64,
    samples: u32,
    // number of AdsNotificationSamples in the ads_notification_filed
    ads_notification_filed: AdsNotificationSample
}

#[derive(Debug, PartialEq)]
pub struct AdsNotificationSample {
    notification_handle: u32,
    sample_size: u32,
    data: [u8]
}

#[derive(Debug, PartialEq, Clone)]
pub struct AdsVersion {
    version: u8,
    revision: u8,
    build: u16
}

#[cfg(test)]
mod tests {
    use core::ads::*;

    #[test]
    fn parse_ams_net_id() {
        let mut id1 = AmsNetId::new(127, 0, 0, 1, 1, 1);
        let id2 = AmsNetId::parse("127.0.0.1.1.1");
        assert_eq!(Ok(id1), id2);
        id1 = AmsNetId::from([127, 0, 0, 1, 1, 1]);
        assert_eq!(Ok(id1), id2);
    }

    #[test]
    fn into_ams_net_id() {
        let id1 = "127.0.0.1.1.1".to_ams_id();
        let id2 = "127.0.0.1.1.1".to_string().to_ams_id();
        assert_eq!(id1, id2);
    }
}