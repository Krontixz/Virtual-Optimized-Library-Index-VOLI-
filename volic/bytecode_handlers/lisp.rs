pub struct LispHandler {
    pub header: Vec<u8>,
}

impl LispHandler {
    pub fn new() -> Self {
        Self {
            header: Vec::new(),
        }
    }
}
