use std::io::{Read, Write, Result, Seek, SeekFrom};
use crate::volic::VolicClass;

pub struct VolisEntry {
    pub name: String,
    pub offset: u64,
    pub size: u64,
}

pub struct VolisStack {
    pub magic: [u8; 5],
    pub version: u32,
    pub entries: Vec<VolisEntry>,
}

impl VolisStack {
    pub fn new() -> Self {
        Self {
            magic: *b"VOLIS",
            version: 1,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, name: String, offset: u64, size: u64) {
        self.entries.push(VolisEntry { name, offset, size });
    }
}
