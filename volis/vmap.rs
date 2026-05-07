use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Result};
use crate::volis::stack::VolisStack;

pub struct VolisMapper;

impl VolisMapper {
    pub fn jump_to_class(stack: &VolisStack, name: &str, mut file: &File) -> Result<Vec<u8>> {
        for entry in &stack.entries {
            if entry.name == name {
                file.seek(SeekFrom::Start(entry.offset))?;
                let mut buffer = vec![0u8; entry.size as usize];
                file.read_exact(&mut buffer)?;
                return Ok(buffer);
            }
        }
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Class not found in stack"))
    }
}
