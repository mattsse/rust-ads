use crate::proto::DataMessage;
use byteorder::{LittleEndian, WriteBytesExt};
use bytes::BufMut;
use std::io::{self, Error, Write};

#[derive(Debug)]
pub enum Request {
    Read(ReadRequest),
    Write(DataMessage),
    ReadWrite(ReadWriteRequest),
    DeleteDeviceNotification(u32),
    AddDeviceNotification(AddDeviceNotificationRequest),
    WriteControl(WriteControlRequest),
}

pub trait WriteTo {
    fn write_to<W: Write>(&self, wtr: W) -> io::Result<()>;
}

pub trait SendRecieve {
    // TODO add router as param that implements read to write
    fn send_receive(&self) -> io::Result<()>;
}

/// ADS Read Write
#[derive(Debug, PartialEq)]
pub struct ReadWriteRequest {
    index_group: u32,
    index_offset: u32,
    read_length: u32,
    write_length: u32,
    data: Vec<u8>,
}

impl WriteTo for ReadWriteRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.index_group)?;
        wtr.write_u32::<LittleEndian>(self.index_offset)?;
        wtr.write_u32::<LittleEndian>(self.read_length)?;
        wtr.write_u32::<LittleEndian>(self.write_length)?;
        wtr.write_all(self.data.as_slice())
    }
}

/// ADS Add Device Notification
#[derive(Debug, PartialEq, Clone)]
pub struct AddDeviceNotificationRequest {
    index_group: u32,
    index_offset: u32,
    length: u32,
    transmission_mode: u32,
    max_delay: u32,
    cycle_time: u32,
    reserved: [u8; 16],
}

impl WriteTo for AddDeviceNotificationRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.index_group)?;
        wtr.write_u32::<LittleEndian>(self.index_offset)?;
        wtr.write_u32::<LittleEndian>(self.length)?;
        wtr.write_u32::<LittleEndian>(self.transmission_mode)?;
        wtr.write_u32::<LittleEndian>(self.max_delay)?;
        wtr.write_u32::<LittleEndian>(self.cycle_time)?;
        wtr.write_all(&self.reserved)
    }
}

/// ADS Write Control
#[derive(Debug, PartialEq)]
pub struct WriteControlRequest {
    ads_state: u16,
    device_state: u16,
    data: Vec<u8>,
}

impl WriteTo for WriteControlRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u16::<LittleEndian>(self.ads_state)?;
        wtr.write_u16::<LittleEndian>(self.device_state)?;
        wtr.write_u32::<LittleEndian>(self.data.len() as u32)?;
        wtr.write(self.data.as_slice())?;
        Ok(())
    }
}

/// ADS Read
#[derive(Debug, PartialEq)]
pub struct ReadRequest {
    index_group: u32,
    index_offset: u32,
    length: u32,
}

impl WriteTo for ReadRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.index_group)?;
        wtr.write_u32::<LittleEndian>(self.index_offset)?;
        wtr.write_u32::<LittleEndian>(self.length)
    }
}

impl WriteTo for Request {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        match self {
            Request::Read(r) => r.write_to(wtr),
            Request::Write(r) => r.write_to(wtr),
            Request::ReadWrite(r) => r.write_to(wtr),
            Request::AddDeviceNotification(r) => r.write_to(wtr),
            Request::WriteControl(r) => r.write_to(wtr),
            Request::DeleteDeviceNotification(r) => wtr.write_u32::<LittleEndian>(*r),
        }
    }
}
