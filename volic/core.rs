use std::io::{Read, Write, Result};
use crate::volic::types::VolicType;

pub struct VolicHeader {
    pub magic: [u8; 5],
    pub class_type: VolicType,
    pub version: u32,
    pub payload_size: u64,
}

pub struct VolicClass {
    pub header: VolicHeader,
    pub payload: Vec<u8>,
}

impl VolicClass {
    pub fn new(class_type: VolicType, version: u32, payload: Vec<u8>) -> Self {
        Self {
            header: VolicHeader {
                magic: *b"VOLIC",
                class_type,
                version,
                payload_size: payload.len() as u64,
            },
            payload,
        }
    }

    pub fn save<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write_all(&self.header.magic)?;
        writer.write_all(&[self.header.class_type.to_byte()])?;
        writer.write_all(&self.header.version.to_be_bytes())?;
        writer.write_all(&self.header.payload_size.to_be_bytes())?;
        writer.write_all(&self.payload)?;
        Ok(())
    }

    pub fn load<R: Read>(mut reader: R) -> Result<Self> {
        let mut magic = [0u8; 5];
        reader.read_exact(&mut magic)?;

        let mut type_byte = [0u8; 1];
        reader.read_exact(&mut type_byte)?;

        let mut version_bytes = [0u8; 4];
        reader.read_exact(&mut version_bytes)?;

        let mut size_bytes = [0u8; 8];
        reader.read_exact(&mut size_bytes)?;

        let payload_size = u64::from_be_bytes(size_bytes);
        let mut payload = vec![0u8; payload_size as usize];
        reader.read_exact(&mut payload)?;

        Ok(Self {
            header: VolicHeader {
                magic,
                class_type: VolicType::from_byte(type_byte),
                version: u32::from_be_bytes(version_bytes),
                payload_size,
            },
            payload,
        })
    }
}
