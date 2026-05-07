pub struct BeamHandler {
    pub format_id: [u8; 4],
}

impl BeamHandler {
    pub fn new() -> Self {
        Self {
            format_id: *b"FOR1",
        }
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        if data.len() < 12 {
            return false;
        }
        &data[0..4] == b"FOR1" && &data[8..12] == b"BEAM"
    }
}
