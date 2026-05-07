pub struct ParrotHandler {
    pub magic: [u8; 8],
}

impl ParrotHandler {
    pub fn new() -> Self {
        Self {
            magic: [0xFE, 0x31, 0x4C, 0x31, 0x0A, 0x0D, 0x0A, 0x00],
        }
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        if data.len() < 8 {
            return false;
        }
        data[0..8] == self.magic
    }
}
