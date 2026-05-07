pub struct DotNetHandler {
    pub signature: [u8; 2],
}

impl DotNetHandler {
    pub fn new() -> Self {
        Self {
            signature: [0x4D, 0x5A],
        }
    }

    pub fn check_cil(&self, data: &[u8]) -> bool {
        if data.len() < 2 {
            return false;
        }
        data[0..2] == self.signature
    }
}
