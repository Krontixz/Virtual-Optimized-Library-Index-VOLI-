use std::fs::File;
use std::io::{Read, Write, Result};
use std::path::Path;

pub struct VolicConfig {
    pub auto_verify: bool,
    pub optimization_level: u8,
    pub default_handler_id: u8,
}

impl VolicConfig {
    pub fn default() -> Self {
        Self {
            auto_verify: true,
            optimization_level: 2,
            default_handler_id: 0,
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;
        let data = [
            self.auto_verify as u8,
            self.optimization_level,
            self.default_handler_id,
        ];
        file.write_all(&data)?;
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 3];
        file.read_exact(&mut buffer)?;

        Ok(Self {
            auto_verify: buffer != 0,
            optimization_level: buffer,
            default_handler_id: buffer,
        })
    }
}
