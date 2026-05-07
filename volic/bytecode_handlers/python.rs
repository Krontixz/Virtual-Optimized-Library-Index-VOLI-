pub struct PythonHandler {
    pub magic: [u8; 4],
    pub timestamp: [u8; 4],
    pub size: [u8; 4],
}

impl PythonHandler {
    pub fn new() -> Self {
        Self {
            magic: [0x00; 4],
            timestamp: [0x00; 4],
            size: [0x00; 4],
        }
    }

    pub fn is_pyc(&self, data: &[u8]) -> bool {
        data.len() >= 16
    }
}
