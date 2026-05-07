pub struct FlashHandler {
    pub signature: [u8; 3],
}

impl FlashHandler {
    pub fn new() -> Self {
        Self {
            signature: [0x46, 0x57, 0x53],
        }
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        if data.len() < 3 {
            return false;
        }
        data[0..3] == self.signature || data[0..3] == [0x43, 0x57, 0x53] || data[0..3] == [0x5A, 0x57, 0x53]
    }
}
