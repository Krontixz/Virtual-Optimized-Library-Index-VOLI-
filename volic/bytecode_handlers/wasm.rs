pub struct WasmHandler {
    pub magic: [u8; 4],
    pub version: [u8; 4],
}

impl WasmHandler {
    pub fn new() -> Self {
        Self {
            magic: [0x00, 0x61, 0x73, 0x6D],
            version: [0x01, 0x00, 0x00, 0x00],
        }
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        if data.len() < 8 {
            return false;
        }
        data[0..4] == self.magic && data[4..8] == self.version
    }
}
