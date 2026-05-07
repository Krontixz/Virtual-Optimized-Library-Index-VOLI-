pub struct LuaHandler {
    pub magic: [u8; 4],
}

impl LuaHandler {
    pub fn new() -> Self {
        Self {
            magic: [0x1B, 0x4C, 0x75, 0x61],
        }
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }
        data[0..4] == self.magic
    }
}
