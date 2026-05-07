pub struct JvmHandler {
    pub magic: [u8; 4],
    pub minor_version: u16,
    pub major_version: u16,
}

impl JvmHandler {
    pub fn new() -> Self {
        Self {
            magic: [0xCA, 0xFE, 0xBA, 0xBE],
            minor_version: 0,
            major_version: 0,
        }
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }
        data[0..4] == self.magic
    }
}
