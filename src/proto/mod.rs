use crate::proto::request::WriteTo;
use crate::proto::response::ReadFrom;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Error, Read, Write};

pub mod request;
pub mod response;

#[derive(Debug)]
pub struct DataMessage {
    result: u32,
    length: u32,
    data: Vec<u8>,
}

impl WriteTo for DataMessage {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.result)?;
        wtr.write_u32::<LittleEndian>(self.length)?;
        wtr.write(self.data.as_slice())?;
        Ok(())
    }
}

impl ReadFrom for DataMessage {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let result = read.read_u32::<LittleEndian>()?;
        let length = read.read_u32::<LittleEndian>()?;
        let mut data = Vec::with_capacity(length as usize);
        read.read_exact(data.as_mut_slice())?;
        Ok(Self {
            result,
            length,
            data,
        })
    }
}
